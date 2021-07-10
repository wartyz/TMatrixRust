use crate::guis::gui_shader::GuiShader;
use crate::guis::gui_texture::GuiTexture;
use crate::models::raw_model::RawModel;
use crate::render_engine::loader::Loader;
use crate::toolbox::maths::*;

type M4CG = cgmath::Matrix4<f32>;

pub struct GuiRenderer {
    quad: RawModel,
    shader: GuiShader,
}

impl GuiRenderer {
    pub fn new(loader: &mut Loader) -> GuiRenderer {
        let positions = vec![-1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0];
        let quad = loader.load_to_vao2(&positions, 2);
        let shader = GuiShader::new();

        GuiRenderer {
            quad,
            shader,
        }
    }

    pub fn render(&self, guis: &Vec<GuiTexture>) {
        self.shader.start();
        unsafe {
            gl::BindVertexArray(self.quad.get_vao_id());
            //Activa VAO 0 (vértices).
            gl::EnableVertexAttribArray(0);

            // Para poder tener transparencia en imagenes GUI
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            // Para poder poner varias GUIs sin que se tapen
            gl::Disable(gl::DEPTH_TEST);

            // Renderizamos
            for gui in guis {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, gui.get_texture());
                let matrix: M4CG = create_transformation_matrix2(gui.get_position(),
                                                                 gui.get_scale());
                self.shader.load_transformation(matrix);
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, self.quad.get_vertex_count());
            }
            gl::Enable(gl::DEPTH_TEST);
            gl::Disable(gl::BLEND);
            gl::DisableVertexAttribArray(0);
            gl::BindVertexArray(0);
            self.shader.stop();
        }
    }

    pub fn cleanup(&self) {
        self.shader.cleanup();
    }
}