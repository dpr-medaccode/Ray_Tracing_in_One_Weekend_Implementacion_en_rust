#[derive(Debug, Clone, Copy)]

pub struct Intervalo {
    pub minimo: f64,
    pub maximo: f64,
}

impl Intervalo {
    pub fn new(minimo: f64, maximo: f64) -> Self {
        Self { minimo, maximo }
    }

    pub fn new_from_intervalos(a: Intervalo, b: Intervalo) -> Self {
        Self {
            minimo: if a.minimo <= b.minimo {
                a.minimo
            } else {
                b.minimo
            },
            maximo: if a.maximo >= b.maximo {
                a.maximo
            } else {
                b.maximo
            },
        }
    }

    pub fn vacio() -> Self {
        Self {
            minimo: f64::INFINITY,
            maximo: f64::NEG_INFINITY,
        }
    }

    pub fn universo() -> Self {
        Self {
            minimo: f64::NEG_INFINITY,
            maximo: f64::INFINITY,
        }
    }

    pub fn tamanno(&self) -> f64 {
        self.maximo - self.minimo
    }

    pub fn contiene(&self, x: f64) -> bool {
        self.minimo <= x && x <= self.maximo
    }

    /// Estrictamente dentro
    pub fn rodea(&self, x: f64) -> bool {
        self.minimo < x && x < self.maximo
    }

    pub fn limitar(&self, x: f64) -> f64 {
        if x < self.minimo {
            self.minimo
        } else if x > self.maximo {
            self.maximo
        } else {
            x
        }
    }

    pub fn expandir(&self, delta: f64) -> Self {
        let pading = delta / 2.0;
        return Intervalo {
            minimo: self.minimo - pading,
            maximo: self.maximo + pading,
        };
    }
}
