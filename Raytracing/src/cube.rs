
use nalgebra_glm::Vec3;
use crate::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct Cube {
    pub min: Vec3,   // Esquina inferior del cubo
    pub max: Vec3,   // Esquina superior del cubo
    pub material: Material,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, material: Material) -> Self {
        Cube { min, max, material }
    }

    pub fn get_normal(&self, point: &Vec3) -> Vec3 {
        // Devuelve la normal adecuada dependiendo de qué cara del cubo fue impactada
        if (point.x - self.min.x).abs() < 1e-4 {
            return Vec3::new(-1.0, 0.0, 0.0);
        } else if (point.x - self.max.x).abs() < 1e-4 {
            return Vec3::new(1.0, 0.0, 0.0);
        } else if (point.y - self.min.y).abs() < 1e-4 {
            return Vec3::new(0.0, -1.0, 0.0);
        } else if (point.y - self.max.y).abs() < 1e-4 {
            return Vec3::new(0.0, 1.0, 0.0);
        } else if (point.z - self.min.z).abs() < 1e-4 {
            return Vec3::new(0.0, 0.0, -1.0);
        } else {
            return Vec3::new(0.0, 0.0, 1.0);
        }
    }
}
