
use nalgebra_glm::Vec3;
use crate::material::Material;
use crate::cube::Cube;  // Import the Cube

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Intersect {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
}

impl Intersect {
    pub fn new(point: Vec3, normal: Vec3, distance: f32, material: Material) -> Self {
        Intersect {
            point,
            normal,
            distance,
            is_intersecting: true,
            material,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            point: Vec3::zeros(),
            normal: Vec3::zeros(),
            distance: 0.0,
            is_intersecting: false,
            material: Material::black(),
        }
    }
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let inv_dir = Vec3::new(1.0, 1.0, 1.0).component_div(ray_direction); // Invertir la dirección del rayo para simplificar los cálculos

        // Limites del cubo (min y max)
        let tmin = (self.min - ray_origin).component_mul(&inv_dir);
        let tmax = (self.max - ray_origin).component_mul(&inv_dir);

        // Ordenamos los valores de tmin y tmax
        let t1 = Vec3::new(tmin.x.min(tmax.x), tmin.y.min(tmax.y), tmin.z.min(tmax.z));
        let t2 = Vec3::new(tmin.x.max(tmax.x), tmin.y.max(tmax.y), tmin.z.max(tmax.z));

        // Máximo valor mínimo y mínimo valor máximo
        let t_near = t1.x.max(t1.y).max(t1.z);  // Máximo de los valores mínimos
        let t_far = t2.x.min(t2.y).min(t2.z);   // Mínimo de los valores máximos


        if t_near < t_far && t_far > 0.0 {
            let point = ray_origin + ray_direction * t_near;
            let normal = self.get_normal(&point); // Normal basada en la cara del cubo
            return Intersect::new(point, normal, t_near, self.material);
        }

        Intersect::empty()
    }
}
