use crate::Vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Rayo {
    origen: Point3,
    direccion: Vec3,
}

impl Rayo {
    //#[inline]
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origen: origin, direccion: direction }
    }

    //#[inline]
    pub fn origen(&self) -> Point3 {
        self.origen
    }
    //#[inline]
    pub fn direccion(&self) -> Vec3 {
        self.direccion
    }

    //#[inline]
    pub fn en(&self, t: f64) -> Point3 {
        self.origen + t * self.direccion
    }
}
