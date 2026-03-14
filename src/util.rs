#![allow(dead_code)]

use rand::Rng;
use std::f64::consts::PI;

use crate::intervalo::Intervalo;

pub fn grados_a_radianes(grados: f64) -> f64 {
    grados * PI / 180.0
}

pub fn ramdom_f64() -> f64 {
    let mut rng = rand::rng();
    rng.random::<f64>()
}

pub fn random_f64_entre(minimo: f64, maximo: f64) -> f64 {
    let mut rng = rand::rng();
    minimo + (maximo - minimo) * rng.random::<f64>()
}

pub fn ramdom_i32() -> i32 {
    let mut rng = rand::rng();
    rng.random::<i32>()
}

pub fn random_i32_entre(minimo: i32, maximo: i32) -> i32 {
    let mut rng = rand::rng();
    minimo + (maximo - minimo) * rng.random::<i32>()
}

pub fn random_usize_entre(min: usize, max: usize) -> usize {
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}

/*pub fn random_usize_entre(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
} */

pub fn random_entre_intervalo(intervalo: Intervalo) -> f64 {
    let mut rng = rand::rng();
    intervalo.minimo + (intervalo.maximo - intervalo.minimo) * rng.random::<f64>()
}
