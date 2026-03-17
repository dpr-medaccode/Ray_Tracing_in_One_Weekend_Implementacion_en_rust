pub mod cornell;
pub mod cornell_humo;
pub mod cuadrilateros;
pub mod final_1;
pub mod final_2;
pub mod luz;
pub mod perlin;

// hacer sistema de escenas con lua

use crate::golpe::{lista_golpeable::ListaGolpeable, nodo::Nodo};

pub fn new_escena_mundo(escena: impl Fn(ListaGolpeable) -> ListaGolpeable) -> ListaGolpeable {
    let mut mundo = escena(ListaGolpeable::new());

    let objetos = std::mem::take(&mut mundo.objetos);

    ListaGolpeable::from(vec![Box::new(Nodo::new_from_lista(objetos))])

}
