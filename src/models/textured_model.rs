use crate::models::raw_model::RawModel;
use crate::textures::model_texture::ModelTexture;

#[derive(Debug, Clone, Copy)]
pub struct TexturedModel {
    pub raw_model: RawModel,
    texture: ModelTexture,
}

impl TexturedModel {
    pub fn new(raw_model: RawModel, texture: ModelTexture) -> TexturedModel {
        TexturedModel {
            raw_model,
            texture,
        }
    }

    pub fn get_raw_model(&self) -> RawModel {
        self.raw_model
    }

    pub fn get_texture(&self) -> ModelTexture {
        self.texture
    }
}