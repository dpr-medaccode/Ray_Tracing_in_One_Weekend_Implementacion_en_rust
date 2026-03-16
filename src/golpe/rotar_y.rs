use std::sync::Arc;

use crate::{
    caja::{self, Caja, Eje},
    golpe::golpeable::Golpeable,
    rayo::Rayo,
    util::grados_a_radianes,
    vec3::Vec3,
};

pub struct RotarY {
    objeto: Arc<dyn Golpeable>,
    caja: Caja,
    angulo_seno: f64,
    angulo_coseno: f64,
}

impl RotarY {
    pub fn new(objeto: Arc<dyn Golpeable>, angulo: f64) -> Self {
        let radianes = grados_a_radianes(angulo);

        let angulo_seno = radianes.sin();
        let angulo_coseno = radianes.cos();
        let caja = objeto.caja();

        let mut mininimo = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut maximo = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * caja.x.maximo + (1 - i) as f64 * caja.x.minimo;
                    let y = j as f64 * caja.y.maximo + (1 - j) as f64 * caja.y.minimo;
                    let z = k as f64 * caja.z.maximo + (1 - k) as f64 * caja.z.minimo;

                    let new_x = angulo_coseno * x + angulo_seno * z;
                    let new_z = -angulo_seno * x + angulo_coseno * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    maximo = Vec3::new(
                        maximo.x().max(tester.x()),
                        maximo.y().max(tester.y()),
                        maximo.z().max(tester.z()),
                    );

                    mininimo = Vec3::new(
                        mininimo.x().min(tester.x()),
                        mininimo.y().min(tester.y()),
                        mininimo.z().min(tester.z()),
                    );
                }
            }
        }

        let caja = Caja::new_from_vec3(mininimo, maximo);

        RotarY {
            objeto,
            caja,
            angulo_seno,
            angulo_coseno,
        }
    }
}

impl Golpeable for RotarY {
    fn caja(&self) -> Caja {
        self.caja
    }

    fn rayo_golpea(
        &self,
        rayo: &crate::rayo::Rayo,
        intervalo: crate::intervalo::Intervalo,
    ) -> Option<super::Golpe> {
        // Transformar el rayo del espacio mundo al espacio del objeto
        let origen = Vec3::new(
            self.angulo_coseno * rayo.origen().x() - self.angulo_seno * rayo.origen().z(),
            rayo.origen().y(),
            self.angulo_seno * rayo.origen().x() + self.angulo_coseno * rayo.origen().z(),
        );

        let direccion = Vec3::new(
            self.angulo_coseno * rayo.direccion().x() - self.angulo_seno * rayo.direccion().z(),
            rayo.direccion().y(),
            self.angulo_seno * rayo.direccion().x() + self.angulo_coseno * rayo.direccion().z(),
        );

        let rayo_rotado = crate::rayo::Rayo::new_con_tiempo(origen, direccion, rayo.tiempo());

        if let Some(mut golpe) = self.objeto.rayo_golpea(&rayo_rotado, intervalo) {
            golpe.lugar = Vec3::new(
                self.angulo_coseno * golpe.lugar().x() + self.angulo_seno * golpe.lugar().z(),
                golpe.lugar().y(),
                -self.angulo_seno * golpe.lugar().x() + self.angulo_coseno * golpe.lugar().z(),
            );

            golpe.normal = Vec3::new(
                self.angulo_coseno * golpe.normal().x() + self.angulo_seno * golpe.normal().z(),
                golpe.normal().y(),
                -self.angulo_seno * golpe.normal().x() + self.angulo_coseno * golpe.normal().z(),
            );

            return Some(golpe);
        }

        None
    }
}
