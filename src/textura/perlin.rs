use crate::{
    textura::Textura,
    util::random_usize_entre,
    vec3::{Point3, Vec3},
};

const PUNTOS: usize = 256;

pub struct Perlin {
    aleatoriedad: [Vec3; PUNTOS],

    permutacion_x: [usize; PUNTOS],
    permutacion_y: [usize; PUNTOS],
    permutacion_z: [usize; PUNTOS],

    escala: f64,
}

impl Perlin {
    pub fn new(escala: f64) -> Self {
        let mut aleatoriedad = [Vec3::new(0.0, 0.0, 0.0); PUNTOS];
        for i in 0..PUNTOS {
            aleatoriedad[i] = Vec3::normalizar(&Vec3::random_entre(-1.0, 1.0));
        }

        Self {
            aleatoriedad,
            permutacion_x: Perlin::generar_permutacion(),
            permutacion_y: Perlin::generar_permutacion(),
            permutacion_z: Perlin::generar_permutacion(),
            escala,
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
        let u = lugar.x() - lugar.x().floor();
        let v = lugar.y() - lugar.y().floor();
        let w: f64 = lugar.z() - lugar.z().floor();

        let i = lugar.x().floor() as i32;
        let j = lugar.y().floor() as i32;
        let k = lugar.z().floor() as i32;

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let xi = ((i + di as i32) & 255) as usize;
                    let yj = ((j + dj as i32) & 255) as usize;
                    let zk = ((k + dk as i32) & 255) as usize;

                    let idx =
                        self.permutacion_x[xi] ^ self.permutacion_y[yj] ^ self.permutacion_z[zk];

                    c[di][dj][dk] = self.aleatoriedad[idx];
                }
            }
        }

        Perlin::interpolacion(c, u, v, w)
    }

    fn turbulencia(&self, lugar: &Point3, profundidad: i32) -> f64 {
        let mut acumuldor = 0.0;
        let mut lugar_temporal = lugar.clone();
        let mut peso = 1.0;

        for i in 0..profundidad  {

            acumuldor += peso * self.ruido(&lugar_temporal);

            peso *= 0.5;

            lugar_temporal = lugar_temporal * 2.0;
            
        }

        return acumuldor.abs();
        
    }

    fn interpolacion(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut acumulador = 0.0;

        for i in 0..2 {
            let fi = i as f64;
            for j in 0..2 {
                let fj = j as f64;
                for k in 0..2 {
                    let fk = k as f64;

                    let peso = Vec3::new(u - i as f64, v - j as f64, w - k as f64);

                    let punto = Vec3::punto(&c[i][j][k], &peso);

                    let mezcla = (fi * uu + (1.0 - fi) * (1.0 - uu))
                        * (fj * vv + (1.0 - fj) * (1.0 - vv))
                        * (fk * ww + (1.0 - fk) * (1.0 - ww));

                    acumulador += mezcla * punto;
                }
            }
        }

        acumulador
    }
}

impl Textura for Perlin {
    fn valor(
        &self,
        _textura_horizontal: f64,
        _textura_vertical: f64,
        lugar: &Point3,
    ) -> crate::color::Color {

       Vec3::new(0.5, 0.5, 0.5) * (1.0 + (self.escala * lugar.z() + 10.0 * self.turbulencia(lugar, 7)).sin())

    }
}
