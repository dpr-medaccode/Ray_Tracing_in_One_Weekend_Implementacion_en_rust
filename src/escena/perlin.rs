use std::sync::Arc;

use crate::{
    camara::Camara,
    color::CIELO,
    escena::new_escena_mundo,
    golpe::esfera::Esfera,
    material::{Material, difuso_lambertiano::DifusoLambertiano},
    output::Output,
    textura::perlin::Perlin,
    vec3::Vec3,
};

pub fn escena_perlin() -> Camara {
    let mundo = new_escena_mundo(|mut mundo| {
        let perlin = Arc::new(DifusoLambertiano::new_from_textura(Arc::new(Perlin::new(
            4.0,
        ))));

        mundo.push(Box::new(Esfera::new_estatica(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::clone(&perlin) as Arc<dyn Material>,
        )));

        mundo.push(Box::new(Esfera::new_estatica(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Arc::clone(&perlin) as Arc<dyn Material>,
        )));

        mundo
    });

    Camara::new(
        Arc::new(Output::new(16.0 / 9.0, 400)),
        mundo,
        20.0,
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        CIELO,
        50,
        100
    )
}
