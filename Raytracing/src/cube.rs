use nalgebra_glm::Vec3;
use crate::material::Material;
use crate::texture::Texture;

#[derive(Debug, Clone)]  // Quitamos Copy
pub struct Cube {
    pub min: Vec3,   // Esquina inferior del cubo
    pub max: Vec3,   // Esquina superior del cubo
    pub material: Material,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, material: Material) -> Self {
        Cube { min, max, material }
    }

    // Calcula la normal en función de la cara del cubo que fue intersectada
    pub fn get_normal(&self, point: &Vec3) -> Vec3 {
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

    // Calcula las coordenadas UV según la intersección
    pub fn get_uv(&self, point: &Vec3) -> (f32, f32) {
        let normal = self.get_normal(point);
        let mut u = 0.0;
        let mut v = 0.0;

        if normal.x.abs() > 0.0 {
            u = (point.y - self.min.y) / (self.max.y - self.min.y);
            v = (point.z - self.min.z) / (self.max.z - self.min.z);
        } else if normal.y.abs() > 0.0 {
            u = (point.x - self.min.x) / (self.max.x - self.min.x);
            v = (point.z - self.min.z) / (self.max.z - self.min.z);
        } else if normal.z.abs() > 0.0 {
            u = (point.x - self.min.x) / (self.max.x - self.min.x);
            v = (point.y - self.min.y) / (self.max.y - self.min.y);
        }

        (u, v)
    }
}
