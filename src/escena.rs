pub mod cornell;
pub mod cuadrilateros;
pub mod final_1;
pub mod luz;
pub mod perlin;

// hacer sistema de escenas con lua

use std::sync::Arc;

use crate::golpe::{lista_golpeable::ListaGolpeable, nodo::Nodo};

pub fn new_escena_mundo(escena: impl Fn(ListaGolpeable) -> ListaGolpeable) -> ListaGolpeable {
    let mut mundo = ListaGolpeable::new();

    mundo = escena(mundo);

    ListaGolpeable::from(vec![Arc::new(Nodo::new_from_lista(&mut mundo))])
}
