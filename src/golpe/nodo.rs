use std::cmp::Ordering;
use std::sync::Arc;

use crate::{
    caja::{Caja, Eje},
    golpe::{Golpe, golpeable::Golpeable, lista_golpeable::ListaGolpeable},
    intervalo::Intervalo,
    rayo::Rayo,
};

pub struct Nodo {
    izquierda: Arc<dyn Golpeable>,
    derecha: Arc<dyn Golpeable>,
    caja: Caja,
}

impl Nodo {
    pub fn new_from_lista(lista: &mut ListaGolpeable) -> Self {
        Nodo::new(&mut lista.objetos(), 0, lista.objetos().len())
    }

    pub fn new(objetos: &mut Vec<Arc<dyn Golpeable>>, inicio: usize, fin: usize) -> Self {
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

        let izquierda: Arc<dyn Golpeable>;
        let derecha: Arc<dyn Golpeable>;

        match tamanno {
            1 => {
                izquierda = objetos[inicio].clone();
                derecha = objetos[inicio].clone();
            }

            2 => {
                izquierda = objetos[inicio].clone();
                derecha = objetos[inicio + 1].clone();
            }

            _ => {
                objetos[inicio..fin].sort_by(comparador);

                let mid = inicio + tamanno / 2;

                izquierda = Arc::new(Nodo::new(objetos, inicio, mid));
                derecha = Arc::new(Nodo::new(objetos, mid, fin));
            }
        }

        let caja_izq = izquierda.caja();
        let caja_der = derecha.caja();

        Nodo {
            izquierda,
            derecha,
            caja: Caja::new_from_cajas(caja_izq, caja_der),
        }
    }

    pub fn comparar_caja_x(a: &Arc<dyn Golpeable>, b: &Arc<dyn Golpeable>) -> Ordering {
        a.caja()
            .x
            .minimo
            .partial_cmp(&b.caja().x.minimo)
            .unwrap_or(Ordering::Equal)
    }

    pub fn comparar_caja_y(a: &Arc<dyn Golpeable>, b: &Arc<dyn Golpeable>) -> Ordering {
        a.caja()
            .y
            .minimo
            .partial_cmp(&b.caja().y.minimo)
            .unwrap_or(Ordering::Equal)
    }

    pub fn comparar_caja_z(a: &Arc<dyn Golpeable>, b: &Arc<dyn Golpeable>) -> Ordering {
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

        let golpe_der = self.derecha.rayo_golpea(rayo, nuevo_intervalo);

        golpe_der.or(golpe_izq)
    }
}
