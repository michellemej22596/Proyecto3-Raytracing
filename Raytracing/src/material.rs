use crate::color::Color;
use crate::texture::Texture; // Asegúrate de tener el archivo `texture.rs`

#[derive(Debug, Clone)]  // Quitamos `Copy`
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 2],
    pub texture: Option<Texture>,  // Deja la textura como `Option`
    pub reflectivity: f32,         // Nuevo: Nivel de reflectividad (0.0 a 1.0)
    pub transparency: f32,         // Nuevo: Nivel de transparencia (0.0 a 1.0)
}

impl Material {
    // Método para un material sin textura
    pub fn new(diffuse: Color, specular: f32, albedo: [f32; 2], reflectivity: f32, transparency: f32) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            texture: None, // Por defecto, no hay textura
            reflectivity,  // Agregamos reflectividad
            transparency,  // Agregamos transparencia
        }
    }

    // Método para un material con textura
    pub fn with_texture(diffuse: Color, specular: f32, albedo: [f32; 2], texture: Texture, reflectivity: f32, transparency: f32) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            texture: Some(texture), // Agregamos la textura
            reflectivity,  // Agregamos reflectividad
            transparency,  // Agregamos transparencia
        }
    }

    // Método para un material negro sin reflectividad ni transparencia
    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            specular: 0.0,
            albedo: [0.0, 0.0],
            texture: None,
            reflectivity: 0.0, // No tiene reflectividad
            transparency: 0.0, // No es transparente
        }
    }
}
