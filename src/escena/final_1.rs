use std::sync::Arc;

use crate::{
    camara::Camara,
    color::{ATARDECER, BLANCO, NEGRO},
    escena::new_escena_mundo,
    golpe::{esfera::Esfera, objeto::Objeto},
    material::{
        Material, dielectrico::Dielectrico, difuso_lambertiano::DifusoLambertiano, metal::Metal,
    },
    output::Output,
    textura::{
        Textura, ajedrez::Ajedrez, color_solido::ColorSolido, textura_imagen::TexturaImagen,
    },
    util::random_f64_entre,
    vec3::Vec3,
};

pub fn escena_final_1() -> Camara {
    let mundo = new_escena_mundo(|mut mundo| {
        let material_suelo: Arc<dyn Material> = Arc::new(DifusoLambertiano::new_from_textura(
            Arc::new(Ajedrez::new_from_colores(0.32, BLANCO, NEGRO)),
        ));

        mundo.push(Box::new(Objeto::Esfera(Esfera::new_estatica(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::clone(&material_suelo),
        ))));

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
                        let tex: Arc<dyn Textura> = if rand::random::<f64>() < 0.5 {
                            Arc::new(ColorSolido::new(Vec3::ramdon() * Vec3::ramdon()))
                        } else {
                            Arc::new(Ajedrez::new(
                                0.09,
                                Arc::new(ColorSolido::new(Vec3::ramdon())),
                                Arc::new(ColorSolido::new(Vec3::ramdon())),
                            ))
                        };

                        let material: Arc<dyn Material> =
                            Arc::new(DifusoLambertiano::new_from_textura(tex));

                        if rand::random::<f64>() < 0.5 {
                            let center2 = center + Vec3::new(0.0, random_f64_entre(0.0, 0.5), 0.0);
                            mundo.push(Box::new(Objeto::Esfera(Esfera::new_en_movimiento(
                                center, center2, 0.2, material,
                            ))));
                        } else {
                            mundo.push(Box::new(Objeto::Esfera(Esfera::new_estatica(center, 0.2, material))));
                        }
                    } else if choose_mat < 0.95 {
                        let albedo = Vec3::random_entre(0.5, 1.0);
                        let fuzz = 1.0;
                        let material: Arc<dyn Material> = Arc::new(Metal::new(albedo, fuzz));
                        mundo.push(Box::new(Objeto::Esfera(Esfera::new_estatica(center, 0.2, material))));
                    } else {
                        let material: Arc<dyn Material> = Arc::new(Dielectrico::new(1.5));
                        mundo.push(Box::new(Objeto::Esfera(Esfera::new_estatica(center, 0.2, material))));
                    }
                }
            }
        }

        let material1: Arc<dyn Material> =
            Arc::new(DifusoLambertiano::new_from_color(Vec3::new(0.4, 0.2, 0.1)));

        mundo.push(Box::new(Objeto::Esfera(Esfera::new_estatica(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            material1,
        ))));

        let material3: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.5));

        mundo.push(Box::new(Objeto::Esfera(Esfera::new_estatica(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            material3,
        ))));

        let material2: Arc<dyn Material> = Arc::new(DifusoLambertiano::new_from_textura(Arc::new(
            TexturaImagen::new("./public/jupiter.jpg"),
        )));

        mundo.push(Box::new(Objeto::Esfera(Esfera::new_estatica(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            material2,
        ))));
        mundo
    });

    Camara::new(
        Arc::new(Output::new(16.0 / 9.0, 400)),
        Objeto::Lista(mundo),
        20.0,
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
        ATARDECER,
        50,
        100,
    )
}
