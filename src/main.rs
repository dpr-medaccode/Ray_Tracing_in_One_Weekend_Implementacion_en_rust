mod Camara;
mod Color;
mod Esfera;
mod Golpe;
mod Imagen;
mod Intervalo;
mod Material;
mod Rayo;
mod Vec3;
mod util;

use std::{fs::File, io::BufWriter, sync::Arc};

use crate::{
    Color::{INDIGO, NARANJA, ROJO, VERDE, atenuar_color, mezclar_colores},
    Golpe::Lista_golpeable::ListaGolpeable,
    Material::{Dielectrico::Dielectrico, Difuso_lambertiano::DifusoLambertiano, Metal::Metal},
};

fn main() {
    // Lista de objetos
    let mut mundo = ListaGolpeable::new();

    // Suelo
    let material_suelo: Arc<dyn Material::Material> =
        Arc::new(DifusoLambertiano::new(Vec3::Vec3::new(0.5, 0.5, 0.5)));

    mundo.push(Arc::new(Esfera::Esfera::new(
        Vec3::Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&material_suelo),
    )));

    // Esferas pequeñas aleatorias
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rand::random();
            let center = Vec3::Vec3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Vec3::Vec3::new(4.0, 0.2, 0.0)).logitud() > 0.9 {
                if choose_mat < 0.8 {
                    // Difuso
                    let albedo = Vec3::Vec3::ramdon() * Vec3::Vec3::ramdon();
                    let material: Arc<dyn Material::Material> =
                        Arc::new(DifusoLambertiano::new(albedo));
                    mundo.push(Arc::new(Esfera::Esfera::new(center, 0.2, material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::Vec3::random_entre(0.5, 1.0);
                    let fuzz = 1.0;
                    let material: Arc<dyn Material::Material> = Arc::new(Metal::new(albedo, fuzz));
                    mundo.push(Arc::new(Esfera::Esfera::new(center, 0.2, material)));
                } else {
                    // Vidrio
                    let material: Arc<dyn Material::Material> = Arc::new(Dielectrico::new(1.5));
                    mundo.push(Arc::new(Esfera::Esfera::new(center, 0.2, material)));
                }
            }
        }
    }

    // Esferas grandes
    let material1: Arc<dyn Material::Material> = Arc::new(Dielectrico::new(1.5));
    mundo.push(Arc::new(Esfera::Esfera::new(
        Vec3::Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2: Arc<dyn Material::Material> = Arc::new(DifusoLambertiano::new(INDIGO));
    mundo.push(Arc::new(Esfera::Esfera::new(
        Vec3::Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3: Arc<dyn Material::Material> = Arc::new(Metal::new(
        atenuar_color(&mezclar_colores(&VERDE, &NARANJA)),
        0.5,
    ));
    mundo.push(Arc::new(Esfera::Esfera::new(
        Vec3::Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // Cámara
    let de_donde_mira = Vec3::Vec3::new(13.0, 2.0, 3.0);
    let hacia_donde_mira = Vec3::Vec3::new(0.0, 0.0, 0.0);
    let hacia_arriba = Vec3::Vec3::new(0.0, 1.0, 0.0);

    let imagen = Arc::new(Imagen::Imagen::new(16.0 / 9.0, 400));

    let camara = Camara::Camara::new(
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
