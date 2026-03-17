use std::{fs::File, io::BufWriter};

use crate::escena::{cornell::escena_caja_cornell};

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

    let camara = escena_caja_cornell();

    let f = File::create("out/imagen.ppm").unwrap();
    let mut writer = BufWriter::new(f);

    camara.render(&mut writer, true, true);
    
}