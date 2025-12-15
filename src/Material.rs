pub mod Dielectrico;
pub mod Difuso_lambertiano;
pub mod Metal;

use crate::{Color::Color, Golpe::Golpe, Rayo::Rayo};

pub trait Material: Send + Sync {
    fn dispersion(&self, rayo_entrante: &Rayo, golpe: &Golpe) -> Option<(Rayo, Color)>;
}
