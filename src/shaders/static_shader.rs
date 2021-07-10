use cgmath::{vec2, vec3};

use std::ffi::CStr;

use crate::entities::camera::Camera;
use crate::entities::light::Light;
use crate::shaders::shader_program::ShaderProgram;
use crate::toolbox::maths;

type M4CG = cgmath::Matrix4<f32>;

pub const MAX_LIGHTS: usize = 4;
const VERTEX_FILE: &str = "res/shaders/shader.vert";
const FRAGMENT_FILE: &str = "res/shaders/shader.frag";
/// Macro to get c strings from literals without runtime overhead
/// Literal must not contain any interior nul bytes!
macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    };
}
#[derive(Debug, Clone, Copy)]
pub struct StaticShader {
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
    location_use_fake_lighting: i32,
    location_sky_colour: i32,
    location_number_of_rows: i32,
    location_offset: i32,
}

impl StaticShader {
    pub fn new() -> StaticShader {
        let p = ShaderProgram::new(VERTEX_FILE, FRAGMENT_FILE);
        let mut location_light_position: [i32; MAX_LIGHTS] = [0; MAX_LIGHTS];
        let mut location_light_color: [i32; MAX_LIGHTS] = [0; MAX_LIGHTS];
        let mut location_light_attenuation: [i32; MAX_LIGHTS] = [0; MAX_LIGHTS];

        for i in 0..MAX_LIGHTS {
            location_light_position[i] = p.location_light_position[i];
            location_light_color[i] = p.location_light_color[i];
            location_light_attenuation[i] = p.location_light_attenuation[i];
        }

        StaticShader {
            program_id: p.program_id,
            vertex_shader_id: p.vertex_shader_id,
            fragment_shader_id: p.fragment_shader_id,
            location_transformation_matrix: p.location_transformation_matrix,
            location_projection_matrix: p.location_projection_matrix,
            location_view_matrix: p.location_view_matrix,
//            location_light_position: p.location_light_position,
//            location_light_color: p.location_light_color,
            location_shine_damper: p.location_shine_damper,
            location_reflectivity: p.location_reflectivity,
            location_use_fake_lighting: p.location_use_fake_lighting,
            location_sky_colour: p.location_sky_colour,
            location_number_of_rows: p.location_number_of_rows,
            location_offset: p.location_offset,

            location_light_position,
            location_light_color,
            location_light_attenuation,
        }
    }

    // Para textura atlas
    pub fn load_number_of_rows(&self, number_of_rows: i32) {
        ShaderProgram::load_float(self.location_number_of_rows, number_of_rows as f32);
    }

    pub fn load_offset(&self, x: f32, y: f32) {
        ShaderProgram::load_2d_vector(self.location_offset, vec2(x, y));
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

    pub fn load_fake_lighting_variable(&self, use_fake: bool) {
        ShaderProgram::load_boolean(self.location_use_fake_lighting, use_fake);
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
                ShaderProgram::load_vector(self.location_light_attenuation[i], lights[i].get_attenuation
                ());
            } else {
                ShaderProgram::load_vector(self.location_light_position[i], vec3(0.0, 0.0, 0.0));
                ShaderProgram::load_vector(self.location_light_color[i], vec3(0.0, 0.0, 0.0));
                ShaderProgram::load_vector(self.location_light_attenuation[i], vec3(1.0, 0.0, 0.0));
            }
        }
    }

//    pub fn load_float(location: i32, value: f32) {
//        unsafe {
//            gl::Uniform1f(location, value); //Modifica variable uniform float con value
//        }
//    }
//
//    pub fn load_vector(location: i32, vector: V3CG) {
//        unsafe {
//            //Modifica variable 3D uniform location con vector
//            gl::Uniform3f(location, vector.x, vector.y, vector.z);
//        }
//    }

//    //Modifica variable  uniform booleana en location con value
//    pub fn load_boolean(location: i32, value: bool) {
//        let mut to_load: f32 = 0.0;
//        if value {
//            to_load = 1.0;
//        }
//        unsafe {
//            gl::Uniform1f(location, to_load);
//        }
//    }
//
//    pub fn load_matrix(location: i32, matrix: &M4CG) {
////        matrix.store(matrixBuffer); //Almacenamos la matriz en el FloatBuffer
////        matrixBuffer.flip();
//        //dbg!(location);
//        unsafe {
//
//            //Enviamos el buffer con datos al shader
//            //gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.data.as_ptr());
//            gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
//        }
//    }


//    pub fn load_shader(filename: &str, tipo: GLenum) -> GLuint {
//        let mut fichero = match File::open(filename) {
//            Err(error) => panic!("no se puede abrir {:?}: {}", filename, error),
//            Ok(f) => f,
//        };
//        let mut file_data = String::new();
//        fichero.read_to_string(&mut file_data);
//
//        unsafe {
//            let sh_id = gl::CreateShader(tipo); // crea shader
//
//            let sh_c_str = CString::new(file_data).unwrap();
//
//            // carga lo que habia en fichero
//            gl::ShaderSource(sh_id, 1, &sh_c_str.as_ptr(), ptr::null());
//            gl::CompileShader(sh_id); // lo compila
//
//            //Comprueba errores
//            let mut status: GLint = 0;
//            gl::GetShaderiv(sh_id, gl::COMPILE_STATUS, &mut status);
//            if (status == 0) {
//                panic!("ERROR: No se puede compilar shader {:?}", filename);
//            }
//            sh_id
//        }
//    }

//    pub fn bind_attribute(program_id: GLuint, attribute: GLuint, variable_name: &CStr) {
//        // carga una variable atributo en el shader
//        // c_str() genera una secuencia de caracteres finalizada con null,
//        // con el mismo contenido que el objeto string, y la devuelve como un
//        // puntero a un arreglo de char
//        unsafe {
//            gl::BindAttribLocation(program_id, attribute, variable_name.as_ptr());
//        }
//    }

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