type V2CG = cgmath::Vector2<f32>;


pub struct GuiTexture {
    texture: u32,
    position: V2CG,
    scale: V2CG,
}

impl GuiTexture {
    pub fn new(texture: u32, position: V2CG, scale: V2CG) -> GuiTexture {
        GuiTexture {
            texture,
            position,
            scale,
        }
    }

    pub fn get_texture(&self) -> u32 {
        self.texture
    }

    pub fn get_position(&self) -> V2CG {
        self.position
    }

    pub fn get_scale(&self) -> V2CG {
        self.scale
    }
}