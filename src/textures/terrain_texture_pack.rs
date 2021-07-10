use crate::textures::terrain_texture::TerrainTexture;

#[derive(Debug, Copy)]
pub struct TerrainTexturePack {
    bacground_texture: TerrainTexture,
    r_texture: TerrainTexture,
    g_texture: TerrainTexture,
    b_texture: TerrainTexture,

}

impl Clone for TerrainTexturePack {
    fn clone(&self) -> Self {
        TerrainTexturePack {
            bacground_texture: self.bacground_texture,
            r_texture: self.r_texture,
            g_texture: self.g_texture,
            b_texture: self.b_texture,
        }
    }
}

impl TerrainTexturePack {
    pub fn new(bacground_texture: TerrainTexture,
               r_texture: TerrainTexture,
               g_texture: TerrainTexture,
               b_texture: TerrainTexture) -> TerrainTexturePack {
        TerrainTexturePack {
            bacground_texture,
            r_texture,
            g_texture,
            b_texture,
        }
    }

    pub fn get_background_texture(&self) -> TerrainTexture {
        self.bacground_texture
    }

    pub fn get_r_texture(&self) -> TerrainTexture {
        self.r_texture
    }

    pub fn get_g_texture(&self) -> TerrainTexture {
        self.g_texture
    }

    pub fn get_b_texture(&self) -> TerrainTexture {
        self.b_texture
    }
}