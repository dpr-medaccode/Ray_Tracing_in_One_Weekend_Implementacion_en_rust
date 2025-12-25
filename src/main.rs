mod caja;
mod camara;
mod color;
mod esfera;
mod golpe;
mod imagen;
mod intervalo;
mod material;
mod nodo;
mod rayo;
mod vec3;
mod util;

use std::{fs::File, io::BufWriter, sync::Arc};

use crate::{
    golpe::lista_golpeable::ListaGolpeable,
    material::{dielectrico::Dielectrico, difuso_lambertiano::DifusoLambertiano, metal::Metal},
    util::random_f64_entre,
};

fn main() {
    // Lista de objetos
    let mut mundo = ListaGolpeable::new();

    // Suelo
    let material_suelo: Arc<dyn material::Material> =
        Arc::new(DifusoLambertiano::new(vec3::Vec3::new(0.5, 0.5, 0.5)));

    mundo.push(Arc::new(esfera::Esfera::new_estatica(
        vec3::Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&material_suelo),
    )));

    // Esferas pequeñas aleatorias
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rand::random();
            let center = vec3::Vec3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - vec3::Vec3::new(4.0, 0.2, 0.0)).logitud() > 0.9 {
                if choose_mat < 0.8 {
                    // Difuso
                    let albedo = vec3::Vec3::ramdon() * vec3::Vec3::ramdon();
                    let material: Arc<dyn material::Material> =
                        Arc::new(DifusoLambertiano::new(albedo));
                    let centro_2 = center + vec3::Vec3::new(0.0, random_f64_entre(0.0, 0.5), 0.0);
                    mundo.push(Arc::new(esfera::Esfera::new_en_movimiento(
                        center, centro_2, 0.2, material,
                    )));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = vec3::Vec3::random_entre(0.5, 1.0);
                    let fuzz = 1.0;
                    let material: Arc<dyn material::Material> = Arc::new(Metal::new(albedo, fuzz));
                    mundo.push(Arc::new(esfera::Esfera::new_estatica(
                        center, 0.2, material,
                    )));
                } else {
                    // Vidrio
                    let material: Arc<dyn material::Material> = Arc::new(Dielectrico::new(1.5));
                    mundo.push(Arc::new(esfera::Esfera::new_estatica(
                        center, 0.2, material,
                    )));
                }
            }
        }
    }

    // Esferas grandes
    let material1: Arc<dyn material::Material> = Arc::new(Dielectrico::new(1.5));
    mundo.push(Arc::new(esfera::Esfera::new_estatica(
        vec3::Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2: Arc<dyn material::Material> =
        Arc::new(DifusoLambertiano::new(vec3::Vec3::new(0.4, 0.2, 0.1)));
    mundo.push(Arc::new(esfera::Esfera::new_estatica(
        vec3::Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3: Arc<dyn material::Material> =
        Arc::new(Metal::new(vec3::Vec3::new(0.7, 0.6, 0.5), 0.5));
    mundo.push(Arc::new(esfera::Esfera::new_estatica(
        vec3::Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

   
    mundo = ListaGolpeable::from(vec![Arc::new(nodo::Nodo::new_from_lista(&mut mundo))]);

    // Cámara
    let de_donde_mira = vec3::Vec3::new(13.0, 2.0, 3.0);
    let hacia_donde_mira = vec3::Vec3::new(0.0, 0.0, 0.0);
    let hacia_arriba = vec3::Vec3::new(0.0, 1.0, 0.0);

    let imagen = Arc::new(imagen::Imagen::new(16.0 / 9.0, 400));

    let camara = camara::Camara::new(
        imagen,
        mundo,
        20.0,
        de_donde_mira,
        hacia_donde_mira,
        hacia_arriba,
        0.6,
        10.0,
    );

    let f = File::create("out/imagen.ppm").unwrap();
    let mut writer = BufWriter::new(f);

    camara.render(&mut writer, true, true);
}
