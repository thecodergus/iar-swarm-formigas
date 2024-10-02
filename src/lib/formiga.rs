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
const ALPHA: f64 = 0.35;
const K1: f64 = 0.5;
const K2: f64 = 0.025;

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
    graos_guard: &Vec<Grao>,
) -> (Option<Grao>, Vec<Grao>) {
    let mut graos_na_vizinhanca: Vec<Grao> = vec![];
    let mut grao_mais_proximo: Option<Grao> = None;
    let mut distancia_minima: f64 = f64::MAX;

    // Trava o mutex para acessar a posição da formiga
    // Trava o mutex para acessar a lista de grãos
    for grao in graos_guard.iter() {
        let distancia_x: f64 = (grao.posicao.x as f64 - posicao.x as f64).abs();
        let distancia_y: f64 = (grao.posicao.y as f64 - posicao.y as f64).abs();
        let distancia_total: f64 = (distancia_x.powi(2) + distancia_y.powi(2)).sqrt();

        // Verifica se o grão está dentro da vizinhança e não está na mesma posição exata
        if distancia_x <= TAMANHO_VIZINHANCA && distancia_y <= TAMANHO_VIZINHANCA {
            // Verifica se este grão é o mais próximo
            if distancia_total < distancia_minima {
                // Se for o mais próximo até agora, atualiza o mais próximo
                if let Some(grao_atual) = grao_mais_proximo.take() {
                    // Adiciona o antigo grão mais próximo ao vetor de grãos
                    graos_na_vizinhanca.push(grao_atual);
                }
                grao_mais_proximo = Some(grao.clone());
                distancia_minima = distancia_total;
            }

            graos_na_vizinhanca.push(grao.clone());
        }
    }

    (grao_mais_proximo, graos_na_vizinhanca)
}

fn acao_segurar_objeto(
    posicao_formiga: Arc<Mutex<Ponto>>,
    objeto: Arc<Mutex<Option<Grao>>>,
    graos: Arc<Mutex<Vec<Grao>>>,
) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let probabilidade: f64 = rng.gen_range(0.0..=1.0);

    // Tenta adquirir o lock no objeto e trabalhar com ele
    if let Ok(mut objeto_guard) = objeto.lock() {
        if let Ok(mut graos_guard) = graos.lock() {
            if let Ok(posicao_guard) = posicao_formiga.lock() {
                // Chamando a função e desestruturando o retorno em duas variáveis
                let (grao_mais_proximo, graos_restantes) =
                    encontrar_grao_mais_proximo_vizinhanca(&posicao_guard, &graos_guard);

                // Se a formiga já estiver carregando algum grão
                if let Some(grao_carregado) = &mut *objeto_guard {
                    if !ha_grao_na_posicao_formiga(&posicao_guard, &graos_guard) {
                        // Largar (caso queira largar o objeto em uma posição vazia)
                        if probabilidade <= pode_largar(grao_carregado, &graos_restantes) {
                            // Adiciona o grão na lista de grãos novamente
                            grao_carregado.posicao = posicao_guard.clone();
                            adicionar_grao(grao_carregado, &mut graos_guard); // Passa referência ao grão
                            *objeto_guard = None; // Limpa a mão da formiga
                        }
                    }
                } else {
                    // Se a formiga não estiver carregando nada, tenta pegar um grão na posição
                    if let Some(grao) = &grao_mais_proximo {
                        // Probabilidade de pegar o grão
                        if probabilidade <= pode_pegar(grao, &graos_restantes) {
                            // Adicionar o grão à mão da formiga
                            objeto_guard.replace(grao.clone());

                            // Remover o grão do vetor de grãos
                            remover_grao(grao, &mut graos_guard);
                        }
                    }
                }
            }
        }
    }
}

// Função que calcula a distância euclidiana adaptada entre dois vetores
// No contexto de "Ant-based clustering", essa função é utilizada para medir a similaridade
// entre dois objetos (ou grãos). Em vez de comparar diretamente as coordenadas espaciais,
// a distância euclidiana adaptada compara os dados representados pelos vetores "a" e "b".
// Isso é importante, pois formigas decidem suas ações (pegar ou largar um grão) com base na
// similaridade entre os objetos ao seu redor, onde a distância menor indica maior similaridade.

fn distancia_euclidiana_adaptada(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    // Primeiro, assegura que os dois vetores têm o mesmo tamanho
    // Essa verificação evita erros ao calcular a distância entre vetores de tamanhos diferentes.
    assert_eq!(a.len(), b.len(), "Os vetores devem ter o mesmo tamanho.");

    // A seguir, itera sobre os dois vetores simultaneamente usando zip, subtraindo os elementos
    // correspondentes de 'a' e 'b', elevando essa diferença ao quadrado (como na fórmula
    // da distância euclidiana). Depois, soma todas as diferenças ao quadrado e finalmente tira
    // a raiz quadrada, que dá a distância euclidiana entre os dois vetores.
    // Isso mede a "proximidade" dos dois vetores nos seus espaços multidimensionais.
    return a
        .iter() // Itera sobre os elementos do vetor 'a'
        .zip(b.iter()) // Combina com os elementos correspondentes do vetor 'b'
        .map(|(a_i, b_i)| (a_i - b_i).powi(2)) // Calcula o quadrado da diferença entre cada par de elementos
        .sum::<f64>() // Soma todas essas diferenças quadráticas
        .sqrt(); // Calcula a raiz quadrada da soma, obtendo a distância euclidiana
}

// Função que calcula a similaridade entre um grão e os grãos próximos
// Esta função é inspirada no comportamento de formigas organizando itens (como larvas ou detritos)
// em suas colônias. A similaridade é um fator importante para que a formiga decida onde largar ou pegar
// um item, com base nos itens semelhantes ao redor.
// No contexto do "ant-based clustering", formigas agrupam itens semelhantes em áreas específicas.
fn similaridade(grao: &Grao, graos_perto: &Vec<Grao>) -> f64 {
    // Se não há grãos por perto, a similaridade é zero
    if graos_perto.len() == 0 {
        return 0.0;
    } else {
        // Calcula a similaridade entre o grão atual e os grãos ao redor
        // A fórmula usada aqui é baseada na média das similaridades individuais
        // A função `distancia_euclidiana_adaptada` calcula a distância entre os dados dos grãos
        // A constante ALPHA é usada para ajustar a escala da similaridade, com base na distância
        return (1.0 / (graos_perto.len() as f64).powi(2))
            * graos_perto
                .iter()
                .map(|grao_aux| {
                    // A similaridade é baseada na distância euclidiana adaptada entre os dados
                    // quanto menor a distância, maior a similaridade
                    1.0 - (distancia_euclidiana_adaptada(&grao.dados, &grao_aux.dados)) / ALPHA
                })
                .sum::<f64>();
    }
}

// Função que calcula a probabilidade de pegar um grão
// Formigas decidem pegar itens quando estes estão isolados ou em regiões onde a similaridade
// com os itens ao redor é baixa. Isso simula o comportamento natural das formigas quando encontram
// itens fora do lugar e decidem movê-los.
fn pode_pegar(grao: &Grao, graos_perto: &Vec<Grao>) -> f64 {
    // A probabilidade de pegar um item é inversamente proporcional à similaridade com os itens ao redor
    // Se o grão for muito diferente dos seus vizinhos, a similaridade será baixa e a probabilidade de
    // pegá-lo será maior. A constante K1 ajusta a sensibilidade desta probabilidade.
    (K1 / (K1 + similaridade(grao, graos_perto))).powi(2)
}

// Função que calcula a probabilidade de largar um grão
// Depois que a formiga carrega um item, ela precisa decidir onde largá-lo. Formigas largam itens
// em regiões onde outros itens semelhantes já estão presentes. Isso favorece a formação de grupos
// de itens semelhantes, comportamento comum em colônias de formigas.
// Similaridade alta significa uma probabilidade maior de largar o item naquele local.
fn pode_largar(grao: &Grao, graos_perto: &Vec<Grao>) -> f64 {
    // Primeiro, calcula-se a similaridade com os grãos próximos
    let similidarida_result: f64 = similaridade(grao, graos_perto);

    // A probabilidade de largar o item aumenta à medida que a similaridade com os grãos próximos aumenta
    // A constante K2 ajusta a sensibilidade dessa probabilidade
    (similidarida_result / (K2 + similidarida_result)).powi(2)
}

// Função que remove um grão da lista de grãos
// Quando a formiga pega um grão, esse grão é removido do ambiente. O vetor de grãos deve ser atualizado
// removendo o grão que foi retirado pela formiga. Esta função garante que a remoção ocorra de forma
// segura usando mecanismos de concorrência como Arc<Mutex> para garantir que outros processos não
// acessem os dados simultaneamente.
fn remover_grao(g: &Grao, graos: &mut Vec<Grao>) {
    // Filtra o vetor de grãos para manter apenas os grãos cujo ID seja diferente do grão removido
    graos.retain(|g_| g.id != g_.id);
}

// Função que adiciona um grão à lista de grãos
// Quando a formiga decide largar um grão, o grão deve ser inserido novamente no ambiente.
// Essa função adiciona o grão ao vetor de grãos de forma segura, garantindo que a inserção seja
// feita de maneira concorrente, evitando problemas de race conditions usando Arc<Mutex>.
fn adicionar_grao(g: &Grao, graos: &mut Vec<Grao>) {
    graos.push(g.clone());
}

fn ha_grao_na_posicao_formiga(posicao_formiga: &Ponto, graos_guard: &Vec<Grao>) -> bool {
    // Trava o mutex para acessar a posição da formiga
    // Trava o mutex para acessar a lista de grãos
    // Itera sobre os grãos e verifica se algum está na mesma posição que a formiga
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
