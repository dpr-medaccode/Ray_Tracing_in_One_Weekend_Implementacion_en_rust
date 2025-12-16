use crate::{Color::Color, Golpe::Golpe, Material::Material, Rayo::Rayo, Vec3::Vec3};

pub struct DifusoLambertiano {
    color: Color,
}

impl DifusoLambertiano {
    #[inline]
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material for DifusoLambertiano {
    fn dispersion(&self, rayo_entrante: &Rayo, golpe: &Golpe) -> Option<(Rayo, Color)> {
        let mut direccion_dispersion = golpe.normal() + Vec3::normalizado_random();

        if direccion_dispersion.cerca_de_cero() {
            direccion_dispersion = golpe.normal();
        }

        let rayo_resultante = Rayo::new_con_tiempo(golpe.lugar(), direccion_dispersion, rayo_entrante.tiempo());

        Some((rayo_resultante, self.color))
    }
}
