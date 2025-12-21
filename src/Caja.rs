use crate::{Golpe::Golpeable::Golpeable, Intervalo::Intervalo, Rayo::Rayo, Vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Caja {
    x: Intervalo,
    y: Intervalo,
    z: Intervalo,
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

    pub fn rayo_golpea(&self, rayo: &Rayo, intervalo: &mut Intervalo) -> bool {
        let ejes = [self.x, self.y, self.z];

        for (i, eje) in ejes.iter().enumerate() {
            let direccion_inversa = 1.0 / rayo.direccion()[i];

            let mut entrada = (eje.minimo - rayo.origen()[i]) * direccion_inversa;
            let mut salida = (eje.maximo - rayo.origen()[i]) * direccion_inversa;

            // Asegurarse de que t0 <= t1
            if entrada > salida {
                std::mem::swap(&mut entrada, &mut salida);
            }

            // Actualizar el intervalo
            if entrada > intervalo.minimo {
                intervalo.minimo = entrada;
            }
            if salida < intervalo.maximo {
                intervalo.maximo = salida;
            }

            // Si el intervalo se vuelve inválido, el rayo no golpea la caja
            if intervalo.maximo <= intervalo.minimo {
                return false;
            }
        }

        true
    }
}
