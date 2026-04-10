#![allow(dead_code)]
use crate::{
    caja::Caja,
    golpe::{golpeable::Golpeable, objeto::Objeto},
    rayo::Rayo,
    vec3::Vec3,
};

pub struct Trasladar {
    objeto: Box<Objeto>, // arc es necesario?
    desplazo: Vec3,
    caja: Caja,
}

impl Trasladar {
    pub fn new(objeto: Box<Objeto>, desplazo: Vec3) -> Self {
        let caja = objeto.caja() + desplazo;

        Trasladar {
            objeto,
            desplazo,
            caja,
        }
    }
}

impl Golpeable for Trasladar {
    fn caja(&self) -> Caja {
        self.caja
    }

    fn rayo_golpea(
        &self,
        rayo: &crate::rayo::Rayo,
        intervalo: crate::intervalo::Intervalo,
    ) -> Option<super::Golpe> {
        let rayo_desplazado = Rayo::new_con_tiempo(
            rayo.origen() - self.desplazo,
            rayo.direccion(),
            rayo.tiempo(),
        );

        if let Some(mut golpe) = self.objeto.rayo_golpea(&rayo_desplazado, intervalo) {
            golpe.lugar = golpe.lugar + self.desplazo;

            return Some(golpe);
        }

        None
    }
}
