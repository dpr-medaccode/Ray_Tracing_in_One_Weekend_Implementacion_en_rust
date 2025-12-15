use crate::{Golpe::Golpeable::Golpeable, Intervalo::Intervalo, Rayo::Rayo, Vec3::Vec3};

pub type Color = Vec3;

pub const NEGRO: Color = Color::new(0.0, 0.0, 0.0);
pub const BLANCO: Color = Color::new(1.0, 1.0, 1.0);
pub const GRIS: Color = Color::new(0.5, 0.5, 0.5);
pub const ROJO: Color = Color::new(1.0, 0.0, 0.0);
pub const VERDE: Color = Color::new(0.0, 1.0, 0.0);
pub const AZUL: Color = Color::new(0.0, 0.0, 1.0);
pub const AMARILLO: Color = Color::new(1.0, 1.0, 0.0);
pub const CIAN: Color = Color::new(0.0, 1.0, 1.0);
pub const MAGENTA: Color = Color::new(1.0, 0.0, 1.0);
pub const NARANJA: Color = Color::new(1.0, 0.5, 0.0);
pub const LIMA: Color = Color::new(0.5, 1.0, 0.0);
pub const TURQUESA: Color = Color::new(0.0, 0.5, 1.0);
pub const ROSA: Color = Color::new(1.0, 0.5, 0.5);
pub const LAVANDA: Color = Color::new(0.7, 0.5, 1.0);
pub const INDIGO: Color = Color::new(0.29, 0.0, 0.51);

pub const CIELO: Color = Color::new(0.5, 0.7, 1.0);
pub const ATARDECER: Color = Color::new(1.0, 0.6, 0.2);

const INTENSIDAD: Intervalo = Intervalo {
    minimo: 0.0,
    maximo: 0.999,
};

pub fn escribir_color(pixel_color: &Color) -> String {
    format!(
        "{} {} {}\n",
        to_byte(linear_a_gamma(pixel_color.x())),
        to_byte(linear_a_gamma(pixel_color.y())),
        to_byte(linear_a_gamma(pixel_color.z()))
    )
}

fn linear_a_gamma(elemento_linear: f64) -> f64 {
    if elemento_linear > 0.0 {
        elemento_linear.sqrt()
    } else {
        0.0
    }
}

fn to_byte(v: f64) -> u8 {
    (256.0 * INTENSIDAD.limitar(v)) as u8
}

pub fn color_rayo(rayo: &Rayo, mundo: &dyn Golpeable, profundidad: i32) -> Color {

    if let Some(golpe) = mundo.rayo_golpea(rayo, &Intervalo::new(0.001, f64::INFINITY)) {
        
        if profundidad <= 0 {
            return NEGRO;
        }

        if let Some((dispersion, atenuacion)) = golpe.material().dispersion(rayo, &golpe) {
            return atenuacion * color_rayo(&dispersion, mundo, profundidad - 1);
        }

        return NEGRO;
    }

    let unidad = Vec3::normalizar(&rayo.direccion());
    let t = 0.5 * (unidad.y() + 1.0);
    BLANCO * (1.0 - t) + CIELO * t
}
