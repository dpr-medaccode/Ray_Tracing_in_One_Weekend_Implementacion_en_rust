use std::sync::Arc;

use crate::{
    Caja::Caja,
    Golpe::{Golpe, Golpeable::Golpeable},
    Intervalo::Intervalo,
    Material::Material,
    Rayo::Rayo,
    Vec3::Vec3,
};



pub struct Esfera {
    lugar: Rayo,
    radio: f64,
    material: Arc<dyn Material>,
    caja: Caja,
}



impl Esfera {
    pub fn new_estatica(lugar: Vec3, radio: f64, material: Arc<dyn Material>) -> Self {
        let tamanno = Vec3::new(radio, radio, radio);

        Self {
            lugar: Rayo::new(lugar, Vec3::new(0.0, 0.0, 0.0)),
            radio,
            material,
            caja: Caja::new(lugar - tamanno, lugar + tamanno),
        }
    }

    pub fn new_en_movimiento(
        lugar1: Vec3,
        lugar2: Vec3,
        radio: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let tamanno = Vec3::new(radio, radio, radio);
        let lugar = Rayo::new(lugar1, lugar2 - lugar1);

        let caja1 = Caja::new(lugar.en(0.0) - tamanno, lugar.en(0.0) + tamanno);
        let caja2 = Caja::new(lugar.en(1.0) - tamanno, lugar.en(1.0) + tamanno);

        Self {
            lugar,
            radio,
            material,
            caja: Caja::new_from_cajas(caja1, caja2),
        }
    }
}

impl Golpeable for Esfera {
    fn caja(&self) -> Caja {
        self.caja
    }

    fn rayo_golpea(&self, rayo: &Rayo, intervalo: Intervalo) -> Option<Golpe> {
        // Vector desde el origen del rayo hasta el centro de la esfera

        let lugar_actual = self.lugar.en(rayo.tiempo());

        let oc = lugar_actual - rayo.origen();
 
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
        let normal_exterior = (lugar_golpe - lugar_actual) / self.radio; // Vec3::normalizar(&(lugar_golpe - lugar_actual)); lo mismo

        Some(Golpe::new(
            lugar_golpe,
            normal_exterior,
            distancia,
            rayo,
            Arc::clone(&self.material),
        ))
    }
}
