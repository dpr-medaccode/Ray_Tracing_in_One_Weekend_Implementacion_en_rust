use std::sync::Arc;

use crate::{
    Golpe::{Golpe, Golpeable::Golpeable},
    Intervalo::Intervalo,
    Rayo::Rayo,
};

pub struct Lista_golpeable {
    objetos: Vec<Arc<dyn Golpeable>>,
}

impl Lista_golpeable {
    pub fn new() -> Self {
        Self {
            objetos: Vec::new(),
        }
    }
 
    pub fn from(lista: Vec<Arc<dyn Golpeable>>) -> Self {
        Self { objetos: lista }
    }

    pub fn clear(&mut self) {
        self.objetos.clear();
    }

    pub fn push(&mut self, obj: Arc<dyn Golpeable>) {
        self.objetos.push(obj);
    }
}

impl Golpeable for Lista_golpeable {
    fn rayo_golpea(&self, rayo: &Rayo, intervalo: &Intervalo) -> Option<Golpe> {
        let mut golpe_cercano: Option<Golpe> = None;

        let mut lugar_mas_cercano = intervalo.maximo;

        for objeto in &self.objetos {

            let intervalo_temp = Intervalo::new(intervalo.minimo, lugar_mas_cercano);

            if let Some(golpe) = objeto.rayo_golpea(rayo, &intervalo_temp) {
                lugar_mas_cercano = golpe.distancia();
                golpe_cercano = Some(golpe);
            }
        }

        golpe_cercano
    }
}
