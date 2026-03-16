use std::sync::Arc;

use crate::{
    material::Material,
    rayo::Rayo,
    vec3::{Point3, Vec3},
};

pub mod golpeable;
pub mod lista_golpeable;
pub mod cuadrilatero;
pub mod esfera;
pub mod nodo;
pub mod trasladar;
pub mod rotar_y;
pub mod constante_media;

pub struct Golpe {
    lugar: Point3,
    normal: Vec3,
    distancia: f64,
    textura_horizontal: f64,
    textura_vertical: f64,
    cara_frontal: bool,
    material: Arc<dyn Material>,
}

impl Golpe {
    pub fn new(
        lugar_golpe: Point3,
        normal_exterior: Vec3,
        distancia: f64,
        textura_horizontal: f64,
        textura_vertical: f64,
        rayo: &Rayo,
        material: Arc<dyn Material>,
    ) -> Self {
        let (normal, cara_frontal) = Self::set_cara_frontal(rayo, &normal_exterior);

        Self {
            lugar: lugar_golpe,
            normal,
            distancia,
            textura_horizontal,
            textura_vertical,
            cara_frontal,
            material,
        }
    }
    pub fn lugar(&self) -> Point3 {
        self.lugar
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn distancia(&self) -> f64 {
        self.distancia
    }
    pub fn textura_horizontal(&self) -> f64 {
        self.textura_horizontal
    }
    pub fn textura_vertical(&self) -> f64 {
        self.textura_vertical
    }
    pub fn cara_frontal(&self) -> bool {
        self.cara_frontal
    }
    pub fn material(&self) -> &Arc<dyn Material> {
        &self.material
    }

   pub fn set_cara_frontal(rayo: &Rayo, normal_exterior: &Vec3) -> (Vec3, bool) {
        let cara_frontal = Vec3::punto(&rayo.direccion(), normal_exterior) < 0.0;

        let normal = if cara_frontal {
            *normal_exterior
        } else {
            -*normal_exterior
        };

        (normal, cara_frontal)
    }
}
