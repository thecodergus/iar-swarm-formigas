use super::mapa::Mapa;
use super::formiga::Formiga;


use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct Cenario{
    mapa: Mapa,
    formigas: Vec<Formiga>,
    gl: GlGraphics,
    window: Window
}

