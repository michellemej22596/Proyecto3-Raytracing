use image::GenericImageView;
use crate::color::Color;

#[derive(Debug, Clone)]  // Añadimos Debug y Clone
pub struct Texture {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}


impl Texture {
    pub fn load_from_file(path: &str) -> Self {
        let img = image::open(path).expect("Error al abrir la imagen");
        let (width, height) = img.dimensions();
        let data = img.to_rgba8().into_raw();
        Texture { data, width, height }
    }

    // Obtener color a partir de las coordenadas UV
    pub fn get_color(&self, u: f32, v: f32) -> Color {
        let u = (u * self.width as f32) as usize % self.width as usize;
        let v = (v * self.height as f32) as usize % self.height as usize;
        let idx = (v * self.width as usize + u) * 4;
        Color::new(self.data[idx], self.data[idx + 1], self.data[idx + 2])
    }
}
