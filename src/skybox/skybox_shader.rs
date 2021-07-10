use cgmath::num_traits::float::FloatCore;
use cgmath::vec3;

use std::ffi::CStr;

use crate::entities::camera::Camera;
use crate::render_engine::display_manager::DisplayManager;
use crate::shaders::shader_program::ShaderProgram;
use crate::toolbox::maths;
use crate::toolbox::maths::Matrix4x4;

type M4CG = cgmath::Matrix4<f32>;

const VERTEX_FILE: &str = "res/shaders/skyboxShader.vert";
const FRAGMENT_FILE: &str = "res/shaders/skyboxShader.frag";

const ROTATE_SPEED: f32 = 1.0;

#[derive(Debug, Clone, Copy)]
pub struct SkyboxShader {
    program_id: u32,
    vertex_shader_id: u32,
    fragment_shader_id: u32,
    location_projection_matrix: i32,
    location_view_matrix: i32,
    location_fog_colour: i32,
    location_cube_map: i32,
    location_cube_map2: i32,
    location_blend_factor: i32,

    rotation: f32,
}

impl SkyboxShader {
    pub fn new() -> SkyboxShader {
        let p = ShaderProgram::new(VERTEX_FILE, FRAGMENT_FILE);


        SkyboxShader {
            program_id: p.program_id,
            vertex_shader_id: p.vertex_shader_id,
            fragment_shader_id: p.fragment_shader_id,
            location_projection_matrix: p.location_projection_matrix,
            location_view_matrix: p.location_view_matrix,
            location_fog_colour: p.location_fog_colour,
            location_cube_map: p.location_cube_map,
            location_cube_map2: p.location_cube_map2,
            location_blend_factor: p.location_blend_factor,
            rotation: 0.0,

        }
    }

    pub fn load_projection_matrix(&self, projection: &M4CG) {
        ShaderProgram::load_matrix(self.location_projection_matrix, &projection);
    }

    pub fn load_view_matrix(&mut self, camera: &mut Camera, dm: &DisplayManager) {
        let mut matrix = maths::create_view_matrix(camera);
        matrix[3][0] = 0.0;// Modificaciones en matriz para skybox
        matrix[3][1] = 0.0;
        matrix[3][2] = 0.0;

        self.rotation += ROTATE_SPEED * dm.get_frame_time_seconds();
        let matrix =
            maths::Matrix4x4::rotate_y(self.rotation.to_radians(), &Matrix4x4::m4cg_to_matrix4x4(matrix));

        ShaderProgram::load_matrix(self.location_view_matrix, &Matrix4x4::matrix4x4_to_m4gc(matrix));
    }

    pub fn load_fog_colour(&self, r: f32, g: f32, b: f32) {
        ShaderProgram::load_vector(self.location_fog_colour, vec3(r, g, b));
    }
    pub fn connect_texture_units(&self) {
        ShaderProgram::load_int(self.location_cube_map, 0);
        ShaderProgram::load_int(self.location_cube_map2, 1);
    }

    pub fn load_blend_factorr(&self, blend: f32) {
        ShaderProgram::load_float(self.location_blend_factor, blend);
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

    pub fn _bind_attributes(&self) {
        unsafe {
            ShaderProgram::bind_attribute(self.program_id, 0, c_str!("position"));
        }
    }
}