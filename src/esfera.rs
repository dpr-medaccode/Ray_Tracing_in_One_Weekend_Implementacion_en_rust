#![allow(dead_code)]

use std::sync::Arc;

use crate::{
    caja::Caja,
    golpe::{Golpe, golpeable::Golpeable},
    intervalo::Intervalo,
    material::Material,
    rayo::Rayo,
    vec3::{Point3, Vec3},
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

    fn cordenada_textura(lugar: Point3) -> (f64, f64) {
        // lugar: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-lugar.y()).acos();
        let phi = (-lugar.z()).atan2(lugar.x()) + std::f64::consts::PI;

        (phi / (2.0 * std::f64::consts::PI), theta / std::f64::consts::PI)
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
        let normal_exterior = Vec3::normalizar(&(lugar_golpe - lugar_actual));


        let (textura_horizontal, textura_vertical) = Esfera::cordenada_textura(normal_exterior);

        let golpe =  Golpe::new(
            lugar_golpe,
            normal_exterior,
            distancia,
            textura_horizontal,
            textura_vertical,
            rayo,
            Arc::clone(&self.material),
        );
        
        Some(golpe)
    }
}
