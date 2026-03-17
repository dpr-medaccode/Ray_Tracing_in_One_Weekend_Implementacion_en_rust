use std::cmp::Ordering;
use std::sync::Arc;

use crate::{
    caja::{self, Caja, Eje},
    golpe::{Golpe, golpeable::Golpeable, lista_golpeable::ListaGolpeable},
    intervalo::Intervalo,
    rayo::Rayo,
};

pub struct Nodo {
    izquierda: Box<dyn Golpeable>,
    derecha: Option<Box<dyn Golpeable>>,
    caja: Caja,
}

impl Nodo {
    pub fn new_from_lista(mut objetos: Vec<Box<dyn Golpeable>>) -> Self {
        let len = objetos.len();
        Nodo::new(&mut objetos, 0, len)
    }

    pub fn new(objetos: &mut Vec<Box<dyn Golpeable>>, inicio: usize, fin: usize) -> Self {
        let mut caja = Caja::vacio();

        for i in inicio..fin {
            caja = Caja::new_from_cajas(caja, objetos[i].caja())
        }

        let eje = caja.eje_mas_largo();

        let comparador = match eje {
            Eje::X => Nodo::comparar_caja_x,
            Eje::Y => Nodo::comparar_caja_y,
            Eje::Z => Nodo::comparar_caja_z,
        };

        let tamanno = fin - inicio;

        let izquierda: Box<dyn Golpeable>;
        let derecha: Option<Box<dyn Golpeable>>;

        match tamanno {
            1 => {
                //izquierda = objetos.remove(inicio);
                //derecha = None;
            }

            2 => {
               //izquierda = objetos.remove(inicio);
               //derecha = Some(objetos.remove(inicio));
            }
            _ => {
                objetos[inicio..fin].sort_by(comparador);
                let mid = inicio + tamanno / 2;
                izquierda = Box::new(Nodo::new(objetos, inicio, mid));
                derecha = Some(Box::new(Nodo::new(objetos, mid, fin)));
            }
        }

        let caja_izq = izquierda.caja();

        let caja = if let Some(ref d) = derecha {
            Caja::new_from_cajas(caja_izq, d.caja())
        } else {
            caja_izq
        };

        Nodo {
            izquierda,
            derecha,
            caja,
        }
    }

    pub fn comparar_caja_x(a: &Box<dyn Golpeable>, b: &Box<dyn Golpeable>) -> Ordering {
        a.caja()
            .x
            .minimo
            .partial_cmp(&b.caja().x.minimo)
            .unwrap_or(Ordering::Equal)
    }

    pub fn comparar_caja_y(a: &Box<dyn Golpeable>, b: &Box<dyn Golpeable>) -> Ordering {
        a.caja()
            .y
            .minimo
            .partial_cmp(&b.caja().y.minimo)
            .unwrap_or(Ordering::Equal)
    }

    pub fn comparar_caja_z(a: &Box<dyn Golpeable>, b: &Box<dyn Golpeable>) -> Ordering {
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
