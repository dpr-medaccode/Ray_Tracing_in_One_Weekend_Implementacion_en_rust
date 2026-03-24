use std::cmp::Ordering;
use std::sync::Arc;

use crate::{
    caja::{self, Caja, Eje},
    golpe::{Golpe, golpeable::Golpeable, objeto::Objeto},
    intervalo::Intervalo,
    rayo::Rayo,
};

pub struct Nodo {
    izquierda: Box<Objeto>,
    derecha: Option<Box<Objeto>>,
    caja: Caja,
}

impl Nodo {

    pub fn new(objetos: &mut Vec<Box<Objeto>>) -> Self {
        let mut caja_total = Caja::vacio();

        for o in objetos.iter() {
            caja_total = Caja::new_from_cajas(caja_total, o.caja());
        }

        let eje = caja_total.eje_mas_largo();

        let comparador = match eje {
            Eje::X => Nodo::comparar_caja_x,
            Eje::Y => Nodo::comparar_caja_y,
            Eje::Z => Nodo::comparar_caja_z,
        };

        let tamanno = objetos.len();

        let izquierda: Box<Objeto>;
        let derecha: Option<Box<Objeto>>;
        let caja: Caja;

        match tamanno {
            0 => panic!("Nodo::new llamado con lista vacía"),

            1 => {
                izquierda = objetos.remove(0);
                derecha = None;
                caja = izquierda.caja();
            }

            2 => {
                objetos.sort_by(comparador);
                 derecha = Some(objetos.remove(1));
                izquierda = objetos.remove(0);
                caja = Caja::new_from_cajas(izquierda.caja(), derecha.as_ref().unwrap().caja());
            }
            _ => {
                objetos.sort_by(comparador);
                let mid = tamanno / 2;

                let mut derecha_vec = objetos.split_off(mid);

                izquierda = Box::new(Objeto::Nodo(Nodo::new(objetos)));
                derecha = Some(Box::new(Objeto::Nodo(Nodo::new(&mut derecha_vec))));
                caja = Caja::new_from_cajas(izquierda.caja(), derecha.as_ref().unwrap().caja());
            }
        }

        Nodo {
            izquierda,
            derecha,
            caja,
        }
    }

    pub fn comparar_caja_x(a: &Box<Objeto>, b: &Box<Objeto>) -> Ordering {
        a.caja()
            .x
            .minimo
            .partial_cmp(&b.caja().x.minimo)
            .unwrap_or(Ordering::Equal)
    }

    pub fn comparar_caja_y(a: &Box<Objeto>, b: &Box<Objeto>) -> Ordering {
        a.caja()
            .y
            .minimo
            .partial_cmp(&b.caja().y.minimo)
            .unwrap_or(Ordering::Equal)
    }

    pub fn comparar_caja_z(a: &Box<Objeto>, b: &Box<Objeto>) -> Ordering {
        a.caja()
            .z
            .minimo
            .partial_cmp(&b.caja().z.minimo)
            .unwrap_or(Ordering::Equal)
    }
}

impl Golpeable for Nodo {
    fn caja(&self) -> Caja {
        self.caja
    }

    fn rayo_golpea(&self, rayo: &Rayo, intervalo: Intervalo) -> Option<Golpe> {
        let intervalo = self.caja.rayo_golpea_caja(rayo, intervalo)?;

        let golpe_izq = self.izquierda.rayo_golpea(rayo, intervalo);

        let nuevo_intervalo = if let Some(ref golpe) = golpe_izq {
            Intervalo::new(intervalo.minimo, golpe.distancia())
        } else {
            intervalo
        };

        let golpe_der = if let Some(ref derecha) = self.derecha {
            derecha.rayo_golpea(rayo, nuevo_intervalo)
        } else {
            None
        };

        golpe_der.or(golpe_izq)
    }
}
