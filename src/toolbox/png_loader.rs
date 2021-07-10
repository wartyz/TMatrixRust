use image::DynamicImage;
use image::GenericImageView;

use std::path::Path;

pub struct PngLoader {
    png_temp: Vec<u8>,
    width: usize,
    height: usize,
}

impl Clone for PngLoader {
    fn clone(&self) -> Self {
        let mut v: Vec<u8> = vec![];
        for f in 0..self.png_temp.len() {
            v.push(self.png_temp[f]);
        }
        PngLoader {
            png_temp: v,
            width: self.width,
            height: self.height,

        }
    }
}

impl PngLoader {
    pub fn new() -> PngLoader {
        PngLoader {
            png_temp: vec![],
            width: 0,
            height: 0,
        }
    }

    pub fn load_image(&mut self, height_map: &str) -> Result<Vec<u8>, String> {
        let img: DynamicImage = image::open(&Path::new(height_map)).map_err(|e|
            format!("No se pudo cargar en PngLoader {}", e))?;

        self.width = img.width() as usize;
        self.height = img.height() as usize;

        self.png_temp = img.raw_pixels(); //convierte a Vec<u8>

        Ok(self.png_temp.clone())
    }

    pub fn _get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }


    pub fn get_rgb(&self, x: usize, y: usize) -> f32 {
        let (mut r, mut g, mut b) = (0, 0, 0);

        if self.height > 0 && self.width > 0 {
            let index_r = 3 * y * self.width + 3 * x + 0;
            let index_g = 3 * y * self.width + 3 * x + 1;
            let index_b = 3 * y * self.width + 3 * x + 2;

            //if index_r < self.png_temp.len() {
            r = self.png_temp[index_r] as i32;
            g = self.png_temp[index_g] as i32;
            b = self.png_temp[index_b] as i32;
        }

        let h: i32 = (r << 16) | (g << 8) | (b << 0);
        let height = h as f32;
        height
    }
}