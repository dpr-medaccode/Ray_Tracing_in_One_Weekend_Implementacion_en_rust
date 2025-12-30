use crate::{color::Color, textura::Textura};

pub struct Color_solido {
    color: Color,
}

impl Color_solido {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Textura for Color_solido {
    fn valor(
        &self,
        _textura_horizontal: f64,
        _textura_vertical: f64,
        _lugar: &crate::vec3::Point3,
    ) -> Color {
        self.color
    }
}
