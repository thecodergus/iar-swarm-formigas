use super::grao::{self, Grao};
use super::outros::Ponto;
use rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex};
use std::{thread, vec};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Formiga {
    pub id: Uuid,
    pub posicao: Arc<Mutex<Ponto>>,
    pub segurando_objeto: Arc<Mutex<Option<Grao>>>,
    matar_thread: Arc<Mutex<bool>>,
}

// Parametros
const TAMANHO_VIZINHANCA: f64 = 1.0; // Definindo o tamanho da vizinhança (a distância máxima em cada direção)
const ALPHA: f64 = 30.0;
const K1: f64 = 0.01;
const K2: f64 = 0.015;

impl Formiga {
    pub fn new(ponto_surgimento: Ponto) -> Formiga {
        Formiga {
            id: Uuid::new_v4(),
            posicao: Arc::new(Mutex::new(ponto_surgimento)),
            segurando_objeto: Arc::new(Mutex::new(None)),
            matar_thread: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(
        &mut self,
        tamanho_mapa: (f64, f64),
        graos: Arc<Mutex<Vec<Grao>>>,
        contador: Arc<Mutex<i64>>,
    ) {
        let posicao: Arc<Mutex<Ponto>> = Arc::clone(&self.posicao);
        let segurando_objeto: Arc<Mutex<Option<Grao>>> = Arc::clone(&self.segurando_objeto);
        let matar_thread: Arc<Mutex<bool>> = Arc::clone(&self.matar_thread);

        let _: thread::JoinHandle<()> = thread::spawn(move || {
            loop {
                // Verificação de matar_thread antes de continuar o loop
                if let Ok(matar) = matar_thread.lock() {
                    if *matar {
                        return; // Encerra o loop e a thread
                    } else {
                        if let Ok(mut contador_guard) = contador.lock() {
                            if *contador_guard <= 0 {
                                return;
                            } else {
                                *contador_guard -= 1;
                            }
                        }
                    }
                } else {
                    eprintln!("Erro ao bloquear mutex: matar_thread");
                    std::process::exit(1);
                }

                // Movendo a formiga
                let nova_posicao = nova_posicao(Arc::clone(&posicao), tamanho_mapa);
                if let Ok(mut posicao_guard) = posicao.lock() {
                    *posicao_guard = nova_posicao;
                }

                // Ações relacionadas a ter ou não itens na mão
                acao_segurar_objeto(
                    Arc::clone(&posicao),
                    Arc::clone(&segurando_objeto),
                    Arc::clone(&graos),
                );
            }
        });
    }

    pub fn stop(&mut self) {
        if let Ok(mut matar_guard) = self.matar_thread.lock() {
            *matar_guard = true;
        } else {
            eprintln!("Erro ao tentar bloquear o mutex: matar_thread");
        }
    }
}

fn nova_posicao(posicao: Arc<Mutex<Ponto>>, tamanho_mapa: (f64, f64)) -> Ponto {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    // Gera um número aleatório de 1 a 8 para escolher a direção
    let num_aleatorio: i32 = rng.gen_range(1..=8);

    if let Ok(posicao_guard) = posicao.lock() {
        let mut nova_posicao = match num_aleatorio {
            1 => Ponto {
                // Cima
                x: posicao_guard.x,
                y: posicao_guard.y + 1 as i32,
            },
            2 => Ponto {
                // Direita
                x: posicao_guard.x + 1 as i32,
                y: posicao_guard.y,
            },
            3 => Ponto {
                // Baixo
                x: posicao_guard.x,
                y: posicao_guard.y - 1 as i32,
            },
            4 => Ponto {
                // Esquerda
                x: posicao_guard.x - 1 as i32,
                y: posicao_guard.y,
            },
            5 => Ponto {
                // Diagonal superior direita
                x: posicao_guard.x + 1 as i32,
                y: posicao_guard.y + 1 as i32,
            },
            6 => Ponto {
                // Diagonal superior esquerda
                x: posicao_guard.x - 1 as i32,
                y: posicao_guard.y + 1 as i32,
            },
            7 => Ponto {
                // Diagonal inferior direita
                x: posicao_guard.x + 1 as i32,
                y: posicao_guard.y - 1 as i32,
            },
            8 => Ponto {
                // Diagonal inferior esquerda
                x: posicao_guard.x - 1 as i32,
                y: posicao_guard.y - 1 as i32,
            },
            _ => Ponto {
                // Mantém a posição atual, caso um número inesperado seja gerado
                x: posicao_guard.x,
                y: posicao_guard.y,
            },
        };

        // Verificação dos limites do mapa com teletransporte
        // Se ultrapassar o limite à esquerda, vai para o limite direito
        if nova_posicao.x < 0 {
            nova_posicao.x = tamanho_mapa.0 as i32;
        // Se ultrapassar o limite direito, vai para o limite esquerdo
        } else if nova_posicao.x > tamanho_mapa.0 as i32 {
            nova_posicao.x = 0;
        }

        // Se ultrapassar o limite superior, vai para o limite inferior
        if nova_posicao.y < 0 {
            nova_posicao.y = tamanho_mapa.1 as i32;
        // Se ultrapassar o limite inferior, vai para o limite superior
        } else if nova_posicao.y > tamanho_mapa.1 as i32 {
            nova_posicao.y = 0;
        }

        nova_posicao
    } else {
        eprintln!("Erro ao bloquear mutex: segurando_objeto");
        std::process::exit(1);
    }
}

pub fn gerar_formigas(numero: i32, tamanho_mapa: (f64, f64)) -> Vec<Formiga> {
    // Criar 10 formigas aleatórias
    let mut rng = rand::thread_rng();
    let mut formigas: Vec<Formiga> = vec![];

    for _ in 0..numero {
        let x: i32 = rng.gen_range(0..=tamanho_mapa.0 as i32);
        let y: i32 = rng.gen_range(0..=tamanho_mapa.1 as i32);
        formigas.push(Formiga::new(Ponto { x: x, y: y }));
    }

    formigas
}

fn encontrar_grao_mais_proximo_vizinhanca(
    posicao: &Ponto,
    graos_guardados: &[Grao], // Usar slice é mais idiomático do que Vec<&Grao> para leitura
    na_mao: &Option<Grao>,
) -> (Option<Grao>, Vec<Grao>) {
    // Vetor para armazenar os grãos dentro da vizinhança
    let mut graos_na_vizinhanca: Vec<Grao> = Vec::new();
    let mut grao_no_local = na_mao.clone(); // Clona o grão se estiver "na mão"

    // Se houver um grão "na mão", adiciona-o à vizinhança
    if let Some(grao) = na_mao {
        graos_na_vizinhanca.push(grao.clone());
    }

    // Itera sobre os grãos guardados
    for grao in graos_guardados.iter() {
        let distancia_x = (grao.posicao.x as f64 - posicao.x as f64).abs();
        let distancia_y = (grao.posicao.y as f64 - posicao.y as f64).abs();

        // Verifica se o grão está dentro da vizinhança, sem considerar a mesma posição
        if distancia_x <= TAMANHO_VIZINHANCA && distancia_y <= TAMANHO_VIZINHANCA {
            graos_na_vizinhanca.push(grao.clone());

            // Se o grão está exatamente na mesma posição e não há grão "no local"
            if grao.posicao == *posicao && grao_no_local.is_none() {
                grao_no_local = Some(grao.clone());
            }
        }
    }

    (grao_no_local, graos_na_vizinhanca)
}

fn acao_segurar_objeto(
    posicao_formiga: Arc<Mutex<Ponto>>,
    objeto: Arc<Mutex<Option<Grao>>>,
    graos: Arc<Mutex<Vec<Grao>>>,
) {
    // Gera a probabilidade aleatória
    let mut rng = rand::thread_rng();
    let probabilidade = rng.gen_range(0.0..=1.0);

    // Adquire os locks de uma vez, tratando possíveis falhas de bloqueio
    let mut objeto_guard = objeto.lock().expect("Erro ao adquirir lock do objeto.");
    let mut graos_guard = graos.lock().expect("Erro ao adquirir lock dos grãos.");
    let posicao_guard = posicao_formiga
        .lock()
        .expect("Erro ao adquirir lock da posição da formiga.");

    // Encontra o grão na posição e os grãos ao redor
    let (grao_na_posicao, graos_entorno) =
        encontrar_grao_mais_proximo_vizinhanca(&posicao_guard, &graos_guard, &objeto_guard);

    // Se a formiga estiver carregando um grão
    if let Some(grao_carregado) = &mut *objeto_guard {
        // Verifica se a formiga pode largar o grão na posição atual
        if !ha_grao_na_posicao_formiga(&posicao_guard, &graos_guard)
            && probabilidade <= pode_largar(grao_carregado, &graos_entorno)
        {
            // Atualiza a posição do grão e o adiciona de volta à lista de grãos
            grao_carregado.posicao = posicao_guard.clone();
            adicionar_grao(grao_carregado, &mut graos_guard);
            *objeto_guard = None;
        }
    }
    // Se a formiga não estiver carregando nada
    else if let Some(grao) = grao_na_posicao {
        if probabilidade <= pode_pegar(&grao, &graos_entorno) {
            objeto_guard.replace(grao.clone());
            remover_grao(&grao, &mut graos_guard);
        }
    }
}

fn distancia_euclidiana_adaptada(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    // Primeiro, assegura que os dois vetores têm o mesmo tamanho
    // Essa verificação evita erros ao calcular a distância entre vetores de tamanhos diferentes.
    assert_eq!(a.len(), b.len(), "Os vetores devem ter o mesmo tamanho.");

    return a
        .iter() // Itera sobre os elementos do vetor 'a'
        .zip(b.iter()) // Combina com os elementos correspondentes do vetor 'b'
        .map(|(a_i, b_i)| (a_i - b_i).powi(2)) // Calcula o quadrado da diferença entre cada par de elementos
        .sum::<f64>() // Soma todas essas diferenças quadráticas
        .sqrt(); // Calcula a raiz quadrada da soma, obtendo a distância euclidiana
}

fn similaridade(grao: &Grao, graos_perto: &Vec<Grao>) -> f64 {
    // Se não há grãos por perto, a similaridade é zero
    if graos_perto.len() == 0 {
        return 0.0;
    } else {
        return (1.0 / (graos_perto.len() as f64).powi(2))
            * graos_perto
                .iter()
                .map(|grao_aux| {
                    1.0 - (distancia_euclidiana_adaptada(&grao.dados, &grao_aux.dados) / ALPHA)
                })
                .sum::<f64>();
    }
}

fn pode_pegar(grao: &Grao, graos_perto: &Vec<Grao>) -> f64 {
    (K1 / (K1 + similaridade(grao, graos_perto))).powi(2)
}

fn pode_largar(grao: &Grao, graos_perto: &Vec<Grao>) -> f64 {
    // Primeiro, calcula-se a similaridade com os grãos próximos
    let similidarida_result: f64 = similaridade(grao, graos_perto);

    // A probabilidade de largar o item aumenta à medida que a similaridade com os grãos próximos aumenta
    // A constante K2 ajusta a sensibilidade dessa probabilidade
    (similidarida_result / (K2 + similidarida_result)).powi(2)
}

fn remover_grao(g: &Grao, graos: &mut Vec<Grao>) {
    graos.retain(|g_| g.id != g_.id);
}

fn adicionar_grao(g: &Grao, graos: &mut Vec<Grao>) {
    graos.push(g.clone());
}

fn ha_grao_na_posicao_formiga(posicao_formiga: &Ponto, graos_guard: &Vec<Grao>) -> bool {
    for grao in graos_guard.iter() {
        if grao.posicao == *posicao_formiga {
            return true; // Retorna true se encontrar um grão na mesma posição
        }
    }

    false // Retorna false se nenhum grão for encontrado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distancia_vetores_iguais() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        let result = distancia_euclidiana_adaptada(&a, &b);
        assert_eq!(
            result, 0.0,
            "A distância entre vetores idênticos deve ser 0."
        );
    }

    #[test]
    fn test_distancia_vetores_simples() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let result = distancia_euclidiana_adaptada(&a, &b);
        let expected =
            ((1.0 - 4.0 as f64).powi(2) + (2.0 - 5.0 as f64).powi(2) + (3.0 - 6.0 as f64).powi(2))
                .sqrt();
        assert_eq!(
            result, expected,
            "A distância deve ser calculada corretamente."
        );
    }

    #[test]
    fn test_distancia_vetores_com_pontos_flutuantes() {
        let a: Vec<f64> = vec![1.1, 2.2, 3.3];
        let b: Vec<f64> = vec![4.4, 5.5, 6.6];
        let result: f64 = distancia_euclidiana_adaptada(&a, &b);
        let expected =
            ((1.1 - 4.4 as f64).powi(2) + (2.2 - 5.5 as f64).powi(2) + (3.3 - 6.6 as f64).powi(2))
                .sqrt();
        assert!(
            (result - expected).abs() < f64::EPSILON,
            "A distância deve ser precisa para valores em ponto flutuante."
        );
    }

    #[test]
    fn test_distancia_vetores_com_zeros() {
        let a = vec![0.0, 0.0, 0.0];
        let b = vec![0.0, 0.0, 0.0];
        let result = distancia_euclidiana_adaptada(&a, &b);
        assert_eq!(
            result, 0.0,
            "A distância entre dois vetores de zeros deve ser 0."
        );
    }

    #[test]
    fn test_distancia_vetores_negativos() {
        let a = vec![-1.0, -2.0, -3.0];
        let b = vec![1.0, 2.0, 3.0];
        let result = distancia_euclidiana_adaptada(&a, &b);
        let expected: f64 = ((-1.0 - 1.0 as f64).powi(2)
            + (-2.0 - 2.0 as f64).powi(2)
            + (-3.0 - 3.0 as f64).powi(2))
        .sqrt();
        assert_eq!(
            result, expected,
            "A distância deve ser calculada corretamente para valores negativos."
        );
    }

    #[test]
    #[should_panic(expected = "Os vetores devem ter o mesmo tamanho.")]
    fn test_distancia_vetores_tamanho_diferente() {
        let a = vec![1.0, 2.0];
        let b = vec![1.0, 2.0, 3.0];
        distancia_euclidiana_adaptada(&a, &b); // Deve entrar em panic porque os tamanhos são diferentes.
    }
}
