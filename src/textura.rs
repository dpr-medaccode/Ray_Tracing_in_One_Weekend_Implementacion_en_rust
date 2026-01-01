pub mod color_solido;
pub mod ajedrez;
pub mod textura_imagen;
pub mod perlin;

use crate::{color::Color, vec3::Point3};

pub trait Textura: Send + Sync {

    fn valor(&self, textura_horizontal: f64, textura_vertical: f64, lugar: &Point3) -> Color;
    
}