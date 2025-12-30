use image::{ImageReader, RgbImage};

use crate::{color::Color, vec3::Vec3};

pub struct Imagen {
    imagen: RgbImage,
}

impl Imagen {
    pub fn imagen(&self) -> RgbImage {
        self.imagen.clone()
    }

    pub fn ancho(&self) -> i32 {
        self.imagen.width() as i32
    }

    pub fn alto(&self) -> i32 {
        self.imagen.height() as i32
    }

    pub fn new(ruta: &str) -> Self {
        let img = ImageReader::open(ruta)
            .expect("No se pudo abrir la imagen")
            .decode()
            .expect("Formato inválido")
            .to_rgb8();

        Self { imagen: img }
    }

    pub fn pixel_from(&self, x: i32, y: i32) -> Pixel {
        if self.imagen.is_empty() {
            return [255, 0, 255]; // magenta
        }

        let x = x.clamp(0, self.ancho() - 1) as u32;
        let y = y.clamp(0, self.alto() - 1) as u32;

        let p = self.imagen.get_pixel(x, y);
        [p[0], p[1], p[2]]
    }
}

type Pixel = [u8; 3];

pub fn pixel_to_color(p: Pixel) -> Color {
    let r = srgb_a_lineal(p[0] as f64 / 255.0);
    let g = srgb_a_lineal(p[1] as f64 / 255.0);
    let b = srgb_a_lineal(p[2] as f64 / 255.0);

    Vec3::new(r, g, b)
}

fn srgb_a_lineal(c: f64) -> f64 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}
