use std::sync::Arc;

use crate::{
    camara::Camara,
    color::NEGRO,
    escena::new_escena_mundo,
    golpe::cuadrilatero::Cuadrilatero,
    material::{Material, difuso_lambertiano::DifusoLambertiano, luz_difusa::LuzDifusa},
    output::Output,
    vec3::Vec3,
};

pub fn escena_caja_cornell() -> Camara {
    let mundo = new_escena_mundo(|mut mundo| {
        let rojo = Arc::new(DifusoLambertiano::new_from_color(Vec3::new(
            0.65, 0.05, 0.05,
        )));
        let blanco = Arc::new(DifusoLambertiano::new_from_color(Vec3::new(
            0.73, 0.73, 0.73,
        )));
        let verde = Arc::new(DifusoLambertiano::new_from_color(Vec3::new(
            0.12, 0.45, 0.15,
        )));
        let luz = Arc::new(LuzDifusa::new_from_color(Vec3::new(15.0, 15.0, 15.0)));

        mundo.push(Arc::new(Cuadrilatero::new(
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            verde,
        )));

        mundo.push(Arc::new(Cuadrilatero::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            rojo,
        )));

        mundo.push(Arc::new(Cuadrilatero::new(
            Vec3::new(343.0, 554.0, 332.0),
            Vec3::new(-130.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -105.0),
            luz,
        )));

        mundo.push(Arc::new(Cuadrilatero::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            Arc::clone(&blanco) as Arc<dyn Material>,
        )));

        mundo.push(Arc::new(Cuadrilatero::new(
            Vec3::new(555.0, 555.0, 555.0),
            Vec3::new(-555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -555.0),
            Arc::clone(&blanco) as Arc<dyn Material>,
        )));

        mundo.push(Arc::new(Cuadrilatero::new(
            Vec3::new(0.0, 0.0, 555.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Arc::clone(&blanco) as Arc<dyn Material>,
        )));

        Cuadrilatero::new_caja(
            Vec3::new(130.0, 0.0, 65.0),
            Vec3::new(295.0, 165.0, 230.0),
            Arc::clone(&blanco) as Arc<dyn Material>,
            &mut mundo,
        );

        Cuadrilatero::new_caja(
            Vec3::new(265.0, 0.0, 295.0),
            Vec3::new(430.0, 330.0, 460.0),
            Arc::clone(&blanco) as Arc<dyn Material>,
            &mut mundo,
        );

        mundo
    });

    Camara::new(
        Arc::new(Output::new(1.0, 600)),
        mundo,
        40.0,
        Vec3::new(278.0, 278.0, -800.0),
        Vec3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        NEGRO,
    )
}
