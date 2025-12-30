use crate::{
    imagen::{Imagen, pixel_to_color},
    intervalo::Intervalo,
    textura::Textura,
};

pub struct TexturaImagen {
    imagen: Imagen,
}

impl TexturaImagen {
    pub fn new(ruta: &str) -> Self {
        Self {
            imagen: Imagen::new(ruta),
        }
    }
}

impl Textura for TexturaImagen {
    fn valor(
        &self,
        textura_horizontal: f64,
        textura_vertical: f64,
        lugar: &crate::vec3::Point3,
    ) -> crate::color::Color {
        if self.imagen.alto() == 0 {
            return crate::color::Color::new(0.0, 1.0, 1.0);
        }

        let u = f64::clamp(textura_horizontal, 0.0, 1.0);
        let v = 1.0 - f64::clamp(textura_vertical, 0.0, 1.0);
        //let u = Intervalo::new(0.0, 1.0).limitar(textura_horizontal);
        //let v = 1.0 - Intervalo::new(0.0, 1.0).limitar(textura_vertical);

        let i = (u * (self.imagen.ancho() - 1) as f64) as i32;
        let j = (v * (self.imagen.alto() - 1) as f64) as i32;

        let p = self.imagen.pixel_from(i, j);
        pixel_to_color(p)
    }
}
