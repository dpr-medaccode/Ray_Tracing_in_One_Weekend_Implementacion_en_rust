pub mod Difuso_lambertiano;
pub mod Metal;
pub mod Dielectrico;

use crate::{Color::Color, Golpe::Golpe, Rayo::Rayo};

pub trait Material {
    fn dispersion(&self, rayo_entrante: &Rayo, golpe: &Golpe) -> Option<(Rayo, Color)>;
}
