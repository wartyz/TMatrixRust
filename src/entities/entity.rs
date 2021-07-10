use crate::models::textured_model::TexturedModel;

type V3CG = cgmath::Vector3<f32>;

#[derive(Debug, Copy)]
pub struct Entity {
    pub id: usize,
    pub model: TexturedModel,
    position: V3CG,
    rotation: V3CG,
    scale: V3CG,
    texture_index: i32,
}

impl Clone for Entity {
    fn clone(&self) -> Self {
        Entity::new2(self.id, self.model, self.texture_index, self.position,
                     self.rotation, self.scale, )
    }
}


impl Entity {
    pub fn new(id: usize, tex_model: TexturedModel,
               position: V3CG, rotation: V3CG, scale: V3CG) -> Entity {
        Entity {
            id,
            model: tex_model,
            position,
            rotation,
            scale,
            texture_index: 0,
        }
    }

    pub fn new2(id: usize, tex_model: TexturedModel, index: i32,
                position: V3CG, rotation: V3CG, scale: V3CG) -> Entity {
        Entity {
            id,
            model: tex_model,
            position,
            rotation,
            scale,
            texture_index: index,
        }
    }

    // Offset para el cálculo de las texturas atlas
    pub fn get_texture_x_offset(&self) -> f32 {
        let column = self.texture_index % self.model.get_texture().get_number_of_rows();
        column as f32 / self.model.get_texture().get_number_of_rows() as f32
    }

    pub fn get_texture_y_offset(&self) -> f32 {
        let row = self.texture_index / self.model.get_texture().get_number_of_rows();
        row as f32 / self.model.get_texture().get_number_of_rows() as f32
    }
    pub fn increase_position(&mut self, translation: V3CG) {
        self.position.x += translation.x;
        self.position.y += translation.y;
        self.position.z += translation.z;
    }

    pub fn increase_rotation(&mut self, rotation: V3CG) {
        self.rotation.x += rotation.x;
        self.rotation.y += rotation.y;
        self.rotation.z += rotation.z;
    }

    pub fn _scale(&mut self, scale: V3CG) {
        self.scale.x *= scale.x;
        self.scale.y *= scale.y;
        self.scale.z *= scale.z;
    }

    pub fn get_model(&self) -> TexturedModel {
        self.model
    }

    pub fn _set_model(&mut self, tex_model: TexturedModel) {
        self.model = tex_model;
    }

    pub fn get_position(&self) -> V3CG {
        self.position
    }
    pub fn _get_position_y(&self) -> f32 {
        self.position.y
    }

    pub fn set_position(&mut self, position: V3CG) {
        self.position = position;
    }
    pub fn set_position_y(&mut self, y: f32) {
        self.position.y = y;
    }


    pub fn get_rotation_x(&self) -> f32 {
        self.rotation.x
    }
    pub fn get_rotation_y(&self) -> f32 {
        self.rotation.y
    }
    pub fn get_rotation_z(&self) -> f32 {
        self.rotation.z
    }

    pub fn _set_rotation_x(&mut self, rot: f32) {
        self.rotation.x = rot;
    }
    pub fn _set_rotation_y(&mut self, rot: f32) {
        self.rotation.y = rot;
    }
    pub fn _set_rotation_z(&mut self, rot: f32) {
        self.rotation.z = rot;
    }

    pub fn get_scale(&self) -> V3CG {
        self.scale
    }

    pub fn _set_scale(&mut self, scale: V3CG) {
        self.scale = scale;
    }
}