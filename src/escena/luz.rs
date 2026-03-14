use std::sync::Arc;

use crate::{
    camara::Camara,
    color::NEGRO,
    escena::new_escena_mundo,
    golpe::{cuadrilatero::Cuadrilatero, esfera::Esfera},
    material::{
        Material, difuso_lambertiano::DifusoLambertiano, luz_difusa::LuzDifusa,
    },
    output::Output,
    textura::perlin::Perlin,
    vec3::Vec3,
};

pub fn escena_luz_1() -> Camara {
    let mundo = new_escena_mundo(|mut mundo| {
        let perlin = Arc::new(DifusoLambertiano::new_from_textura(Arc::new(Perlin::new(
            4.0,
        ))));

        mundo.push(Arc::new(Esfera::new_estatica(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Arc::clone(&perlin) as Arc<dyn Material>,
        )));

        mundo.push(Arc::new(Esfera::new_estatica(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::clone(&perlin) as Arc<dyn Material>,
        )));

        let luz = Arc::new(LuzDifusa::new_from_color(Vec3::new(4.0, 4.0, 4.0)));

        mundo.push(Arc::new(Cuadrilatero::new(
            Vec3::new(3.0, 1.0, -2.0),
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
            luz,
        )));

        mundo
    });

    Camara::new(
        Arc::new(Output::new(1.0, 400)),
        mundo,
        20.0,
        Vec3::new(26.0, 3.0, 6.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        NEGRO,
    )
}

pub fn escena_luz_2() -> Camara {
    let mundo = new_escena_mundo(|mut mundo| {
        let perlin = Arc::new(DifusoLambertiano::new_from_textura(Arc::new(Perlin::new(
            4.0,
        ))));

        mundo.push(Arc::new(Esfera::new_estatica(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Arc::clone(&perlin) as Arc<dyn Material>,
        )));

        mundo.push(Arc::new(Esfera::new_estatica(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::clone(&perlin) as Arc<dyn Material>,
        )));

        let luz = Arc::new(LuzDifusa::new_from_color(Vec3::new(4.0, 4.0, 4.0)));

        mundo.push(Arc::new(Cuadrilatero::new(
            Vec3::new(3.0, 1.0, -2.0),
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
            Arc::clone(&luz) as Arc<dyn Material>,
        )));

        mundo.push(Arc::new(Esfera::new_estatica(
            Vec3::new(0.0, 7.0, 0.0),
            2.0,
            Arc::clone(&luz) as Arc<dyn Material>,
        )));

        mundo
    });

    Camara::new(
        Arc::new(Output::new(1.0, 400)),
        mundo,
        20.0,
        Vec3::new(26.0, 3.0, 6.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        NEGRO,
    )
}
