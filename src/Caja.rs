use crate::{Intervalo::Intervalo, Rayo::Rayo, Vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Caja {
    pub x: Intervalo,
    pub y: Intervalo,
    pub z: Intervalo,
}

impl Caja {
    pub fn new_vacio() -> Self {
        Caja {
            x: Intervalo {
                minimo: 0.0,
                maximo: 0.0,
            },

            y: Intervalo {
                minimo: 0.0,
                maximo: 0.0,
            },

            z: Intervalo {
                minimo: 0.0,
                maximo: 0.0,
            },
        }
    }

    pub fn new(a: Vec3, b: Vec3) -> Self {
        let x = if a.x() <= b.x() {
            Intervalo::new(a.x(), b.x())
        } else {
            Intervalo::new(b.x(), a.x())
        };

        let y = if a.y() <= b.y() {
            Intervalo::new(a.y(), b.y())
        } else {
            Intervalo::new(b.y(), a.y())
        };

        let z = if a.z() <= b.z() {
            Intervalo::new(a.z(), b.z())
        } else {
            Intervalo::new(b.z(), a.z())
        };

        Caja { x, y, z }
    }

    pub fn new_from_cajas(a: Caja, b: Caja) -> Self {
        Self {
            x: Intervalo::new_from_intervalos(a.x, b.x),
            y: Intervalo::new_from_intervalos(a.y, b.y),
            z: Intervalo::new_from_intervalos(a.z, b.z),
        }
    }

    pub fn rayo_golpea_caja(&self, rayo: &Rayo, intervalo: Intervalo) -> Option<Intervalo> {
        let mut intervalo: Intervalo = intervalo.clone();

        let ejes = [self.x, self.y, self.z];

        for (i, eje) in ejes.iter().enumerate() {
            let inv_dir = 1.0 / rayo.direccion()[i];

            let mut t0 = (eje.minimo - rayo.origen()[i]) * inv_dir;
            let mut t1 = (eje.maximo - rayo.origen()[i]) * inv_dir;

            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }

            intervalo.minimo = intervalo.minimo.max(t0);
            intervalo.maximo = intervalo.maximo.min(t1);

            if intervalo.maximo <= intervalo.minimo {
                return None;
            }
        }

        Some(intervalo)
    }

    pub fn eje_mas_largo(&self) -> usize {
        let tx = self.x.tamanno();
        let ty = self.y.tamanno();
        let tz = self.z.tamanno();

        if tx >= ty && tx >= tz {
            0
        } else if ty >= tz {
            1
        } else {
            2
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
