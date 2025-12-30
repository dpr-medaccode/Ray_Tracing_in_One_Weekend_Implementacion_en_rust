use std::sync::Arc;

use crate::{
    color::Color,
    textura::{Textura, color_solido::Color_solido},
};

pub struct Ajedrez {
    escala_cuadro: f64,

    textura_on: Arc<dyn Textura>,
    textura_off: Arc<dyn Textura>,
}

impl Ajedrez {
    pub fn new(escala: f64, on: Arc<dyn Textura>, off: Arc<dyn Textura>) -> Self {
        Self {
            escala_cuadro: 1.0 / escala,
            textura_on: on,
            textura_off: off,
        }
    }

   pub fn new_from_colores(escala: f64, on: Color, off: Color) -> Self {
        Self {
            escala_cuadro: 1.0 / escala,
            textura_on: Arc::new(Color_solido::new(on)),
            textura_off: Arc::new(Color_solido::new(off)),
        }
    }
}

impl Textura for Ajedrez {
    fn valor(
        &self,
        textura_horizontal: f64,
        textura_vertical: f64,
        lugar: &crate::vec3::Point3,
    ) -> crate::color::Color {
        let x = (self.escala_cuadro * lugar.x()).floor() as i32;
        let y = (self.escala_cuadro * lugar.y()).floor() as i32;
        let z = (self.escala_cuadro * lugar.z()).floor() as i32;

        let es_par = (x + y + z) % 2 == 0;

        if es_par {
            self.textura_on
                .valor(textura_horizontal, textura_vertical, lugar)
        } else {
            self.textura_off
                .valor(textura_horizontal, textura_vertical, lugar)
        }
    }
}
