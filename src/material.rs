pub mod dielectrico;
pub mod difuso_lambertiano;
pub mod metal;
pub mod luz_difusa;

use crate::{
    color::{Color, NEGRO},
    golpe::Golpe,
    rayo::Rayo, vec3::Point3,
};

pub trait Material: Send + Sync {
    fn dispersion(&self, rayo_entrante: &Rayo, golpe: &Golpe) -> Option<(Rayo, Color)>{
        None
    }

    fn luz_emitida(&self, textura_horizontal: f64, textura_vertical: f64, lugar: &Point3) -> Color {
        NEGRO
    }
}
