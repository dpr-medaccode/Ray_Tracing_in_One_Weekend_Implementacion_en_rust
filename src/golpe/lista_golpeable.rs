#![allow(dead_code)]

use crate::{
    caja::Caja,
    golpe::{Golpe, golpeable::Golpeable},
    intervalo::Intervalo,
    rayo::Rayo,
};

pub struct ListaGolpeable {
  pub objetos: Vec<Box<dyn Golpeable>>,
    caja: Caja,
}

impl ListaGolpeable {
    pub fn new() -> Self {
        Self {
            objetos: Vec::new(),
            caja: Caja::vacio(),
        }
    }

    pub fn objetos(&self) -> &Vec<Box<dyn Golpeable>> {
        &self.objetos
    }

    /*pub fn objetos_mut(&mut self) -> &mut Vec<Arc<dyn Golpeable>> {
        &mut self.objetos
    }*/

    pub fn from(lista: Vec<Box<dyn Golpeable>>) -> Self {
        let mut lista_golpeable = Self {
            objetos: Vec::new(),
            caja: Caja::vacio(),
        };

        for obj in lista {
            lista_golpeable.push(obj);
        }

        lista_golpeable
    }

    pub fn clear(&mut self) {
        self.objetos.clear();
        self.caja = Caja::vacio();
    }

    pub fn push(&mut self, obj: Box<dyn Golpeable>) {
        self.caja = Caja::new_from_cajas(self.caja, obj.caja());
        self.objetos.push(obj);
    }
}

impl Golpeable for ListaGolpeable {
    fn caja(&self) -> Caja {
        self.caja
    }

    fn rayo_golpea(&self, rayo: &Rayo, intervalo: Intervalo) -> Option<Golpe> {
        let mut golpe_cercano: Option<Golpe> = None;

        let mut lugar_mas_cercano = intervalo.maximo;

        for objeto in &self.objetos {
            let intervalo_temp = Intervalo::new(intervalo.minimo, lugar_mas_cercano);

            if let Some(golpe) = objeto.rayo_golpea(rayo, intervalo_temp) {
                lugar_mas_cercano = golpe.distancia();
                golpe_cercano = Some(golpe);
            }
        }

        golpe_cercano
    }
}
