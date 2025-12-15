use crate::{Color::Color, Golpe::Golpe, Material::Material, Rayo::Rayo, Vec3::Vec3};

pub struct Difuso_lambertiano {
    color: Color,
}

impl Difuso_lambertiano {
    #[inline]
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material for Difuso_lambertiano {
    fn dispersion(&self, _rayo_entrante: &Rayo, golpe: &Golpe) -> Option<(Rayo, Color)> {

        let mut direccion_dispersion = golpe.normal() + Vec3::normalizado_random();

        if direccion_dispersion.cerca_de_cero() {
            direccion_dispersion = golpe.normal();
        }

        let rayo_resultante = Rayo::new(golpe.lugar(), direccion_dispersion);

        Some((rayo_resultante, self.color))
    }
}
