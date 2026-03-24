#![allow(dead_code)]
use crate::{golpe::{golpeable::Golpeable, objeto::Objeto}, intervalo::Intervalo, rayo::Rayo, vec3::Vec3};

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

pub fn color_rayo(rayo: &Rayo, mundo: &Objeto, profundidad: i32, fondo: Color) -> Color {
    if profundidad <= 0 {
        return NEGRO;
    }

    let Some(golpe) = mundo.rayo_golpea(rayo, Intervalo::new(0.001, f64::INFINITY)) else {
        return fondo;
    };

    let luz = golpe.material().luz_emitida(
        golpe.textura_horizontal(),
        golpe.textura_vertical(),
        &golpe.lugar(),
    );

    let Some((dispersion, atenuacion)) = golpe.material().dispersion(rayo, &golpe) else {
        return luz;
    };

    luz + atenuacion * color_rayo(&dispersion, mundo, profundidad - 1, fondo)
}

pub fn mezclar_colores(color_1: &Color, color_2: &Color) -> Color {
    Color::new(
        color_1.x() / 2.0 + color_2.x() / 2.0,
        color_1.y() / 2.0 + color_2.y() / 2.0,
        color_1.z() / 2.0 + color_2.z() / 2.0,
    )
}

pub fn oscurecer_color(color: &Color) -> Color {
    Color::new(color.x() / 2.0, color.y() / 2.0, color.z() / 2.0)
}

pub fn aclasrar_color(color: &Color) -> Color {
    Color::new(color.x() * 2.0, color.y() * 2.0, color.z() * 2.0)
}

fn to_linear(c: f64) -> f64 {
    c.powf(2.2)
}

fn to_srgb(c: f64) -> f64 {
    c.powf(1.0 / 2.2)
}

pub fn atenuar_color(color: &Color) -> Color {
    Color::new(
        to_srgb(to_linear(color.x()) * 0.5),
        to_srgb(to_linear(color.y()) * 0.5),
        to_srgb(to_linear(color.z()) * 0.5),
    )
}
