#![allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Imagen {
    relacion_aspecto: f64,
    ancho: u32,
    alto: u32,
}

impl Imagen {
    pub fn new(relacion_aspecto: f64, ancho: u32) -> Self {
        let mut img = Self {
            relacion_aspecto,
            ancho,
            alto: 1,
        };

        img.recalcular_alto();

        img
    }

    pub fn relacion_aspecto(&self) -> f64 {
        self.relacion_aspecto
    }

    pub fn ancho(&self) -> u32 {
        self.ancho
    }

    pub fn alto(&self) -> u32 {
        self.alto
    }

    pub fn set_relacion_aspecto(&mut self, val: f64) {
        self.relacion_aspecto = val;
        self.recalcular_alto();
    }

    pub fn set_ancho(&mut self, val: u32) {
        self.ancho = val;
        self.recalcular_alto();
    }

    pub fn set_alto(&mut self, val: u32) {
        self.alto = val.max(1);
        self.recalcular_ancho();
    }

    fn recalcular_alto(&mut self) {
        let h = (self.ancho as f64 / self.relacion_aspecto).floor() as i64;
        self.alto = h.max(1) as u32;
    }

    fn recalcular_ancho(&mut self) {
        let w = (self.alto as f64 * self.relacion_aspecto).floor() as i64;
        self.ancho = w.max(1) as u32;
    }

    pub fn encabezado_imagen(&self) -> String {
        format!("P3\n{} {}\n255\n", self.ancho, self.alto)
    }
}
