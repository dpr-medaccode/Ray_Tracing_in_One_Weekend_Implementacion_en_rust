use std::sync::Arc;

use crate::{camara::Camara, color::NEGRO, escena::new_escena_mundo, golpe::{constante_media::ConstanteMedia, cuadrilatero::Cuadrilatero, rotar_y::RotarY, trasladar::Trasladar}, material::{Material, difuso_lambertiano::DifusoLambertiano, luz_difusa::LuzDifusa}, output::Output, vec3::Vec3};

pub fn escena_cornell_humo() -> Camara {
    let mundo = new_escena_mundo(|mut mundo| {
        // Materiales
        let rojo = Arc::new(DifusoLambertiano::new_from_color(Vec3::new(0.65, 0.05, 0.05)));
        let blanco = Arc::new(DifusoLambertiano::new_from_color(Vec3::new(0.73, 0.73, 0.73)));
        let verde = Arc::new(DifusoLambertiano::new_from_color(Vec3::new(0.12, 0.45, 0.15)));
        let luz = Arc::new(LuzDifusa::new_from_color(Vec3::new(7.0, 7.0, 7.0)));

        // Paredes
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
            Vec3::new(113.0, 554.0, 127.0),
            Vec3::new(330.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 305.0),
            luz,
        )));
        mundo.push(Arc::new(Cuadrilatero::new(
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            Arc::clone(&blanco) as Arc<dyn Material>,
        )));
        mundo.push(Arc::new(Cuadrilatero::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            Arc::clone(&blanco) as Arc<dyn Material>,
        )));
        mundo.push(Arc::new(Cuadrilatero::new(
            Vec3::new(0.0, 0.0, 555.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Arc::clone(&blanco) as Arc<dyn Material>,
        )));

        // Cajas
        let caja1 = Cuadrilatero::new_cubo(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 330.0, 165.0),
            Arc::clone(&blanco) as Arc<dyn Material>,
        );
        let caja1 = RotarY::new(Arc::new(caja1), 15.0);
        let caja1 = Trasladar::new(Arc::new(caja1), Vec3::new(265.0, 0.0, 295.0));

        let caja2 = Cuadrilatero::new_cubo(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 165.0, 165.0),
            Arc::clone(&blanco) as Arc<dyn Material>,
        );
        let caja2 = RotarY::new(Arc::new(caja2), -18.0);
        let caja2 = Trasladar::new(Arc::new(caja2), Vec3::new(130.0, 0.0, 65.0));

        // Humo
        let humo1 = ConstanteMedia::new_from_color(Arc::new(caja1), 0.01, Vec3::new(0.0, 0.0, 0.0));
        let humo2 = ConstanteMedia::new_from_color(Arc::new(caja2), 0.01, Vec3::new(1.0, 1.0, 1.0));

        mundo.push(Arc::new(humo1));
        mundo.push(Arc::new(humo2));

        mundo
    });

    // Cámara
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
        100,
        200,
    )
}