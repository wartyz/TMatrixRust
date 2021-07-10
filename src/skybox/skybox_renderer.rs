use crate::entities::camera::Camera;
use crate::models::raw_model::RawModel;
use crate::render_engine::display_manager::DisplayManager;
use crate::render_engine::loader::Loader;
use crate::skybox::skybox_shader::SkyboxShader;

type M4CG = cgmath::Matrix4<f32>;

pub struct SkyboxRenderer {
    cube: RawModel,
    texture: u32,
    night_texture: u32,
    shader: SkyboxShader,
    time: f32,
}

impl SkyboxRenderer {
    pub fn new(loader: &mut Loader, projection_matrix: M4CG) -> SkyboxRenderer {
        let shader = SkyboxShader::new();
        let size: f32 = 500.0; // Tamaño del cubo
        let texture_files: Vec<&str> = vec![
            "res/textures/right.png", "res/textures/left.png", "res/textures/top.png",
            "res/textures/bottom.png", "res/textures/back.png", "res/textures/front.png"];

        let night_texture_files: Vec<&str> = vec![
            "res/textures/nightRight.png", "res/textures/nightLeft.png",
            "res/textures/nightTop.png", "res/textures/nightBottom.png",
            "res/textures/nightBack.png", "res/textures/nightFront.png"];

        let vertices: Vec<f32> = vec![
            -size, size, -size,
            -size, -size, -size,
            size, -size, -size,
            size, -size, -size,
            size, size, -size,
            -size, size, -size,
            -size, -size, size,
            -size, -size, -size,
            -size, size, -size,
            -size, size, -size,
            -size, size, size,
            -size, -size, size,
            size, -size, -size,
            size, -size, size,
            size, size, size,
            size, size, size,
            size, size, -size,
            size, -size, -size,
            -size, -size, size,
            -size, size, size,
            size, size, size,
            size, size, size,
            size, -size, size,
            -size, -size, size,
            -size, size, -size,
            size, size, -size,
            size, size, size,
            size, size, size,
            -size, size, size,
            -size, size, -size,
            -size, -size, -size,
            -size, -size, size,
            size, -size, -size,
            size, -size, -size,
            -size, -size, size,
            size, -size, size];

        let sk = SkyboxRenderer {
            cube: loader.load_to_vao2(&vertices, 3),
            texture: loader.load_cube_map(texture_files),
            night_texture: loader.load_cube_map(night_texture_files),
            shader,
            time: 0.0,

        };

        shader.start();
        shader.connect_texture_units();
        shader.load_projection_matrix(&projection_matrix);
        shader.stop();

        sk
    }

    pub fn render(&mut self, camera: &mut Camera, r: f32, g: f32, b: f32, dm: &DisplayManager) {
        self.shader.start();
        self.shader.load_view_matrix(camera, dm);
        self.shader.load_fog_colour(r, g, b);
        unsafe {
            gl::BindVertexArray(self.cube.get_vao_id());
            //Activa VAO 0 (vértices).
            gl::EnableVertexAttribArray(0);

            // Renderizamos
            self.bind_textures(dm);

            gl::DrawArrays(gl::TRIANGLES, 0, self.cube.get_vertex_count());
            gl::DisableVertexAttribArray(0);
            gl::BindVertexArray(0);
            self.shader.stop();
        }
    }

    pub fn bind_textures(&mut self, dm: &DisplayManager) {
        self.time += dm.get_frame_time_seconds() * 1000.0;
        self.time %= 24000.0;
        let texture1: u32;
        let texture2: u32;
        let blend_factor: f32;
        if self.time >= 0.0 && self.time < 5000.0 {
            texture1 = self.night_texture;
            texture2 = self.night_texture;
            blend_factor = (self.time - 0.0) / (5000.0 - 0.0);
        } else if self.time >= 5000.0 && self.time < 8000.0 {
            texture1 = self.night_texture;
            texture2 = self.texture;
            blend_factor = (self.time - 5000.0) / (8000.0 - 5000.0);
        } else if self.time >= 8000.0 && self.time < 21000.0 {
            texture1 = self.texture;
            texture2 = self.texture;
            blend_factor = (self.time - 8000.0) / (21000.0 - 8000.0);
        } else {
            texture1 = self.texture;
            texture2 = self.night_texture;
            blend_factor = (self.time - 21000.0) / (24000.0 - 21000.0);
        }


        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture2);
        }
        self.shader.load_blend_factorr(blend_factor);
    }
}