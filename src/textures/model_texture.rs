#[derive(Debug, Clone, Copy)]
pub struct ModelTexture {
    texture_id: u32,
    shine_damper: f32,
    reflectivity: f32,
    has_transparency: bool,
    use_fake_lighting: bool,
    number_of_rows: i32, // Para atlas de textura
}

impl ModelTexture {
    pub fn new(id: u32) -> ModelTexture {
        ModelTexture {
            texture_id: id,
            shine_damper: 1.0,
            reflectivity: 0.0,
            has_transparency: false,//Transparencia
            use_fake_lighting: false,//Mala iluminación
            number_of_rows: 1,// Numero de filas del fichero atlas de textura
        }
    }
    pub fn get_number_of_rows(&self) -> i32 {
        self.number_of_rows
    }
    pub fn set_number_of_rows(&mut self, number_of_rows: i32) {
        self.number_of_rows = number_of_rows;
    }

    pub fn is_use_fake_lighting(&self) -> bool {
        self.use_fake_lighting
    }

    pub fn set_use_fake_lighting(&mut self, use_fake_lighting: bool) {
        self.use_fake_lighting = use_fake_lighting;
    }

    pub fn is_has_transparency(&self) -> bool {
        self.has_transparency
    }

    pub fn set_has_transparency(&mut self, has_transparency: bool) {
        self.has_transparency = has_transparency;
    }
    pub fn get_id(&self) -> u32 {
        self.texture_id
    }

    pub fn get_shine_damper(&self) -> f32 {
        self.shine_damper
    }

    pub fn _set_shine_damper(&mut self, shine_damper: f32) {
        self.shine_damper = shine_damper;
    }

    pub fn get_reflectivity(&self) -> f32 {
        self.reflectivity
    }

    pub fn _set_reflectivity(&mut self, reflectivity: f32) {
        self.reflectivity = reflectivity;
    }
}