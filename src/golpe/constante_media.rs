use std::sync::Arc;

use crate::{
    color::{ Color},
    golpe::{Golpe, golpeable::Golpeable},
    intervalo::Intervalo,
    material::{Material, isotropico::Isotropico},
    rayo::Rayo,
    textura::Textura,
    util::ramdom_f64,
    vec3::Vec3,
};

pub struct ConstanteMedia {
    limite: Arc<dyn Golpeable>,

    densidad_invertida_negativa: f64,

    funcion_fase: Arc<dyn Material>,
}

impl ConstanteMedia {
    pub fn new_from_textura(
        limite: Arc<dyn Golpeable>,
        densidad: f64,
        textura: Arc<dyn Textura>,
    ) -> Self {
        let densidad_invertida_negativa = -1.0 / densidad;

        ConstanteMedia {
            limite,
            densidad_invertida_negativa,
            funcion_fase: Arc::new(Isotropico::new_from_textura(textura)),
        }
    }

    pub fn new_from_color(limite: Arc<dyn Golpeable>, densidad: f64, color: Color) -> Self {
        let densidad_invertida_negativa = -1.0 / densidad;

        ConstanteMedia {
            limite,
            densidad_invertida_negativa,
            funcion_fase: Arc::new(Isotropico::new_from_color(color)),
        }
    }
}

impl Golpeable for ConstanteMedia {
    fn caja(&self) -> crate::caja::Caja {
        self.limite.caja()
    }

    fn rayo_golpea(&self, rayo: &Rayo, intervalo: Intervalo) -> Option<Golpe> {
        // Primer golpe
        let mut golpe_1 = match self.limite.rayo_golpea(rayo, Intervalo::universo()) {
            Some(hit) => hit,
            None => return None,
        };

        // Segundo golpe
        let mut golpe_2 = match self.limite.rayo_golpea(
            rayo,
            Intervalo::new(golpe_1.distancia + 0.0001, f64::INFINITY),
        ) {
            Some(hit) => hit,
            None => return None,
        };

        // Ajustar intervalo
        golpe_1.distancia = golpe_1.distancia.max(intervalo.minimo);
        golpe_2.distancia = golpe_2.distancia.min(intervalo.maximo);

        if golpe_1.distancia >= golpe_2.distancia {
            return None;
        }

        if golpe_1.distancia < 0.0 {
            golpe_1.distancia = 0.0;
        }

        let longitud_rayo = rayo.direccion().logitud();
        let distancia_dentro_limite = (golpe_2.distancia - golpe_1.distancia) * longitud_rayo;
        let distancia_golpe = self.densidad_invertida_negativa * ramdom_f64().ln();

        if distancia_golpe > distancia_dentro_limite {
            return None;
        }

        let distancia = golpe_1.distancia + distancia_golpe / longitud_rayo;
        let lugar = rayo.en(distancia);

        
        let normal = Vec3::new(1.0, 0.0, 0.0);

        Some(Golpe {
            lugar,
            normal,
            distancia,
            textura_horizontal: golpe_1.textura_horizontal(),
            textura_vertical: golpe_1.textura_horizontal(),
            cara_frontal: true,
            material: self.funcion_fase.clone(),
        })
    }
}
