use crate::{Caja::Caja, Golpe::Golpe, Intervalo::Intervalo, Rayo::Rayo};

pub trait Golpeable: Send + Sync {
    fn rayo_golpea(&self, rayo: &Rayo, intervalo: &Intervalo) -> Option<Golpe>;

    fn caja(&self) -> Caja;
}
