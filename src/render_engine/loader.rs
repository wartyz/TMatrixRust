use gl;
use gl::types::*;
use image::ColorType;
use image::GenericImageView;

use std::mem;
use std::os::raw::c_void;
use std::path::Path;

use crate::models::raw_model::RawModel;
use crate::textures::texture_data::TextureData;

//use std::ptr;

pub struct Loader {
    raw_model: Option<RawModel>,
    vaos: Vec<GLuint>,
    vbos: Vec<GLuint>,
    textures: Vec<GLuint>,
}

impl Loader {
    pub fn new() -> Loader {
        let raw_model: Option<RawModel> = None;
        Loader {
            raw_model,
            vaos: vec![],
            vbos: vec![],
            textures: vec![],
        }
    }

    pub fn load_to_vao(&mut self,
                       positions: &Vec<f32>,
                       texture_coords: &Vec<f32>,
                       normals: &Vec<f32>,
                       indices: &Vec<u32>) -> RawModel {
        let vao_id = self.create_vao(); //Crea VAO y almacena ID en vaoID.
        self.bind_indices_vbo(indices);


        //almacena datos posicionales en lista de atributos
        self.store_data_in_attribute_list(0, 3, positions);
        //almacena datos u,v de textura en lista de atributos
        self.store_data_in_attribute_list(1, 2, texture_coords);
        //almacena datos de normals en lista de atributos
        self.store_data_in_attribute_list(2, 3, normals);

        self.unbind_vao();
        //dbg!(positions.len() * std::mem::size_of::<f32>());
        self.raw_model = Some(RawModel::new(vao_id, indices.len() as i32));
        self.raw_model.unwrap()
    }
    pub fn load_to_vao2(&mut self, positions: &Vec<f32>, dimensions: i32) -> RawModel {
        let vao_id = self.create_vao(); //Crea VAO y almacena ID en vaoID.
        self.store_data_in_attribute_list(0, dimensions, positions);
        self.unbind_vao();
        self.raw_model = Some(RawModel::new(vao_id, positions.len() as i32 / dimensions));
        self.raw_model.unwrap()
    }

    pub fn load_texture(&mut self, path: &str) -> Result<u32, String> {// read image data
        let img = image::open(&Path::new(path)).map_err(|e| format!("Could not load texture {}", e))?;

        let format = match img.color() {
            ColorType::RGB(_) => gl::RGB,
            ColorType::RGBA(_) => gl::RGBA,
            ColorType::Gray(_) => gl::DEPTH_COMPONENT,
            ColorType::GrayA(_) => gl::DEPTH_STENCIL,
            ColorType::BGR(_) => gl::BGR,
            ColorType::BGRA(_) => gl::BGRA,
            ColorType::Palette(_) => gl::DEPTH_COMPONENT,
        };

        unsafe {
            // Mipmapping
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER,
                              gl::LINEAR_MIPMAP_LINEAR as GLint);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_LOD_BIAS, -0.4);
            // initialize texture


            let mut texture = 0;
            gl::GenTextures(1, &mut texture);

            // set the texture wrapping parameters
            // set texture wrapping to GL_REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            // transfer image data
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as GLint,
                img.width() as GLint,
                img.height() as GLint,
                0,
                format,
                gl::UNSIGNED_BYTE,
                &img.raw_pixels()[0] as *const u8 as *const c_void,
            );

            // generate all mip map images for us
            self.textures.push(texture);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            Ok(texture)
        }
    }

    pub fn load_cube_map(&mut self, texture_files: Vec<&str>) -> u32 {
        let mut tex_id = 0;

        unsafe {
            gl::GenTextures(1, &mut tex_id);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, tex_id);

            for i in 0..texture_files.len() {
                let data: TextureData =
                    self.decode_texture_file(texture_files[i as usize]).unwrap();

                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32, // Ojo cambia el #define al sumar 1
                    0,
                    gl::RGB as GLint, // Ojo RGB por raw_pixels()
                    data.get_width() as i32,
                    data.get_height() as i32,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    &data.get_buffer()[0] as *const u8 as *const c_void);
            }

            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            self.textures.push(tex_id);

            // En los comentarios del video dice que hay tarjetas de video que necesita estas 2 líneas
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        }

        tex_id
    }
    // Devuelve un Vec<u8> del fichero gráfico envuelto en TextureData
    fn decode_texture_file(&self, path: &str) -> Result<TextureData, String> {
        let img = image::open(&Path::new(path)).map_err(|e| format!("Could not load texture {}", e))?;
        let width = img.width();
        let height = img.height();
        let data: Vec<u8> = img.raw_pixels();
        //let data = &img.raw_pixels()[0] as *const u8 as *const c_void;


        Ok(TextureData::new(data, width, height))
    }

    fn create_vao(&mut self) -> u32 {
        let mut vao_id: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
            //Añade vaoID al list VAO
            self.vaos.push(vao_id);

            //Activa el VAO
            gl::BindVertexArray(vao_id);
        }
        //dbg!(vao_id);
        vao_id
    }
    pub fn cleanup(&mut self) {}

    pub fn store_data_in_attribute_list(&mut self,
                                        attribute_number: GLuint,
                                        coordenate_size: i32,
                                        data: &Vec<GLfloat>) {
        unsafe {
            //Crea un VBO vacio
            let mut vbo_id = 0;
            gl::GenBuffers(1, &mut vbo_id);
            //dbg!(vbo_id);

            //Añade vboID al vbo list.
            self.vbos.push(vbo_id);

            //Activa VBO. Ahora se puede almacenar datos en el.
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id); // enlaza buffer de vertices

            // Almacena datos en el VBO.
            // Tamaño de los datos en bytes tomando el tamaño de un f32 con std::mem::size_of::<f32>(),
            // multiplicándolo por el número de elementos en el vector (data.len()),
            // y luego forzándolo al entero gl::types::GLsizeiptr
            let vert_size = (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;

            // Obtenemos un puntero vacío a los datos mediante la función .as_ptr(),
            // pero nos devuelve *const f32, mientras que OpenGL necesita *const GLvoid
            //data.as_ptr() as *const GLvoid, // puntero a datos
            let vert_ref = &data[0] as *const f32 as *const c_void;
            //dbg!(data.len());

            gl::BufferData(
                gl::ARRAY_BUFFER, // objetivo
                vert_size,
                vert_ref,
                gl::STATIC_DRAW,
            ); // buffer -> GL_ARRAY


            //dbg!(attribute_number);
            //Pone el VBO en el VAO
            gl::VertexAttribPointer(
                // índice del atributo de vtces genérico("layout (location = 0)")
                attribute_number,
                coordenate_size,                // número de componentes del atributo de vértices genérico
                gl::FLOAT,        // tipo de datos
                gl::FALSE,        // normalizado (conversión int-a-float )
                // stride (separación en bytes entre attributos consecutivos)
                //3 * mem::size_of::<GLfloat>() as GLsizei,
                0,
                std::ptr::null(), // desplazamiento del primer componente
            );

            //Desenlaza el VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn unbind_vao(&mut self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    //Carga el buffer de indices = lo enlaza con el VAO que vamos a renderizar.
    fn bind_indices_vbo(&mut self, indices: &Vec<GLuint>) {
        unsafe {
            let indices_ref = &indices[0] as *const u32 as *const c_void;

            let indices_size = (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;
            //dbg!(indices.len());
            //Creamos VBO vacío
            let mut vbo_id = 0;

            gl::GenBuffers(1, &mut vbo_id);
            //Añadimos a la lista de VBO
            self.vbos.push(vbo_id); // enlaza buffer de indices
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo_id);

            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           indices_size,
                           indices_ref,// puntero a datos,
                           gl::STATIC_DRAW);
        }
    }
}