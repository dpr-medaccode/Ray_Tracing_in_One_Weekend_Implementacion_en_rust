use std::sync::Arc;

use crate::{
    camara::Camara,
    color::NEGRO,
    escena::new_escena_mundo,
    golpe::{
        constante_media::ConstanteMedia, cuadrilatero::Cuadrilatero, esfera::Esfera, golpeable::Golpeable, lista_golpeable::ListaGolpeable, nodo::Nodo, rotar_y::RotarY, trasladar::Trasladar
    },
    material::{
        Material, dielectrico::Dielectrico, difuso_lambertiano::DifusoLambertiano,
        luz_difusa::LuzDifusa, metal::Metal,
    },
    output::Output,
    textura::{perlin::Perlin, textura_imagen::TexturaImagen},
    util::ramdom_f64,
    vec3::Vec3,
};

pub fn escena_final_2() -> Camara {
    let mundo = new_escena_mundo(|mut mundo| {
        let mut boxes1 = ListaGolpeable::new();
        let ground = Arc::new(DifusoLambertiano::new_from_color(Vec3::new(
            0.48, 0.83, 0.53,
        )));
        let boxes_per_side = 20;
        let w = 100.0;

        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let x0 = -1000.0 + i as f64 * w;
                let z0 = -1000.0 + j as f64 * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let z1 = z0 + w;
                let y1 = ramdom_f64() * 100.0 + 1.0; // [1,101)

                let caja = Cuadrilatero::new_cubo(
                    Vec3::new(x0, y0, z0),
                    Vec3::new(x1, y1, z1),
                    Arc::clone(&ground) as Arc<dyn Material>,
                );
                boxes1.push(Arc::new(caja));
            }
        }

        let mut boxes1_bvh = Nodo::new_from_lista(&mut boxes1);
        
        mundo.push(Arc::new(boxes1_bvh));

        // --- Luz ---
        let light = Arc::new(LuzDifusa::new_from_color(Vec3::new(7.0, 7.0, 7.0)));
        let luz_cuad = Cuadrilatero::new(
            Vec3::new(123.0, 554.0, 147.0),
            Vec3::new(300.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 265.0),
            light,
        );
        mundo.push(Arc::new(luz_cuad));

        // --- Esferas principales ---
        let center1 = Vec3::new(400.0, 400.0, 200.0);
        let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
        let sphere_material = Arc::new(DifusoLambertiano::new_from_color(Vec3::new(0.7, 0.3, 0.1)));
        let sphere = Esfera::new_en_movimiento(
            center1,
            center2,
            50.0,
            Arc::clone(&sphere_material) as Arc<dyn Material>,
        );

        mundo.push(Arc::new(sphere));

        mundo.push(Arc::new(Esfera::new_estatica(
            Vec3::new(260.0, 150.0, 45.0),
            50.0,
            Arc::new(Dielectrico::new(1.5)),
        )));
        mundo.push(Arc::new(Esfera::new_estatica(
            Vec3::new(0.0, 150.0, 145.0),
            50.0,
            Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)),
        )));

        // --- Medios ---
        let boundary1 = Arc::new(Esfera::new_estatica(
            Vec3::new(360.0, 150.0, 145.0),
            70.0,
            Arc::new(Dielectrico::new(1.5)),
        ));
        mundo.push(Arc::clone(&boundary1) as Arc<dyn Golpeable>);
        mundo.push(Arc::new(ConstanteMedia::new_from_color(
            boundary1,
            0.2,
            Vec3::new(0.2, 0.4, 0.9),
        )));

        let boundary2 = Arc::new(Esfera::new_estatica(
            Vec3::new(0.0, 0.0, 0.0),
            5000.0,
            Arc::new(Dielectrico::new(1.5)),
        ));
        mundo.push(Arc::new(ConstanteMedia::new_from_color(
            boundary2,
            0.0001,
            Vec3::new(1.0, 1.0, 1.0),
        )));

        // --- Esferas con textura ---
        let emat = Arc::new(DifusoLambertiano::new_from_textura(Arc::new(
            TexturaImagen::new("./public/tierra.jpg"),
        )));
        mundo.push(Arc::new(Esfera::new_estatica(
            Vec3::new(400.0, 200.0, 400.0),
            100.0,
            emat,
        )));

        let pertext = Arc::new(Perlin::new(0.2));
        mundo.push(Arc::new(Esfera::new_estatica(
            Vec3::new(220.0, 280.0, 300.0),
            80.0,
            Arc::new(DifusoLambertiano::new_from_textura(pertext)),
        )));

        let mut boxes2 = ListaGolpeable::new();
        let white = Arc::new(DifusoLambertiano::new_from_color(Vec3::new(
            0.73, 0.73, 0.73,
        )));
        for _ in 0..1000 {
            let pos = Vec3::random_entre(0.0, 165.0);
            boxes2.push(Arc::new(Esfera::new_estatica(
                pos,
                10.0,
                Arc::clone(&white) as Arc<dyn Material>,
            )));
        }

        let bvh_boxes2 = Nodo::new_from_lista(&mut boxes2);
        let bvh_rotated = RotarY::new(Arc::new(bvh_boxes2), 15.0);
        let bvh_translated = Trasladar::new(Arc::new(bvh_rotated), Vec3::new(-100.0, 270.0, 395.0));
        mundo.push(Arc::new(bvh_translated));

        

        mundo
    });

    Camara::new(
        Arc::new(Output::new(1.0, 800)),
        mundo,
        40.0,
        Vec3::new(478.0, 278.0, -600.0),
        Vec3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        NEGRO,
        //40,
        20,
        //10000,
        1000
    )
}
