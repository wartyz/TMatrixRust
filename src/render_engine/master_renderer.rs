use cgmath::vec3;

use crate::entities::camera::Camera;
use crate::entities::entity::Entity;
use crate::entities::light::Light;
use crate::render_engine::display_manager::DisplayManager;
use crate::render_engine::entity_renderer::EntityRenderer;
use crate::render_engine::loader::Loader;
use crate::render_engine::terrain_renderer::TerrainRenderer;
use crate::shaders::static_shader::*;
use crate::shaders::terrain_shader::TerrainShader;
use crate::skybox::skybox_renderer::SkyboxRenderer;
use crate::terrains::terrain::Terrain;
use crate::toolbox::maths::create_projection_matrix_perspective;

type M4CG = cgmath::Matrix4<f32>;

// Color del fondo y la niebla
const RED: f32 = 0.5;
const GREEN: f32 = 1.0;
const BLUE: f32 = 1.0;


pub struct MasterRenderer {
    projection_matrix: M4CG,
    shader: StaticShader,
    terrain_shader: TerrainShader,
    renderer: EntityRenderer,
    terrain_renderer: TerrainRenderer,
    skybox_renderer: SkyboxRenderer,
    entities: Vec<Vec<Entity>>,
    terrains: Vec<Terrain>,
}

impl MasterRenderer {
    pub fn new(dm: &DisplayManager, loader: &mut Loader) -> MasterRenderer {
        MasterRenderer::enable_culling();
        let projection_matrix =
            create_projection_matrix_perspective(dm.width as f32, dm.height as f32);
        let shader = StaticShader::new();
        let terrain_shader = TerrainShader::new();

        MasterRenderer {
            projection_matrix,
            shader,
            terrain_shader,
            renderer: EntityRenderer::new(shader, &projection_matrix),
            terrain_renderer: TerrainRenderer::new(terrain_shader, projection_matrix),
            skybox_renderer: SkyboxRenderer::new(loader, projection_matrix),
            entities: MasterRenderer::inicializa_entities(),
            terrains: vec![],
            //nombre_vector,
        }
    }

    // Una variable por cada tipo de objeto, se debe modificar si modificamos el numero
    fn inicializa_entities() -> Vec<Vec<Entity>> {
        let aa: Vec<Entity> = vec![];
        let bb: Vec<Entity> = vec![];
        let cc: Vec<Entity> = vec![];
        let dd: Vec<Entity> = vec![];
        let ee: Vec<Entity> = vec![];
        let ff: Vec<Entity> = vec![];
        let gg: Vec<Entity> = vec![];
        let hh: Vec<Entity> = vec![];
        let ii: Vec<Entity> = vec![];
        vec![aa, bb, cc, dd, ee, ff, gg, hh, ii]
    }

    pub fn get_projection_matrix(&self) -> M4CG {
        self.projection_matrix
    }

    pub fn enable_culling() {
        unsafe {
            gl::Enable(gl::CULL_FACE); //Caras posteriores no se ven
            gl::CullFace(gl::BACK);
        }
    }

    pub fn disable_culling() {
        unsafe {
            gl::Disable(gl::CULL_FACE);
        }
    }
    // renderiza antes de presentarlo en pantalla
    pub fn render(&mut self, lights: &Vec<Light>, camera: &mut Camera, dm: &DisplayManager) {
        self.prepare();

        self.shader.start();
        self.shader.load_sky_colour(RED, GREEN, BLUE);
        self.shader.load_lights(lights);
        self.shader.load_view_matrix(camera);
        self.renderer.render(&self.entities); //Renderizamos entities
        self.shader.stop();


        self.terrain_shader.start();
        self.terrain_shader.load_sky_colour(RED, GREEN, BLUE);
        self.terrain_shader.load_lights(lights);
        self.terrain_shader.load_view_matrix(camera);
        self.terrain_renderer.render(&self.terrains);
        self.terrain_shader.stop();

        self.skybox_renderer.render(camera, RED, GREEN, BLUE, dm);

        self.terrains.clear();


        self.entities = MasterRenderer::inicializa_entities(); // En vez de entities.clear()
    }

    pub fn copia_entity(entity: &Entity) -> Entity {
        let rotacion =
            vec3(entity.get_rotation_x(), entity.get_rotation_y(), entity.get_rotation_z());
        let escala = vec3(1.0, 1.0, 1.0);
        Entity::new(entity.id, entity.get_model(), entity.get_position(), rotacion, escala)
    }

    pub fn process_terrain(&mut self, terrain: &Terrain) {
        self.terrains.push(terrain.clone());
    }


    // Un numero por cada tipo de objeto, se debe modificar si modificamos el numero de objetos
    // máximo hay 9 objetos
    pub fn process_entity(&mut self, entity: &Entity) {
        let id = entity.id;
        //dbg!(id,tam);
        match id {
            0 => self.entities[0].push(entity.clone()),

            1 => self.entities[1].push(entity.clone()),
            2 => self.entities[2].push(entity.clone()),
            3 => self.entities[3].push(entity.clone()),
            4 => self.entities[4].push(entity.clone()),
            5 => self.entities[5].push(entity.clone()),

            // Lamparas
            6 => self.entities[6].push(entity.clone()),
            7 => self.entities[7].push(entity.clone()),
            8 => self.entities[8].push(entity.clone()),

            _ => println!("Falta id entity en master_renderer"),
        }
    }

    pub fn cleanup(&mut self) {
        self.shader.cleanup();
        self.terrain_shader.cleanup();
    }

    //Llamado una vez cada frame. Prepara OpenGL para renderizar el juego.
    pub fn prepare(&mut self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(RED, GREEN, BLUE, 1.0);
        }
    }
}