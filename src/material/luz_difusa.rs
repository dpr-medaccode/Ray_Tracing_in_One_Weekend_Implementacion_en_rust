#![allow(dead_code)]

use std::sync::Arc;

use crate::{
    color::Color,
    material::Material,
    textura::{Textura, color_solido::ColorSolido},
};

pub struct LuzDifusa {
    textura: Arc<dyn Textura>,
}

impl LuzDifusa {
    pub fn new_from_color(color: Color) -> Self {
        Self {
            textura: Arc::new(ColorSolido::new(color)),
        }
    }

    pub fn new_from_textura(textura: Arc<dyn Textura>) -> Self {
        LuzDifusa { textura }
    }
}

impl Material for LuzDifusa {
    fn luz_emitida(
        &self,
        textura_horizontal: f64,
        textura_vertical: f64,
        lugar: &crate::vec3::Point3,
    ) -> Color {
        self.textura
            .valor(textura_horizontal, textura_vertical, lugar)
    }
}
