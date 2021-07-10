use cgmath::Matrix;
use gl::types::*;

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::prelude::*;
use std::ptr;

use crate::entities::camera::Camera;
use crate::shaders::static_shader::MAX_LIGHTS;
use crate::toolbox::maths;

type V2CG = cgmath::Vector2<f32>;
type V3CG = cgmath::Vector3<f32>;
type M4CG = cgmath::Matrix4<f32>;

//const VERTEX_FILE: &str = "res/shaders/shader.vert";
//const FRAGMENT_FILE: &str = "res/shaders/shader.frag";
/// Macro to get c strings from literals without runtime overhead
/// Literal must not contain any interior nul bytes!
macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    };
}
#[derive(Debug, Clone, Copy)]
pub struct ShaderProgram {
    pub program_id: u32,
    pub vertex_shader_id: u32,
    pub fragment_shader_id: u32,
    pub location_transformation_matrix: i32,
    pub location_projection_matrix: i32,
    pub location_view_matrix: i32,
    //pub location_light_position: i32,
    //pub location_light_color: i32,
    pub location_shine_damper: i32,
    pub location_reflectivity: i32,
    pub location_use_fake_lighting: i32,
    pub location_sky_colour: i32,
    pub location_background_texture: i32,
    pub location_r_texture: i32,
    pub location_g_texture: i32,
    pub location_b_texture: i32,
    pub location_blend_map: i32,
    pub location_number_of_rows: i32,
    pub location_offset: i32,

    pub location_light_position: [i32; MAX_LIGHTS],
    pub location_light_color: [i32; MAX_LIGHTS],
    pub location_light_attenuation: [i32; MAX_LIGHTS],
    pub location_fog_colour: i32,

    pub location_cube_map: i32,
    pub location_cube_map2: i32,
    pub location_blend_factor: i32,
}

impl ShaderProgram {
    pub fn new(vertex_file: &str, fragment_file: &str) -> ShaderProgram {
        let vertex_shader_id: GLuint =
            ShaderProgram::load_shader(vertex_file, gl::VERTEX_SHADER);
        let fragment_shader_id: GLuint =
            ShaderProgram::load_shader(fragment_file, gl::FRAGMENT_SHADER);
        unsafe {
            let program_id = gl::CreateProgram();

            //Activamos shaders enlazandolos
            gl::AttachShader(program_id, vertex_shader_id);
            gl::AttachShader(program_id, fragment_shader_id);

            //ShaderProgram::bind_attributes();

            //linkamos programa shader
            gl::LinkProgram(program_id);
            gl::ValidateProgram(program_id);

            let mut status: GLint = 0;
            gl::GetProgramiv(program_id, gl::VALIDATE_STATUS, &mut status);
            if status == 0 {
                println!("ERROR: Programa Shader no validado");
            }

            ShaderProgram::bind_attributes(program_id); //Carga atributos del VAO

            //dbg!(location_transformation_matrix,location_projection_matrix, location_view_matrix);
            let mut s = ShaderProgram {
                program_id,
                vertex_shader_id,
                fragment_shader_id,
                location_transformation_matrix: 0,
                location_projection_matrix: 0,
                location_view_matrix: 0,
                //location_light_position: 0,
                //location_light_color: 0,
                location_shine_damper: 0,
                location_reflectivity: 0,
                location_use_fake_lighting: 0,
                location_sky_colour: 0,
                location_background_texture: 0,
                location_r_texture: 0,
                location_g_texture: 0,
                location_b_texture: 0,
                location_blend_map: 0,
                location_number_of_rows: 0,
                location_offset: 0,

                location_light_position: [0; MAX_LIGHTS],
                location_light_color: [0; MAX_LIGHTS],
                location_light_attenuation: [0; MAX_LIGHTS],

                location_fog_colour: 0,

                location_cube_map: 0,
                location_cube_map2: 0,
                location_blend_factor: 0,
            };
            s.get_all_uniform_locations();
            s
        }
    }

    pub fn get_uniform_location(&self, uniform_name: &CStr) -> i32 {
        unsafe {
            gl::GetUniformLocation(self.program_id, uniform_name.as_ptr())
        }
    }


    //    pub fn start(&self) {
//        unsafe {
//            gl::UseProgram(self.program_id); //Hace funcionar el programa shader
//        }
//    }
    pub fn _stop(&self) {
        unsafe {
            gl::UseProgram(0); //Para el programa shader
        }
    }

    pub fn _cleanup(&self) {
        //Desconectamos y borramos shaders
        self._stop();
        unsafe {
            gl::DetachShader(self.program_id, self.vertex_shader_id);
            gl::DetachShader(self.program_id, self.fragment_shader_id);
            gl::DeleteShader(self.vertex_shader_id);
            gl::DeleteShader(self.fragment_shader_id);
            gl::DeleteProgram(self.program_id);
        }
    }

    pub fn bind_attributes(program_id: u32) {
        unsafe {
            ShaderProgram::bind_attribute(program_id, 0, c_str!("position"));
            ShaderProgram::bind_attribute(program_id, 1, c_str!("textureCoordinates"));
            ShaderProgram::bind_attribute(program_id, 2, c_str!("normal"));
        }
    }

    pub fn get_all_uniform_locations(&mut self) {
        unsafe {
            self.location_transformation_matrix =
                self.get_uniform_location(c_str!("transformationMatrix"));

            self.location_projection_matrix =
                self.get_uniform_location(c_str!("projectionMatrix"));

            self.location_view_matrix =
                self.get_uniform_location(c_str!("viewMatrix"));

//            self.location_light_position =
//                self.get_uniform_location(c_str!("lightPosition"));
//
//            self.location_light_color =
//                self.get_uniform_location(c_str!("lightColour"));

            self.location_shine_damper =
                self.get_uniform_location(c_str!("shineDamper"));

            self.location_reflectivity =
                self.get_uniform_location(c_str!("reflectivity"));

            self.location_use_fake_lighting =
                self.get_uniform_location(c_str!("useFakeLighting"));

            self.location_sky_colour =
                self.get_uniform_location(c_str!("skyColour"));


            self.location_background_texture =
                self.get_uniform_location(c_str!("backgroundTexture"));

            self.location_r_texture =
                self.get_uniform_location(c_str!("rTexture"));

            self.location_g_texture =
                self.get_uniform_location(c_str!("gTexture"));

            self.location_b_texture =
                self.get_uniform_location(c_str!("bTexture"));

            self.location_blend_map =
                self.get_uniform_location(c_str!("blendMap"));

            self.location_number_of_rows =
                self.get_uniform_location(c_str!("numberOfRows"));

            self.location_offset =
                self.get_uniform_location(c_str!("offset"));

            // ------------- carga a lo bestia ----------------------
            self.location_light_position[0] =
                self.get_uniform_location(c_str!("lightPosition[0]"));
            self.location_light_position[1] =
                self.get_uniform_location(c_str!("lightPosition[1]"));
            self.location_light_position[2] =
                self.get_uniform_location(c_str!("lightPosition[2]"));
            self.location_light_position[3] =
                self.get_uniform_location(c_str!("lightPosition[3]"));

            self.location_light_color[0] =
                self.get_uniform_location(c_str!("lightColour[0]"));
            self.location_light_color[1] =
                self.get_uniform_location(c_str!("lightColour[1]"));
            self.location_light_color[2] =
                self.get_uniform_location(c_str!("lightColour[2]"));
            self.location_light_color[3] =
                self.get_uniform_location(c_str!("lightColour[3]"));

            self.location_light_attenuation[0] =
                self.get_uniform_location(c_str!("attenuation[0]"));
            self.location_light_attenuation[1] =
                self.get_uniform_location(c_str!("attenuation[1]"));
            self.location_light_attenuation[2] =
                self.get_uniform_location(c_str!("attenuation[2]"));
            self.location_light_attenuation[3] =
                self.get_uniform_location(c_str!("attenuation[3]"));
            // -----------------------------------------------------

            self.location_fog_colour =
                self.get_uniform_location(c_str!("fogColour"));


            self.location_cube_map =
                self.get_uniform_location(c_str!("cubeMap"));
            self.location_cube_map2 =
                self.get_uniform_location(c_str!("cubeMap2"));
            self.location_blend_factor =
                self.get_uniform_location(c_str!("blendFactor"));
        }
    }

    pub fn _load_shine_variables(&self, damper: f32, reflectivity: f32) {
        ShaderProgram::load_float(self.location_shine_damper, damper);
        ShaderProgram::load_float(self.location_reflectivity, reflectivity);
    }

    //    pub fn load_light(&self, light: &Light) {
//        ShaderProgram::load_vector(self.location_light_position, light.get_position());
//        ShaderProgram::load_vector(self.location_light_color, light.get_color());
//    }
    pub fn load_float(location: i32, value: f32) {
        unsafe {
            gl::Uniform1f(location, value); //Modifica variable uniform float con value
        }
    }
    pub fn load_int(location: i32, value: i32) {
        unsafe {
            gl::Uniform1i(location, value); //Modifica variable uniform int con value
        }
    }
    pub fn load_vector(location: i32, vector: V3CG) {
        unsafe {
            //Modifica variable 3D uniform location con vector
            gl::Uniform3f(location, vector.x, vector.y, vector.z);
        }
    }

    pub fn load_2d_vector(location: i32, vector: V2CG) {
        unsafe {
            //Modifica variable 3D uniform location con vector
            gl::Uniform2f(location, vector.x, vector.y);
        }
    }

    //Modifica variable  uniform booleana en location con value
    pub fn load_boolean(location: i32, value: bool) {
        let mut to_load: f32 = 0.0;
        if value {
            to_load = 1.0;
        }
        unsafe {
            gl::Uniform1f(location, to_load);
        }
    }

    pub fn load_matrix(location: i32, matrix: &M4CG) {
//        matrix.store(matrixBuffer); //Almacenamos la matriz en el FloatBuffer
//        matrixBuffer.flip();
        //dbg!(location);
        unsafe {

            //Enviamos el buffer con datos al shader
            //gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.data.as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
        }
    }


    pub fn load_shader(filename: &str, tipo: GLenum) -> GLuint {
        let mut fichero = match File::open(filename) {
            Err(error) => panic!("no se puede abrir {:?}: {}", filename, error),
            Ok(f) => f,
        };
        let mut file_data = String::new();
        fichero.read_to_string(&mut file_data);

        unsafe {
            let sh_id = gl::CreateShader(tipo); // crea shader

            let sh_c_str = CString::new(file_data).unwrap();

            // carga lo que habia en fichero
            gl::ShaderSource(sh_id, 1, &sh_c_str.as_ptr(), ptr::null());
            gl::CompileShader(sh_id); // lo compila

            //Comprueba errores
            let mut status: GLint = 0;
            gl::GetShaderiv(sh_id, gl::COMPILE_STATUS, &mut status);
            if status == 0 {
                panic!("ERROR: No se puede compilar shader {:?}", filename);
            }
            sh_id
        }
    }

    pub fn bind_attribute(program_id: GLuint, attribute: GLuint, variable_name: &CStr) {
        // carga una variable atributo en el shader
        // c_str() genera una secuencia de caracteres finalizada con null,
        // con el mismo contenido que el objeto string, y la devuelve como un
        // puntero a un arreglo de char
        unsafe {
            gl::BindAttribLocation(program_id, attribute, variable_name.as_ptr());
        }
    }
    pub fn _load_transformation_matrix(&self, matrix: &M4CG) {
        //dbg!(matrix);
        ShaderProgram::load_matrix(self.location_transformation_matrix, &matrix);
    }

    pub fn _load_view_matrix(&self, camera: &mut Camera) {
        let view_matrix = maths::create_view_matrix(camera);
        //dbg!(&view_matrix);
        ShaderProgram::load_matrix(self.location_view_matrix, &view_matrix);
    }
    pub fn _load_projection_matrix(&self, projection: &M4CG) {
        ShaderProgram::load_matrix(self.location_projection_matrix, &projection);
    }
}