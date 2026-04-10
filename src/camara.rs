use std::{
    fs::File,
    io::{BufWriter, Read, Write},
    sync::Arc,
    time::Instant,
};

use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    color::{Color, color_rayo, escribir_color},
    golpe::{lista_golpeable::ListaGolpeable, objeto::Objeto},
    output::Output,
    rayo::Rayo,
    vec3::Vec3,
};

/*const PROFUNDIDAD: i32 = 50;

const MUESTRA_POR_PIXEL: i32 = 100; // anti-aliasing
const ESCALA_PIXEL_MUESTRA: f64 = 1.0 / MUESTRA_POR_PIXEL as f64; */

pub struct Camara {
    output: Arc<Output>,
    mundo: Objeto,

    origen: Vec3,

    delta_pixel_x: Vec3,
    delta_pixel_y: Vec3,

    pixel_00_lugar: Vec3,

    //fov_vertical: f64,
    disco_desenfoque_u: Vec3,
    disco_desenfoque_v: Vec3,

    angulo_desenfoque: f64,
    fondo: Color,

    profundidad: i32,
    muestra_por_pixel: i32,
    escala_pixel_muestra: f64,
}

impl Camara {
    pub fn new(
        imagen: Arc<Output>,
        mundo: Objeto,
        fov_vertical: f64,
        de_donde_mira: Vec3,
        hacia_donde_mira: Vec3,
        hacia_arriba: Vec3,
        angulo_desenfoque: f64,
        distancia_foco: f64,
        fondo: Color,
        profundidad: i32,
        muestra_por_pixel: i32,
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

        let escala_pixel_muestra = 1.0 / muestra_por_pixel as f64;

        Self {
            output: imagen,
            mundo,
            origen,
            delta_pixel_x,
            delta_pixel_y,
            pixel_00_lugar,
            disco_desenfoque_u,
            disco_desenfoque_v,
            angulo_desenfoque,
            fondo,
            profundidad,
            muestra_por_pixel,
            escala_pixel_muestra,
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

        let mut rng = rand::rng();
        let tiempo = rng.random::<f64>();

        Rayo::new_con_tiempo(origen, direccion, tiempo)
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
     
        let alto = self.output.alto();
        let ancho = self.output.ancho();
        let inicio_total = Instant::now();

        writer
            .write_all(self.output.encabezado_imagen().as_bytes())
            .unwrap();

        (0..alto).into_par_iter().for_each(|j| {
            
            let nombre_archivo = format!("fila_{}.temp", j);
            let archivo = File::create(&nombre_archivo).unwrap();
            let mut writer_bloque = BufWriter::new(archivo);

            if mostrar_lineas {
                eprintln!("Líneas restantes: {}", alto - j);
            }

            for i in 0..ancho {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..self.muestra_por_pixel {
                    let rayo = self.rayo_por_pixel(i, j);
                    pixel_color = pixel_color
                        + color_rayo(&rayo, &self.mundo, self.profundidad, self.fondo);
                }
                pixel_color = pixel_color * self.escala_pixel_muestra;
                writer_bloque
                    .write_all(escribir_color(&pixel_color).as_bytes())
                    .unwrap();
            }

            if mostrar_lineas {
                eprintln!("Fila {} terminada", j);
            }
        });

        for j in 0..alto {
            let nombre_archivo = format!("fila_{}.temp", j);
            let mut archivo_bloque = File::open(&nombre_archivo).unwrap();
            let mut buffer = Vec::new();
            archivo_bloque.read_to_end(&mut buffer).unwrap();
            writer.write_all(&buffer).unwrap();
            std::fs::remove_file(&nombre_archivo).unwrap();
        }

        if medir_tiempo {
            let duracion = inicio_total.elapsed();
            eprintln!("Render finalizado en {:.2?}", duracion);
        }
    }
}
