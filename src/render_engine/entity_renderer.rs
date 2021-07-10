use cgmath;

use std::ptr;

use crate::entities::entity::Entity;
use crate::models::textured_model::TexturedModel;
use crate::render_engine::master_renderer::MasterRenderer;
use crate::shaders::static_shader::StaticShader;
use crate::toolbox::maths::*;

type M4CG = cgmath::Matrix4<f32>;

pub struct EntityRenderer {
    shader: StaticShader,
}

impl EntityRenderer {
    pub fn new(
        shader: StaticShader, projection_matrix: &M4CG) -> EntityRenderer {
        shader.start();
        shader.load_projection_matrix(projection_matrix);
        shader.stop();
        EntityRenderer {
            //projection_matrix,
            shader,
        }
    }


    pub fn render(&mut self, entities: &Vec<Vec<Entity>>) {
        for i in 0..entities.len() {
            let vv = entities[i].clone();
            for entity in vv {
                self.prepare_textured_model(&entity.model);
                //println!("e = {:#?} ", e);
                self.prepare_instance(entity);
                //dbg!(e.get_model().get_raw_model().get_vertex_count());
                unsafe {
                    gl::DrawElements(
                        gl::TRIANGLES,// modo
                        entity // Entity
                            .get_model()
                            .get_raw_model()
                            .get_vertex_count(),// número de índices a
                        // renderizar
                        gl::UNSIGNED_INT,
                        ptr::null());
                }
            }
        }
    }

    pub fn _unbind_textured_model(&mut self) {
        unsafe {
            MasterRenderer::enable_culling();
            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            gl::DisableVertexAttribArray(2);
            gl::BindVertexArray(0);
        }
    }

    pub fn prepare_instance(&mut self, entity: Entity) {
        //Crea matriz de transformación con los datos de la entity
        let transformation_matrix = create_transformation_matrix(
            entity.get_position(),
            entity.get_rotation_x(), entity.get_rotation_y(), entity.get_rotation_z(),
            entity.get_scale(),
        );

        //Envia la matriz de transformación de la entity al shader
        self.shader.load_transformation_matrix(&transformation_matrix);
        self.shader.load_offset(entity.get_texture_x_offset(), entity.get_texture_y_offset());
    }


    pub fn prepare_textured_model(&mut self, model: &TexturedModel) {
        let raw_model = model.get_raw_model();
        unsafe {
            gl::BindVertexArray(raw_model.get_vao_id());
            //Activa VAO 0 (vértices).
            gl::EnableVertexAttribArray(0);
            //Activa VAO 1 (coordenadas de textura).
            gl::EnableVertexAttribArray(1);
            //Activa VAO 2 (coordenadas de normals).
            gl::EnableVertexAttribArray(2);


            let texture = model.get_texture();
            self.shader.load_number_of_rows(texture.get_number_of_rows());
            if texture.is_has_transparency() {
                MasterRenderer::disable_culling();
            }

            self.shader.load_fake_lighting_variable(texture.is_use_fake_lighting());
            self.shader.load_shine_variables(texture.get_shine_damper(), texture.get_reflectivity());

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, model.get_texture().get_id());
        }
    }
}
