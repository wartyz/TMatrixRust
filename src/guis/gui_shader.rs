use std::ffi::CStr;

use crate::shaders::shader_program::ShaderProgram;

type M4CG = cgmath::Matrix4<f32>;

const VERTEX_FILE: &str = "res/shaders/guiShader.vert";
const FRAGMENT_FILE: &str = "res/shaders/guiShader.frag";

pub struct GuiShader {
    program_id: u32,
    vertex_shader_id: u32,
    fragment_shader_id: u32,
    location_transformation_matrix: i32,
}

impl GuiShader {
    pub fn new() -> GuiShader {
        let p = ShaderProgram::new(VERTEX_FILE, FRAGMENT_FILE);
        GuiShader {
            program_id: p.program_id,
            vertex_shader_id: p.vertex_shader_id,
            fragment_shader_id: p.fragment_shader_id,
            location_transformation_matrix: p.location_transformation_matrix,
        }
    }

    pub fn load_transformation(&self, matrix: M4CG) {
        ShaderProgram::load_matrix(self.location_transformation_matrix, &matrix)
    }


    pub fn _get_uniform_location(&self, uniform_name: &CStr) -> i32 {
        unsafe {
            gl::GetUniformLocation(self.program_id, uniform_name.as_ptr())
        }
    }
    pub fn _bind_attributes(program_id: u32) {
        unsafe {
            ShaderProgram::bind_attribute(program_id, 0, c_str!("position"));
        }
    }

    pub fn start(&self) {
        unsafe {
            gl::UseProgram(self.program_id); //Hace funcionar el programa shader
        }
    }

    pub fn stop(&self) {
        unsafe {
            gl::UseProgram(0); //Para el programa shader
        }
    }

    pub fn cleanup(&self) {
        //Desconectamos y borramos shaders
        self.stop();
        unsafe {
            gl::DetachShader(self.program_id, self.vertex_shader_id);
            gl::DetachShader(self.program_id, self.fragment_shader_id);
            gl::DeleteShader(self.vertex_shader_id);
            gl::DeleteShader(self.fragment_shader_id);
            gl::DeleteProgram(self.program_id);
        }
    }
}