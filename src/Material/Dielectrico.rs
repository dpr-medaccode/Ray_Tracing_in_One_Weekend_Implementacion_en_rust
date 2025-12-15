use rand::Rng;

use crate::{
    Color::{BLANCO, Color},
    Golpe::Golpe,
    Material::Material,
    Rayo::Rayo,
    Vec3::Vec3,
};

pub struct Dielectrico {
    indice_refraccion: f64,
}

impl Dielectrico {
    pub fn new(indice_refraccion: f64) -> Self {
        Self { indice_refraccion }
    }

    #[inline]
    fn reflectancia(coseno: f64, indice_refraccion: f64) -> f64 {
        let mut r0 = (1.0 - indice_refraccion) / (1.0 + indice_refraccion);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - coseno).powi(5)
    }
}

impl Material for Dielectrico {
    fn dispersion(&self, rayo_entrante: &Rayo, golpe: &Golpe) -> Option<(Rayo, Color)> {
        let atenuacion = BLANCO;

        // etai_over_etat
        let etai_over_etat = if golpe.cara_frontal() {
            1.0 / self.indice_refraccion
        } else {
            self.indice_refraccion
        };

        let direccion_unitaria = Vec3::normalizar(&rayo_entrante.direccion());

        let cos_theta = Vec3::punto(&-direccion_unitaria, &golpe.normal()).min(1.0);

        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let no_puede_refractar = etai_over_etat * sin_theta > 1.0;

        let mut rng = rand::rng();

        let direccion = if no_puede_refractar
            || Dielectrico::reflectancia(cos_theta, etai_over_etat) > rng.random::<f64>()
        {
            Vec3::reflejar(&direccion_unitaria, &golpe.normal())
        } else {
            Vec3::refractar(&direccion_unitaria, &golpe.normal(), etai_over_etat)
        };

        let rayo_salida = Rayo::new(golpe.lugar(), direccion);

        Some((rayo_salida, atenuacion))
    }
}
