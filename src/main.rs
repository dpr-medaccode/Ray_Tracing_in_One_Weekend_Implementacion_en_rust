use std::{fs::File, io::BufWriter, sync::Arc};

mod caja;
mod camara;
mod color;
mod esfera;
mod golpe;
mod intervalo;
mod material;
mod nodo;
mod output;
mod rayo;
mod textura;
mod util;
mod vec3;

use crate::{
    color::{BLANCO, NEGRO},
    esfera::Esfera,
    golpe::lista_golpeable::ListaGolpeable,
    material::{dielectrico::Dielectrico, difuso_lambertiano::DifusoLambertiano, metal::Metal},
    textura::{
        Textura, ajedrez::Ajedrez, color_solido::ColorSolido, perlin::Perlin,
        textura_imagen::TexturaImagen,
    },
    util::random_f64_entre,
    vec3::Vec3,
};

fn main() {
    let mut mundo = ListaGolpeable::new();

    mundo.push(Arc::new(Esfera::new_estatica(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(DifusoLambertiano::new(Arc::new(Perlin::new(4.0)))),
    )));

    mundo.push(Arc::new(Esfera::new_estatica(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(DifusoLambertiano::new(Arc::new(Perlin::new(4.0)))),
    )));

    

    // Suelo
    /*let material_suelo: Arc<dyn material::Material> = Arc::new(DifusoLambertiano::new(Arc::new(
        Ajedrez::new_from_colores(0.32, BLANCO, NEGRO),
    )));

    mundo.push(Arc::new(Esfera::new_estatica(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&material_suelo),
    )));

    /*/ Función auxiliar para crear difuso aleatorio
    fn random_difuso(scale: f64) -> Arc<dyn material::Material> {
        let choose_tex: f64 = rand::random();
        let tex: Arc<dyn Textura> = if choose_tex < 0.5 {
            Arc::new(Color_solido::new(Vec3::ramdon() * Vec3::ramdon()))
        } else {
            Arc::new(Ajedrez::new(
                scale,
                Arc::new(Color_solido::new(Vec3::ramdon())),
                Arc::new(Color_solido::new(Vec3::ramdon())),
            ))
        };
        Arc::new(DifusoLambertiano::new(tex))
    }*/

    // Esferas pequeñas aleatorias
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rand::random();
            let center = Vec3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).logitud() > 0.9 {
                if choose_mat < 0.8 {
                    // Difuso
                    let tex: Arc<dyn Textura> = if rand::random::<f64>() < 0.5 {
                        // Color sólido
                        Arc::new(ColorSolido::new(Vec3::ramdon() * Vec3::ramdon()))
                    } else {
                        // Patrón ajedrez
                        Arc::new(Ajedrez::new(
                            0.09,
                            Arc::new(ColorSolido::new(Vec3::ramdon())),
                            Arc::new(ColorSolido::new(Vec3::ramdon())),
                        ))
                    };

                    let material: Arc<dyn material::Material> =
                        Arc::new(DifusoLambertiano::new(tex));

                    // Aleatoriamente estática o en movimiento
                    if rand::random::<f64>() < 0.5 {
                        let center2 = center + Vec3::new(0.0, random_f64_entre(0.0, 0.5), 0.0);
                        mundo.push(Arc::new(Esfera::new_en_movimiento(
                            center, center2, 0.2, material,
                        )));
                    } else {
                        mundo.push(Arc::new(Esfera::new_estatica(center, 0.2, material)));
                    }
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::random_entre(0.5, 1.0);
                    let fuzz = 1.0;
                    let material: Arc<dyn material::Material> = Arc::new(Metal::new(albedo, fuzz));
                    mundo.push(Arc::new(Esfera::new_estatica(center, 0.2, material)));
                } else {
                    // Vidrio
                    let material: Arc<dyn material::Material> = Arc::new(Dielectrico::new(1.5));
                    mundo.push(Arc::new(Esfera::new_estatica(center, 0.2, material)));
                }
            }
        }
    }

    // Esferas grandes
    let material1: Arc<dyn material::Material> =
        Arc::new(DifusoLambertiano::new_from_color(Vec3::new(0.4, 0.2, 0.1)));

    mundo.push(Arc::new(Esfera::new_estatica(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material3: Arc<dyn material::Material> =
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.5));

    mundo.push(Arc::new(Esfera::new_estatica(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let material2: Arc<dyn material::Material> = Arc::new(DifusoLambertiano::new(Arc::new(
        TexturaImagen::new("./public/jupiter.jpg"),
    )));

    mundo.push(Arc::new(Esfera::new_estatica(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material2,
    )));*/

    mundo = ListaGolpeable::from(vec![Arc::new(nodo::Nodo::new_from_lista(&mut mundo))]);

    // Cámara
    let de_donde_mira = Vec3::new(13.0, 2.0, 3.0);
    let hacia_donde_mira = Vec3::new(0.0, 0.0, 0.0);
    let hacia_arriba = Vec3::new(0.0, 1.0, 0.0);

    let imagen = Arc::new(output::Output::new(16.0 / 9.0, 400));

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
