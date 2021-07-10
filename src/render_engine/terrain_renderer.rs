use cgmath::vec3;

use std::ptr;

use crate::shaders::terrain_shader::TerrainShader;
use crate::terrains::terrain::Terrain;
use crate::toolbox::maths::*;

type M4CG = cgmath::Matrix4<f32>;

pub struct TerrainRenderer {
    shader: TerrainShader,
}

impl TerrainRenderer {
    pub fn new(shader: TerrainShader, projection_matrix: M4CG) -> TerrainRenderer {
        shader.start();
        shader.load_projection_matrix(&projection_matrix);
        shader.connect_texture_units();
        shader.stop();
        TerrainRenderer {
            shader,
        }
    }

    pub fn render(&mut self, terrains: &Vec<Terrain>) {
        for terrain in terrains {
            self.prepare_terrain(terrain);
            self.load_model_matrix(terrain.clone());
            unsafe {
                gl::DrawElements(
                    gl::TRIANGLES,// modo
                    terrain
                        .get_model()
                        .get_vertex_count(),// número de índices a
                    // renderizar
                    gl::UNSIGNED_INT,
                    ptr::null());
            }
            self.unbind_textured_model();
        }
    }

    pub fn prepare_terrain(&mut self, terrain: &Terrain) {
        let raw_model = terrain.get_model();
        unsafe {
            gl::BindVertexArray(raw_model.get_vao_id());
            //Activa VAO 0 (vértices).
            gl::EnableVertexAttribArray(0);
            //Activa VAO 1 (coordenadas de textura).
            gl::EnableVertexAttribArray(1);
            //Activa VAO 2 (coordenadas de normals).
            gl::EnableVertexAttribArray(2);

            self.bind_textures(terrain);
            self.shader.load_shine_variables(1.0, 0.0);
        }
    }

    pub fn bind_textures(&self, terrain: &Terrain) {
        let texture_pack = terrain.get_texture_pack();
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_pack.get_background_texture().get_texture_id());
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture_pack.get_r_texture().get_texture_id());
            gl::ActiveTexture(gl::TEXTURE2);
            gl::BindTexture(gl::TEXTURE_2D, texture_pack.get_g_texture().get_texture_id());
            gl::ActiveTexture(gl::TEXTURE3);
            gl::BindTexture(gl::TEXTURE_2D, texture_pack.get_b_texture().get_texture_id());
            gl::ActiveTexture(gl::TEXTURE4);
            gl::BindTexture(gl::TEXTURE_2D, terrain.get_blend_map().get_texture_id());
        }
    }

    pub fn unbind_textured_model(&self) {
        unsafe {
            gl::DisableVertexAttribArray(0); // VA0 vertices
            gl::DisableVertexAttribArray(1); // VA1 textura
            gl::DisableVertexAttribArray(2); // VA2 vectores normal
            gl::BindVertexArray(0);
        }
    }

    pub fn load_model_matrix(&mut self, terrain: Terrain) {
        let transformation_matrix = create_transformation_matrix(
            vec3(terrain.get_x(), 0.0, terrain.get_z()),
            0.0, 0.0, 0.0,
            vec3(1.0, 1.0, 1.0));
        self.shader.load_transformation_matrix(&transformation_matrix);
    }
}