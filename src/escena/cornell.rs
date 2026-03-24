use std::sync::Arc;

use crate::{
    camara::Camara,
    color::NEGRO,
    escena::new_escena_mundo,
    golpe::{cuadrilatero::Cuadrilatero, objeto::Objeto, rotar_y::RotarY, trasladar::Trasladar},
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

        mundo.push(Box::new(Objeto::Cuadrilatero(Cuadrilatero::new(
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            verde,
        ))));

        mundo.push(Box::new(Objeto::Cuadrilatero(Cuadrilatero::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            rojo,
        ))));

        mundo.push(Box::new(Objeto::Cuadrilatero(Cuadrilatero::new(
            Vec3::new(343.0, 554.0, 332.0),
            Vec3::new(-130.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -105.0),
            luz,
        ))));

        mundo.push(Box::new(Objeto::Cuadrilatero(Cuadrilatero::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            Arc::clone(&blanco) as Arc<dyn Material>,
        ))));

        mundo.push(Box::new(Objeto::Cuadrilatero(Cuadrilatero::new(
            Vec3::new(555.0, 555.0, 555.0),
            Vec3::new(-555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -555.0),
            Arc::clone(&blanco) as Arc<dyn Material>,
        ))));

        mundo.push(Box::new(Objeto::Cuadrilatero(Cuadrilatero::new(
            Vec3::new(0.0, 0.0, 555.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Arc::clone(&blanco) as Arc<dyn Material>,
        ))));

        let caja1 = Cuadrilatero::new_cubo(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 330.0, 165.0),
            Arc::clone(&blanco) as Arc<dyn Material>,
        );
        let caja1 = RotarY::new(Box::new(Objeto::Lista(caja1)), 15.0);
        let caja1 = Trasladar::new(Box::new(Objeto::RotarY(caja1)), Vec3::new(265.0, 0.0, 295.0));
        mundo.push(Box::new(Objeto::Trasladar(caja1)));

        let caja2 = Cuadrilatero::new_cubo(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 165.0, 165.0),
            Arc::clone(&blanco) as Arc<dyn Material>,
        );
        let caja2 = RotarY::new(Box::new(Objeto::Lista(caja2)), -18.0);
        let caja2 = Trasladar::new(Box::new(Objeto::RotarY(caja2)), Vec3::new(130.0, 0.0, 65.0));
        mundo.push(Box::new(Objeto::Trasladar(caja2)));

        mundo
    });

    Camara::new(
        Arc::new(Output::new(1.0, 600)),
        Objeto::Lista(mundo),
        40.0,
        Vec3::new(278.0, 278.0, -800.0),
        Vec3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        NEGRO,
        100,
        200
    )
}
