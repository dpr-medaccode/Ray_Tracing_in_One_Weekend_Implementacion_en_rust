pub mod dielectrico;
pub mod difuso_lambertiano;
pub mod luz_difusa;
pub mod metal;

use crate::{
    color::{Color, NEGRO},
    golpe::Golpe,
    rayo::Rayo,
    vec3::Point3,
};

pub trait Material: Send + Sync {
    fn dispersion(&self, _rayo_entrante: &Rayo, _golpe: &Golpe) -> Option<(Rayo, Color)> {
        None
    }

    fn luz_emitida(
        &self,
        _textura_horizontal: f64,
        _textura_vertical: f64,
        _lugar: &Point3,
    ) -> Color {
        NEGRO
    }
}
