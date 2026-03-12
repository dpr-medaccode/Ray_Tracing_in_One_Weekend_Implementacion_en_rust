#![allow(dead_code)]

use std::sync::Arc;

use crate::{
    color::Color,
    golpe::Golpe,
    material::Material,
    rayo::Rayo,
    textura::{Textura, color_solido::ColorSolido},
    vec3::Vec3,
};

pub struct DifusoLambertiano {
    textura: Arc<dyn Textura>,
}

impl DifusoLambertiano {
    #[inline]
    pub fn new_from_color(color: Color) -> Self {
        Self {
            textura: Arc::new(ColorSolido::new(color)),
        }
    }

    pub fn new_from_textura(textura: Arc<dyn Textura>) -> Self {
        Self { textura }
    }
}

impl Material for DifusoLambertiano {
    fn dispersion(&self, rayo_entrante: &Rayo, golpe: &Golpe) -> Option<(Rayo, Color)> {
        let mut direccion_dispersion = golpe.normal() + Vec3::normalizado_random();

        if direccion_dispersion.cerca_de_cero() {
            direccion_dispersion = golpe.normal();
        }

        let rayo_resultante =
            Rayo::new_con_tiempo(golpe.lugar(), direccion_dispersion, rayo_entrante.tiempo());

        Some((
            rayo_resultante,
            self.textura.valor(
                golpe.textura_horizontal(),
                golpe.textura_vertical(),
                &golpe.lugar(),
            ),
        ))
    }
}
