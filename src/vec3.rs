use rand::Rng;

use crate::{
    intervalo::Intervalo,
    util::{random_f64_entre, random_entre_intervalo},
};
use std::ops::{Add, Div, Index, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    #[inline]
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    #[inline]
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    #[inline]
    pub fn logitud(&self) -> f64 {
        self.logitud_cuadrada().sqrt()
    }
    #[inline]
    pub fn logitud_cuadrada(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    #[inline]
    pub fn cerca_de_cero(&self) -> bool {
        self.x().abs() < 1e-8 && self.y().abs() < 1e-8 && self.z().abs() < 1e-8
    }

    #[inline]
    pub fn punto(u: &Self, v: &Self) -> f64 {
        u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
    }

    #[inline]
    pub fn cruz(u: &Self, v: &Self) -> Self {
        Self::new(
            u.y() * v.z() - u.z() * v.y(),
            u.z() * v.x() - u.x() * v.z(),
            u.x() * v.y() - u.y() * v.x(),
        )
    }

    #[inline]
    pub fn normalizar(valor: &Self) -> Self {
        *valor / valor.logitud()
    }

    #[inline]
    pub fn reflejar(vector: &Self, normal: &Self) -> Self {
        *vector - 2.0 * Self::punto(vector, normal) * *normal
    }

    #[inline]
    pub fn refractar(uv: &Self, normal: &Self, relacion_indices: f64) -> Self {
        let cos_theta = Self::punto(&-*uv, normal).min(1.0);

        let componente_perpendicular = relacion_indices * (*uv + cos_theta * *normal);

        let componente_paralelo = -*normal
            * (1.0 - componente_perpendicular.logitud_cuadrada())
                .abs()
                .sqrt();

        componente_perpendicular + componente_paralelo
    }

    pub fn ramdon() -> Self {
        let mut rng = rand::rng();

        Self::new(
            rng.random::<f64>(),
            rng.random::<f64>(),
            rng.random::<f64>(),
        )
    }

    pub fn normalizado_random() -> Self {
        loop {
            let punto = Self::random_entre(-1.0, 1.0);
            if punto.logitud() > 1e-8 && punto.logitud_cuadrada() <= 1.0 {
                return punto;
            }
        }
    }

    pub fn random_entre(minimo: f64, maximo: f64) -> Self {
        Self::new(
            random_f64_entre(minimo, maximo),
            random_f64_entre(minimo, maximo),
            random_f64_entre(minimo, maximo),
        )
    }

    pub fn random_entre_intervalo(intervalo: Intervalo) -> Self {
        Self::new(
            random_entre_intervalo(intervalo),
            random_entre_intervalo(intervalo),
            random_entre_intervalo(intervalo),
        )
    }

    pub fn aleatorio_en_emisferio(normal: &Self) -> Self {
        let en_esfera_unitaria = Self::normalizado_random();

        if Vec3::punto(&en_esfera_unitaria, normal) > 0.0 {
            return en_esfera_unitaria;
        } else {
            return -en_esfera_unitaria;
        }
    }

    pub fn aleatorio_en_disco() -> Self {
        loop {
            let punto = Self::new(random_f64_entre(-1.0, 1.0), random_f64_entre(-1.0, 1.0), 0.0);

            if punto.logitud_cuadrada() < 1.0 {
                return punto;
            }
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, otro: Self) -> Self::Output {
        Self::new(
            self.x() + otro.x(),
            self.y() + otro.y(),
            self.z() + otro.z(),
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, otro: Self) -> Self::Output {
        Self::new(
            self.x() - otro.x(),
            self.y() - otro.y(),
            self.z() - otro.z(),
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, escalar: f64) -> Self::Output {
        Self::new(self.x() * escalar, self.y() * escalar, self.z() * escalar)
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3; 
    fn mul(self, escalar: f64) -> Vec3 {
        Vec3::new(self.x() * escalar, self.y() * escalar, self.z() * escalar)
    }
}


impl Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, otro: Vec3) -> Self::Output {
        Self::new(
            self.x() * otro.x(),
            self.y() * otro.y(),
            self.z() * otro.z(),
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, escalar: f64) -> Self::Output {
        self * (1.0 / escalar)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    #[inline]
    fn mul(self, valor: Vec3) -> Vec3 {
        valor * self
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

pub type Point3 = Vec3;
