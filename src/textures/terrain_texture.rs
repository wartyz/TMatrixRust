#[derive(Debug, Copy)]
pub struct TerrainTexture {
    texture_id: u32,
}

impl Clone for TerrainTexture {
    fn clone(&self) -> Self {
        TerrainTexture {
            texture_id: self.texture_id,

        }
    }
}

impl TerrainTexture {
    pub fn new(texture_id: u32) -> TerrainTexture {
        TerrainTexture {
            texture_id
        }
    }
    pub fn get_texture_id(&self) -> u32 {
        self.texture_id
    }
}