use crate::{Golpe::Golpe, Intervalo::Intervalo, Rayo::Rayo};

pub trait Golpeable: Send + Sync {
    fn rayo_golpea(&self, rayo: &Rayo, intervalo: &Intervalo) -> Option<Golpe>;
}
 