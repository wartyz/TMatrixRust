pub struct TextureData {
    width: u32,
    height: u32,
    buffer: Vec<u8>,
}

impl TextureData {
    pub fn new(buffer: Vec<u8>, width: u32, height: u32) -> TextureData {
        TextureData {
            width,
            height,
            buffer,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_buffer(&self) -> Vec<u8> {
        self.buffer.clone()
    }
}