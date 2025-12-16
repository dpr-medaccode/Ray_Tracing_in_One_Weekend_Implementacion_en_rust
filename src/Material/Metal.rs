use crate::{Color::Color, Golpe::Golpe, Material::Material, Rayo::Rayo, Vec3::Vec3};

pub struct Metal {
    color: Color,
    borroso: f64,
}

impl Metal {
    #[inline]
    pub fn new(color: Color, borroso: f64) -> Self {
        Self {
            color,
            borroso: if borroso < 1.0 { borroso } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn dispersion(&self, rayo_entrante: &Rayo, golpe: &Golpe) -> Option<(Rayo, Color)> {
        let reflejado = Vec3::reflejar(&rayo_entrante.direccion(), &golpe.normal());

        let reflejado = reflejado + self.borroso * Vec3::normalizado_random();

        let rayo_salida = Rayo::new(golpe.lugar(), reflejado);

        if Vec3::punto(&rayo_salida.direccion(), &golpe.normal()) <= 0.0 {
            return None;
        }

        Some((rayo_salida, self.color))
    }
}
