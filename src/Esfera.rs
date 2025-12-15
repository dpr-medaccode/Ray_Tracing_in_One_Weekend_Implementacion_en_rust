use std::sync::Arc;

use crate::{
    Golpe::{Golpe, Golpeable::Golpeable},
    Intervalo::Intervalo,
    Material::Material,
    Rayo::Rayo,
    Vec3::Vec3,
};

pub struct Esfera {
    lugar: Vec3,
    radio: f64,
    material: Arc<dyn Material>,
}

impl Esfera {
    pub fn new(lugar: Vec3, radio: f64, material: Arc<dyn Material>) -> Self {
        Self {
            lugar,
            radio,
            material,
        }
    }
}

impl Golpeable for Esfera {
    fn rayo_golpea(&self, rayo: &Rayo, intervalo: &Intervalo) -> Option<Golpe> {
        // Vector desde el origen del rayo hasta el centro de la esfera
        let oc = self.lugar - rayo.origen();

        // Coeficientes cuadráticos para la ecuación del rayo-esfera
        let a = Vec3::punto(&rayo.direccion(), &rayo.direccion()); // D·D
        let h = Vec3::punto(&rayo.direccion(), &oc); // D·(C-Q)
        let c = oc.logitud_cuadrada() - self.radio * self.radio; // (C-Q)·(C-Q) - R²

        let discriminante = h * h - a * c;

        // Si el discriminante es negativo, no hay intersección
        if discriminante < 0.0 {
            return None;
        }

        let raiz_discriminante = discriminante.sqrt();

        // Primer posible t
        let mut distancia = (h - raiz_discriminante) / a;
        if distancia < intervalo.minimo || distancia > intervalo.maximo {
            // Segundo posible t
            distancia = (h + raiz_discriminante) / a;
            if distancia < intervalo.minimo || distancia > intervalo.maximo {
                return None;
            }
        }

        let lugar_golpe = rayo.en(distancia);

        // Normal de la superficie apuntando hacia afuera
        let normal_exterior = Vec3::normalizar(&(lugar_golpe - self.lugar));

        Some(Golpe::new(
            lugar_golpe,
            normal_exterior,
            distancia,
            rayo,
            Arc::clone(&self.material),
        ))
    }
}
