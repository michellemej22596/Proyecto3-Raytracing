use nalgebra_glm::Vec3;
use crate::material::Material;
use crate::texture::Texture;

#[derive(Debug, Clone)]
pub struct Cube {
    pub min: Vec3,   // Esquina inferior del cubo
    pub max: Vec3,   // Esquina superior del cubo
    pub material: Material,
    pub is_skybox: bool,  // Nuevo campo para marcar si el cubo es un skybox
}


impl Cube {
    pub fn new(min: Vec3, max: Vec3, material: Material, is_skybox: bool) -> Self {
        Cube { min, max, material, is_skybox }
    }

    pub fn get_normal(&self, point: &Vec3) -> Vec3 {
        // Calcular la normal del cubo
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
        // Calcula las coordenadas UV basadas en la posición del punto en la cara del cubo
        // Este es un ejemplo básico; asegúrate de ajustar según el tamaño y orientación de tu cubo
        let u = (point.x - self.min.x) / (self.max.x - self.min.x);
        let v = (point.y - self.min.y) / (self.max.y - self.min.y);
        (u, v)
    }
}