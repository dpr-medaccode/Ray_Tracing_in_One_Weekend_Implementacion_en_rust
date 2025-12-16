use crate::Vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Rayo {
    origen: Point3,
    direccion: Vec3,
    tiempo: f64,
}

impl Rayo {
    //#[inline]
    pub fn new(origen: Point3, direccion: Vec3) -> Self {
        Self {
            origen,
            direccion,
            tiempo: 0.0,
        }
    }

    pub fn new_con_tiempo(origen: Point3, direccion: Vec3, tiempo: f64) -> Self {
        Self {
            origen,
            direccion,
            tiempo,
        }
    }

    pub fn origen(&self) -> Point3 {
        self.origen
    }

    pub fn direccion(&self) -> Vec3 {
        self.direccion
    }

    pub fn tiempo(&self) -> f64 {
        self.tiempo
    }

    pub fn en(&self, t: f64) -> Point3 {
        self.origen + t * self.direccion
    }
}
