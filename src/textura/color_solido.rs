use crate::{color::Color, textura::Textura};

pub struct ColorSolido {
    color: Color,
}

impl ColorSolido {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Textura for ColorSolido {
    fn valor(
        &self,
        _textura_horizontal: f64,
        _textura_vertical: f64,
        _lugar: &crate::vec3::Point3,
    ) -> Color {
        self.color
    }
}
