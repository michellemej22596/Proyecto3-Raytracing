use crate::color::Color;
use crate::texture::Texture; // Asegúrate de tener el archivo `texture.rs`

#[derive(Debug, Clone)]  // Quitamos `Copy`
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 2],
    pub texture: Option<Texture>,  // Deja la textura como `Option`
}


impl Material {
    pub fn new(diffuse: Color, specular: f32, albedo: [f32; 2]) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            texture: None, // Por defecto, no hay textura
        }
    }

    pub fn with_texture(diffuse: Color, specular: f32, albedo: [f32; 2], texture: Texture) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            texture: Some(texture),
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            specular: 0.0,
            albedo: [0.0, 0.0],
            texture: None,
        }
    }
}
