use std::sync::Arc;

use crate::{
    color::Color,
    material::Material,
    rayo::Rayo,
    textura::{Textura, color_solido::ColorSolido},
    vec3::Vec3,
};

pub struct Isotropico {
    textura: Arc<dyn Textura>,
}

impl Isotropico {
    pub fn new_from_color(color: Color) -> Self {
        Isotropico {
            textura: Arc::new(ColorSolido::new(color)),
        }
    }

    pub fn new_from_textura(textura: Arc<dyn Textura>) -> Self {
        Isotropico { textura }
    }
}

impl Material for Isotropico {
    fn dispersion(
        &self,
        _rayo_entrante: &crate::rayo::Rayo,
        _golpe: &crate::golpe::Golpe,
    ) -> Option<(crate::rayo::Rayo, Color)> {
        let dispersion = Rayo::new_con_tiempo(
            _golpe.lugar(),
            Vec3::normalizado_random(),
            _rayo_entrante.tiempo(),
        );

        let ateniacion = self.textura.valor(
            _golpe.textura_horizontal(),
            _golpe.textura_vertical(),
            &_golpe.lugar(),
        );

        Some((dispersion, ateniacion))
    }
}
