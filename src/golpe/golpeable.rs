use crate::{caja::Caja, golpe::Golpe, intervalo::Intervalo, rayo::Rayo};

pub trait Golpeable: Send + Sync {
    fn rayo_golpea(&self, rayo: &Rayo, intervalo: Intervalo) -> Option<Golpe>;

    fn caja(&self) -> Caja;
}
