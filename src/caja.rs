#![allow(dead_code)]
use std::ops::Add;

use crate::{intervalo::Intervalo, rayo::Rayo, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Caja {
    pub x: Intervalo,
    pub y: Intervalo,
    pub z: Intervalo,
}

impl Caja {
    pub fn new(x: Intervalo, y: Intervalo, z: Intervalo) -> Self {
        let mut caja = Caja { x, y, z };

        caja.expandir_minimos();

        caja
    }

    pub fn new_from_vec3(a: Vec3, b: Vec3) -> Self {
        let mut caja = Caja {
            x: if a.x() <= b.x() {
                Intervalo::new(a.x(), b.x())
            } else {
                Intervalo::new(b.x(), a.x())
            },
            y: if a.y() <= b.y() {
                Intervalo::new(a.y(), b.y())
            } else {
                Intervalo::new(b.y(), a.y())
            },
            z: if a.z() <= b.z() {
                Intervalo::new(a.z(), b.z())
            } else {
                Intervalo::new(b.z(), a.z())
            },
        };

        caja.expandir_minimos();

        caja
    }

    pub fn new_from_cajas(a: Caja, b: Caja) -> Self {
        Self {
            x: Intervalo::new_from_intervalos(a.x, b.x),
            y: Intervalo::new_from_intervalos(a.y, b.y),
            z: Intervalo::new_from_intervalos(a.z, b.z),
        }
    }

    fn expandir_minimos(&mut self) {
        let minimo = 0.0001;

        if self.x.tamanno() < minimo {
            self.x = self.x.expandir(minimo);
        }
        if self.y.tamanno() < minimo {
            self.y = self.y.expandir(minimo);
        }
        if self.z.tamanno() < minimo {
            self.z = self.z.expandir(minimo);
        }
    }

    pub fn rayo_golpea_caja(&self, rayo: &Rayo, intervalo: Intervalo) -> Option<Intervalo> {
        let mut t = intervalo;

        for eje in Eje::new_set() {
            let (eje_intervalo, origen, direccion) = match eje {
                Eje::X => (self.x, rayo.origen().x(), rayo.direccion().x()),
                Eje::Y => (self.y, rayo.origen().y(), rayo.direccion().y()),
                Eje::Z => (self.z, rayo.origen().z(), rayo.direccion().z()),
            };

            let inv_d = 1.0 / direccion;

            let mut t0 = (eje_intervalo.minimo - origen) * inv_d;
            let mut t1 = (eje_intervalo.maximo - origen) * inv_d;

            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t.minimo = t.minimo.max(t0);
            t.maximo = t.maximo.min(t1);

            if t.maximo <= t.minimo {
                return None;
            }
        }

        Some(t)
    }

    pub fn eje_mas_largo(&self) -> Eje {
        let tx = self.x.tamanno();
        let ty = self.y.tamanno();
        let tz = self.z.tamanno();

        if tx >= ty && tx >= tz {
            Eje::X
        } else if ty >= tz {
            Eje::Y
        } else {
            Eje::Z
        }
    }

    pub fn vacio() -> Self {
        Caja {
            x: Intervalo::vacio(),
            y: Intervalo::vacio(),
            z: Intervalo::vacio(),
        }
    }

    pub fn universo() -> Self {
        Caja {
            x: Intervalo::universo(),
            y: Intervalo::universo(),
            z: Intervalo::universo(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Eje {
    X,
    Y,
    Z,
}
impl Eje {

    pub fn new_set() -> [Eje; 3]{

        [Eje::X, Eje::Y, Eje::Z]
        
    }
    
}

impl Add<Vec3> for Caja {

    type Output = Caja;

    fn add(self, desplazo: Vec3) -> Caja {
        Caja {
            x: self.x + desplazo.x(),
            y: self.y + desplazo.y(),
            z: self.z + desplazo.z(),
        }
    }
}

impl Add<Caja> for Vec3 {
    type Output = Caja;

    fn add(self, caja: Caja) -> Caja {
        caja + self
    }

}
