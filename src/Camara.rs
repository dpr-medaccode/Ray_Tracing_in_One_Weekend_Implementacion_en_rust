use std::{io::Write, sync::Arc, time::Instant};

use crate::{
    Color::{color_rayo, escribir_color},
    Golpe::Lista_golpeable::ListaGolpeable,
    Imagen::Imagen,
    Rayo::Rayo,
    Vec3::Vec3,
};

const PROFUNDIDAD: i32 = 10; // cantidad de rayos
const MUESTRA_POR_PIXEL: i32 = 100; // anti-aliasing
const ESCALA_PIXEL_MUESTRA: f64 = 1.0 / MUESTRA_POR_PIXEL as f64;

pub struct Camara {
    imagen: Arc<Imagen>,
    mundo: ListaGolpeable,

    origen: Vec3,

    delta_pixel_x: Vec3,
    delta_pixel_y: Vec3,

    pixel_00_lugar: Vec3,

    fov_vertical: f64,

    disco_desenfoque_u: Vec3,
    disco_desenfoque_v: Vec3,

    angulo_desenfoque: f64,
}

impl Camara {
    pub fn new(
        imagen: Arc<Imagen>,
        mundo: ListaGolpeable,
        fov_vertical: f64,
        de_donde_mira: Vec3,
        hacia_donde_mira: Vec3,
        hacia_arriba: Vec3,
        angulo_desenfoque: f64,
        distancia_foco: f64,
    ) -> Self {
        // Convertir FOV a radianes
        let theta = fov_vertical.to_radians();
        let h = (theta / 2.0).tan();

        let origen = de_donde_mira;

        let atras_w = Vec3::normalizar(&(de_donde_mira - hacia_donde_mira));
        let derecha_u = Vec3::normalizar(&Vec3::cruz(&hacia_arriba, &atras_w));
        let arriba_v = Vec3::cruz(&atras_w, &derecha_u);

        // Dimensiones del viewport
        let viewport_altura = 2.0 * h * distancia_foco;
        let viewport_ancho = viewport_altura * imagen.ancho() as f64 / imagen.alto() as f64;

        // Vectores del viewport
        let viewport_u = derecha_u * viewport_ancho;
        let viewport_v = arriba_v * -viewport_altura;

        // Delta por pixel
        let delta_pixel_x = viewport_u / imagen.ancho() as f64;
        let delta_pixel_y = viewport_v / imagen.alto() as f64;

        // Esquina superior izquierda del viewport
        let viewport_superior_izq =
            origen - atras_w * distancia_foco - viewport_u / 2.0 - viewport_v / 2.0;

        // Centro del pixel (0,0)
        let pixel_00_lugar = viewport_superior_izq + delta_pixel_x * 0.5 + delta_pixel_y * 0.5;

        // Disco de desenfoque
        let radio_desenfoque = distancia_foco * (angulo_desenfoque / 2.0).to_radians().tan();
        let disco_desenfoque_u = derecha_u * radio_desenfoque;
        let disco_desenfoque_v = arriba_v * radio_desenfoque;

        Self {
            imagen,
            mundo,
            origen,
            delta_pixel_x,
            delta_pixel_y,
            pixel_00_lugar,
            fov_vertical,
            disco_desenfoque_u,
            disco_desenfoque_v,
            angulo_desenfoque,
        }
    }

    fn rayo_por_pixel(&self, i: u32, j: u32) -> Rayo {
        let compensacion = self.cuadrado_muestra();

        let pixel_sample = self.pixel_00_lugar
            + self.delta_pixel_x * (i as f64 + compensacion.x())
            + self.delta_pixel_y * (j as f64 + compensacion.y());

        let origen = if self.angulo_desenfoque <= 0.0 {
            self.origen
        } else {
            self.ejemplo_disco_desenfoque()
        };

        let direccion = pixel_sample - origen;

        Rayo::new(origen, direccion)
    }

    fn cuadrado_muestra(&self) -> Vec3 {
        Vec3::new(
            rand::random::<f64>() - 0.5,
            rand::random::<f64>() - 0.5,
            0.0,
        )
    }

    fn ejemplo_disco_desenfoque(&self) -> Vec3 {
        let punto = Vec3::aleatorio_en_disco();
        self.origen + self.disco_desenfoque_u * punto.x() + self.disco_desenfoque_v * punto.y()
    }

     pub fn render<W: Write>(&self, writer: &mut W, mostrar_lineas: bool, medir_tiempo: bool) {
  
        let inicio = Instant::now();

        // Escribir encabezado de imagen
        writer.write_all(self.imagen.encabezado_imagen().as_bytes()).unwrap();

        let alto = self.imagen.alto();
        let ancho = self.imagen.ancho();

        for j in 0..alto {
            if mostrar_lineas {
                eprintln!("Líneas restantes: {}", alto - j);
            }

            for i in 0..ancho {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

                for _ in 0..MUESTRA_POR_PIXEL {
                    let rayo = self.rayo_por_pixel(i, j);
                    pixel_color = pixel_color + color_rayo(&rayo, &self.mundo, PROFUNDIDAD);
                }

                pixel_color = pixel_color * ESCALA_PIXEL_MUESTRA;

                writer
                    .write_all(escribir_color(&pixel_color).as_bytes())
                    .unwrap();
            }
        }

        if medir_tiempo {
            let duracion = inicio.elapsed();
            eprintln!("Render finalizado en {:.2?}", duracion);
        }
    }
}
