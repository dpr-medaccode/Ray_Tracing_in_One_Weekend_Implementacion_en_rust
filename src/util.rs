use rand::Rng;
use std::f64::consts::PI;

use crate::Intervalo::Intervalo;

pub fn grados_a_radianes(grados: f64) -> f64 {
    grados * PI / 180.0
}

pub fn random_entre(minimo: f64, maximo: f64) -> f64 {
    let mut rng = rand::rng();
    minimo + (maximo - minimo) * rng.random::<f64>()
}

pub fn random_entre_intervalo(intervalo: Intervalo) -> f64 {
    let mut rng = rand::rng();
    intervalo.minimo + (intervalo.maximo - intervalo.minimo) * rng.random::<f64>()
}
