use std::sync::Arc;

use crate::{
    caja::Caja,
    golpe::{Golpe, golpeable::Golpeable},
    intervalo::Intervalo,
    material::Material,
    vec3::{Point3, Vec3},
};

pub struct Cuadrilatero {
    origen: Point3,
    u: Vec3,
    v: Vec3,
    material: Arc<dyn Material>,
    caja: Caja,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Cuadrilatero {
    pub fn new(origen: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let n = Vec3::cruz(&u, &v);
        let normal = Vec3::normalizar(&n);
        let d = Vec3::punto(&normal, &origen);
        let w = n / Vec3::punto(&n, &n);

        Cuadrilatero {
            origen,
            u,
            v,
            material,
            caja: Cuadrilatero::caja_delimitadora(origen, u, v),
            normal,
            d,
            w,
        }
    }

    fn caja_delimitadora(origen: Point3, u: Vec3, v: Vec3) -> Caja {
        let diagonal1 = Caja::new_from_vec3(origen, origen + u + v);

        let diagonal2 = Caja::new_from_vec3(origen + u, origen + v);

        Caja::new_from_cajas(diagonal1, diagonal2)
    }

    fn es_interior(alpha: f64, beta: f64) -> Option<(f64, f64)> {
        let intervalo = Intervalo::new(0.0, 1.0);

        if !intervalo.contiene(alpha) || !intervalo.contiene(beta) {
            return None;
        }

        // Disco
        //if (alpha * alpha + beta * beta).sqrt() >= 1.0 {
        //    return None;
        //}

        // Triángulo
        //if !(alpha > 0.0 && beta > 0.0 && alpha + beta < 1.0) {
        //   return None;
        //}

        Some((alpha, beta))
    }
}

impl Golpeable for Cuadrilatero {
    fn caja(&self) -> Caja {
        self.caja
    }

    fn rayo_golpea(
        &self,
        rayo: &crate::rayo::Rayo,
        intervalo: crate::intervalo::Intervalo,
    ) -> Option<crate::golpe::Golpe> {
        let denominador = Vec3::punto(&self.normal, &rayo.direccion());

        if denominador.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - Vec3::punto(&self.normal, &rayo.origen())) / denominador;

        if !intervalo.contiene(t) {
            return None;
        }

        let lugar = rayo.en(t);
        let p = lugar - self.origen;
        let alpha = Vec3::punto(&self.w, &Vec3::cruz(&p, &self.v));
        let beta = Vec3::punto(&self.w, &Vec3::cruz(&self.u, &p));

        let (tex_u, tex_v) = Cuadrilatero::es_interior(alpha, beta)?;

        Some(Golpe::new(
            lugar,
            self.normal,
            t,
            tex_u,
            tex_v,
            rayo,
            Arc::clone(&self.material),
        ))
    }
}
