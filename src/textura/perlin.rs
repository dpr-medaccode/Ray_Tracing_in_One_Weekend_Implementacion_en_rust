use crate::{
    textura::Textura,
    util::{ramdom_f64, random_usize_entre},
    vec3::{Point3, Vec3},
};

const PUNTOS: usize = 256;

pub struct Perlin {
    aleatoriedad: [f64; PUNTOS],

    permutacion_x: [usize; PUNTOS],
    permutacion_y: [usize; PUNTOS],
    permutacion_z: [usize; PUNTOS],
}

impl Perlin {
    pub fn new() -> Self {
        let mut aleatoriedad = [0.0; PUNTOS];
        for i in 0..PUNTOS {
            aleatoriedad[i] = ramdom_f64();
        }

        Self {
            aleatoriedad,
            permutacion_x: Perlin::generar_permutacion(),
            permutacion_y: Perlin::generar_permutacion(),
            permutacion_z: Perlin::generar_permutacion(),
        }
    }

    fn generar_permutacion() -> [usize; PUNTOS] {
        let mut permutacion = [0usize; PUNTOS];
        for i in 0..PUNTOS {
            permutacion[i] = i;
        }

        for i in (1..PUNTOS).rev() {
            let target = random_usize_entre(0, i);

            permutacion.swap(i, target);
        }

        permutacion
    }

    fn ruido(&self, lugar: &Point3) -> f64 {
        let i = ((4.0 * lugar.x()).floor() as i32 & 255) as usize;
        let j = ((4.0 * lugar.y()).floor() as i32 & 255) as usize;
        let k = ((4.0 * lugar.z()).floor() as i32 & 255) as usize;

        self.aleatoriedad[self.permutacion_x[i] ^ self.permutacion_y[j] ^ self.permutacion_z[k]]
    }
}

impl Textura for Perlin {
    fn valor(
        &self,
        _textura_horizontal: f64,
        _textura_vertical: f64,
        lugar: &Point3,
    ) -> crate::color::Color {
        Vec3::new(1.0, 1.0, 1.0) * self.ruido(lugar)
    }
}
