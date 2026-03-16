use std::{fs::File, io::BufWriter};

use crate::escena::final_2::escena_final_2;

mod caja;
mod camara;
mod color;
pub mod escena;
mod golpe;
mod intervalo;
mod material;
mod output;
mod rayo;
mod textura;
mod util;
mod vec3;

fn main() {
    let camara = escena_final_2();

    let f = File::create("out/imagen.ppm").unwrap();
    let mut writer = BufWriter::new(f);

    camara.render(&mut writer, true, true);
}