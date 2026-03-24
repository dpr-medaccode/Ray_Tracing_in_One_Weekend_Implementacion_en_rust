use std::sync::Arc;

use crate::{
    camara::Camara, color::CIELO, escena::new_escena_mundo, golpe::{cuadrilatero::Cuadrilatero, objeto::Objeto},
    material::difuso_lambertiano::DifusoLambertiano, output::Output, vec3::Vec3,
};

pub fn escena_cuadrilateros() -> Camara {
    let mundo = new_escena_mundo(|mut mundo| {
        let rojo_izquierda = Arc::new(DifusoLambertiano::new_from_color(Vec3::new(1.0, 0.2, 0.2)));
        let verde_fondo = Arc::new(DifusoLambertiano::new_from_color(Vec3::new(0.2, 1.0, 0.2)));
        let azul_derecha = Arc::new(DifusoLambertiano::new_from_color(Vec3::new(0.2, 0.2, 1.0)));
        let naranja_arriba = Arc::new(DifusoLambertiano::new_from_color(Vec3::new(1.0, 0.5, 0.0)));
        let cian_abajo = Arc::new(DifusoLambertiano::new_from_color(Vec3::new(0.2, 0.8, 0.8)));

        mundo.push(Box::new(Objeto::Cuadrilatero(Cuadrilatero::new(
            Vec3::new(-3.0, -2.0, 5.0),
            Vec3::new(0.0, 0.0, -4.0),
            Vec3::new(0.0, 4.0, 0.0),
            rojo_izquierda,
        ))));

        mundo.push(Box::new(Objeto::Cuadrilatero(Cuadrilatero::new(
            Vec3::new(-2.0, -2.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 4.0, 0.0),
            verde_fondo,
        ))));

        mundo.push(Box::new(Objeto::Cuadrilatero(Cuadrilatero::new(
            Vec3::new(3.0, -2.0, 1.0),
            Vec3::new(0.0, 0.0, 4.0),
            Vec3::new(0.0, 4.0, 0.0),
            azul_derecha,
        ))));

        mundo.push(Box::new(Objeto::Cuadrilatero(Cuadrilatero::new(
            Vec3::new(-2.0, 3.0, 1.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 4.0),
            naranja_arriba,
        ))));

        mundo.push(Box::new(Objeto::Cuadrilatero(Cuadrilatero::new(
            Vec3::new(-2.0, -3.0, 5.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -4.0),
            cian_abajo,
        ))));

        mundo
    });

    Camara::new(
        Arc::new(Output::new(1.0, 400)),
        Objeto::Lista(mundo),
        80.0,
        Vec3::new(0.0, 0.0, 9.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        CIELO,
        50,
        100
    )
}
