pub mod dielectrico;
pub mod difuso_lambertiano;
pub mod metal;

use crate::{color::Color, golpe::Golpe, rayo::Rayo};

pub trait Material: Send + Sync {
    fn dispersion(&self, rayo_entrante: &Rayo, golpe: &Golpe) -> Option<(Rayo, Color)>;
}
