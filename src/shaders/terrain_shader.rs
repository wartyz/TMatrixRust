use cgmath::vec3;

use std::ffi::CStr;

use crate::entities::camera::Camera;
use crate::entities::light::Light;
use crate::shaders::shader_program::ShaderProgram;
use crate::shaders::static_shader::MAX_LIGHTS;
use crate::toolbox::maths;

type M4CG = cgmath::Matrix4<f32>;

const VERTEX_FILE: &str = "res/shaders/terrainShader.vert";
const FRAGMENT_FILE: &str = "res/shaders/terrainShader.frag";

///// Macro to get c strings from literals without runtime overhead
///// Literal must not contain any interior nul bytes!
//macro_rules! c_str {
//    ($literal:expr) => {
//        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
//    };
//}
#[derive(Debug, Clone, Copy)]
pub struct TerrainShader {
    program_id: u32,
    vertex_shader_id: u32,
    fragment_shader_id: u32,
    location_transformation_matrix: i32,
    location_projection_matrix: i32,
    location_view_matrix: i32,
    location_light_position: [i32; MAX_LIGHTS],
    location_light_color: [i32; MAX_LIGHTS],
    location_light_attenuation: [i32; MAX_LIGHTS],
    location_shine_damper: i32,
    location_reflectivity: i32,
    location_sky_colour: i32,
    location_background_texture: i32,
    location_r_texture: i32,
    location_g_texture: i32,
    location_b_texture: i32,
    location_blend_map: i32,

}

impl TerrainShader {
    pub fn new() -> TerrainShader {
        let p = ShaderProgram::new(VERTEX_FILE, FRAGMENT_FILE);
        let mut location_light_position: [i32; MAX_LIGHTS] = [0; MAX_LIGHTS];
        let mut location_light_color: [i32; MAX_LIGHTS] = [0; MAX_LIGHTS];
        let mut location_light_attenuation: [i32; MAX_LIGHTS] = [0; MAX_LIGHTS];

        for i in 0..MAX_LIGHTS {
            location_light_position[i] = p.location_light_position[i];
            location_light_color[i] = p.location_light_color[i];
            location_light_attenuation[i] = p.location_light_attenuation[i];
        }

        TerrainShader {
            program_id: p.program_id,
            vertex_shader_id: p.vertex_shader_id,
            fragment_shader_id: p.fragment_shader_id,
            location_transformation_matrix: p.location_transformation_matrix,
            location_projection_matrix: p.location_projection_matrix,
            location_view_matrix: p.location_view_matrix,
            //location_light_position: p.location_light_position,
            //location_light_color: p.location_light_color,
            location_shine_damper: p.location_shine_damper,
            location_reflectivity: p.location_reflectivity,
            location_sky_colour: p.location_sky_colour,

            location_background_texture: p.location_background_texture,
            location_r_texture: p.location_r_texture,
            location_g_texture: p.location_g_texture,
            location_b_texture: p.location_b_texture,
            location_blend_map: p.location_blend_map,

            location_light_position,
            location_light_color,
            location_light_attenuation,
        }
    }

    pub fn connect_texture_units(&self) {
        ShaderProgram::load_int(self.location_background_texture, 0);
        ShaderProgram::load_int(self.location_r_texture, 1);
        ShaderProgram::load_int(self.location_g_texture, 2);
        ShaderProgram::load_int(self.location_b_texture, 3);
        ShaderProgram::load_int(self.location_blend_map, 4);
    }


    pub fn _get_uniform_location(program_id: u32, uniform_name: &CStr) -> i32 {
        unsafe {
            gl::GetUniformLocation(program_id, uniform_name.as_ptr())
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

    pub fn _bind_attributes(program_id: u32) {
        unsafe {
            ShaderProgram::bind_attribute(program_id, 0, c_str!("position"));
            ShaderProgram::bind_attribute(program_id, 1, c_str!("textureCoordinates"));
            ShaderProgram::bind_attribute(program_id, 2, c_str!("normal"));
        }
    }

    pub fn load_sky_colour(&self, r: f32, g: f32, b: f32) {
        ShaderProgram::load_vector(self.location_sky_colour, vec3(r, g, b));
    }

    pub fn load_shine_variables(&self, damper: f32, reflectivity: f32) {
        ShaderProgram::load_float(self.location_shine_damper, damper);
        ShaderProgram::load_float(self.location_reflectivity, reflectivity);
    }

    pub fn load_lights(&self, lights: &Vec<Light>) {
        for i in 0..MAX_LIGHTS {
            if i < lights.len() {
                ShaderProgram::load_vector(self.location_light_position[i], lights[i].get_position());
                ShaderProgram::load_vector(self.location_light_color[i], lights[i].get_color());
                ShaderProgram::load_vector(
                    self.location_light_attenuation[i], lights[i].get_attenuation());
            } else {
                ShaderProgram::load_vector(self.location_light_position[i], vec3(0.0, 0.0, 0.));
                ShaderProgram::load_vector(self.location_light_color[i], vec3(0.0, 0.0, 0.));
                ShaderProgram::load_vector(self.location_light_attenuation[i], vec3(1.0, 0.0, 0.0));
            }
        }
    }
    pub fn _load_float(location: i32, value: f32) {
        unsafe {
            gl::Uniform1f(location, value); //Modifica variable uniform float con value
        }
    }

    pub fn load_transformation_matrix(&self, matrix: &M4CG) {
        //dbg!(matrix);
        ShaderProgram::load_matrix(self.location_transformation_matrix, &matrix);
    }

    pub fn load_view_matrix(&self, camera: &mut Camera) {
        let view_matrix = maths::create_view_matrix(camera);
        //dbg!(&view_matrix);
        ShaderProgram::load_matrix(self.location_view_matrix, &view_matrix);
    }
    pub fn load_projection_matrix(&self, projection: &M4CG) {
        ShaderProgram::load_matrix(self.location_projection_matrix, &projection);
    }
}