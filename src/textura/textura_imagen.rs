use image::{DynamicImage, GenericImageView, ImageReader, Pixel};

use crate::{
    color::Color,
    textura::Textura,
};

pub struct TexturaImagen {
    imagen: DynamicImage,
}

impl TexturaImagen {
    pub fn new(ruta: &str) -> Self {
        let imagen = ImageReader::open(ruta)
            .expect("should be a valid image path")
            .decode()
            .expect("should decode");

        Self { imagen }
    }

    pub fn ancho(&self) -> u32 {
        self.imagen.width()
    }

    pub fn alto(&self) -> u32 {
        self.imagen.height()
    }

    pub fn pixel_from(&self, x: u32, y: u32) -> Color {
        let clamped_x = u32::clamp(x, 0, self.ancho() - 1);
        let calmped_y = u32::clamp(y, 0, self.alto() - 1);

        let pixel = self.imagen.get_pixel(clamped_x, calmped_y).to_rgb();

        let r = ((pixel.0[0] as f64) / 255.0).powf(2.2);
        let g = ((pixel.0[1] as f64) / 255.0).powf(2.2);
        let b = ((pixel.0[2] as f64) / 255.0).powf(2.2);

        Color::new(r, g, b)
    }
}

impl Textura for TexturaImagen {
    fn valor(
        &self,
        textura_horizontal: f64,
        textura_vertical: f64,
        _lugar: &crate::vec3::Point3,
    ) -> crate::color::Color {
        if self.ancho() == 0 {
            return crate::color::Color::new(0.0, 1.0, 1.0);
        }

        let u = f64::clamp(textura_horizontal, 0.0, 1.0);
        let v = 1.0 - f64::clamp(textura_vertical, 0.0, 1.0);
        //let u = Intervalo::new(0.0, 1.0).limitar(textura_horizontal);
        //let v = 1.0 - Intervalo::new(0.0, 1.0).limitar(textura_vertical);

        let i = (u * (self.ancho() - 1) as f64) as u32;
        let j = (v * (self.alto() - 1) as f64) as u32;

        self.pixel_from(i, j)
    }
}
