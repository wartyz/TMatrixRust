use cgmath::{vec2, vec3};
use rand::Rng;

use crate::entities::camera::Camera;
use crate::entities::entity::Entity;
use crate::entities::light::Light;
use crate::entities::player::Player;
use crate::guis::gui_renderer::GuiRenderer;
use crate::guis::gui_texture::GuiTexture;
use crate::models::raw_model::RawModel;
use crate::models::textured_model::TexturedModel;
use crate::render_engine::display_manager::DisplayManager;
use crate::render_engine::loader::Loader;
use crate::render_engine::master_renderer::MasterRenderer;
use crate::render_engine::objloader::OBJLoader;
use crate::terrains::terrain::Terrain;
use crate::textures::model_texture::ModelTexture;
use crate::textures::terrain_texture::TerrainTexture;
use crate::textures::terrain_texture_pack::TerrainTexturePack;
use crate::toolbox::mouse_picker::MousePicker;

type V3CG = cgmath::Vector3<f32>;

pub struct MainGameLoop {
    dm: DisplayManager,
    renderer: MasterRenderer,
    gui_renderer: GuiRenderer,
    guis: Vec<GuiTexture>,
    model: RawModel,
    loader: Loader,
    camera: Camera,
    lights: Vec<Light>,
    terrain: Terrain,
    //terrain2: Terrain,
    entities: Vec<Entity>,
    player: Player,
    picker: MousePicker,

}

impl MainGameLoop {
    pub fn new() -> MainGameLoop {
        let mut dm = DisplayManager::new();
        dm.create_display();

        let mut loader = Loader::new();
// ----------------------------- TERRAIN TEXTURE STUFF -----------------------------------------

        let background_texture =
            TerrainTexture::new(loader.load_texture("res/textures/grassy.png").unwrap());
        let r_texture =
            TerrainTexture::new(loader.load_texture("res/textures/dirt.png").unwrap());
        let g_texture =
            TerrainTexture::new(loader.load_texture("res/textures/pinkFlowers.png").unwrap());
        let b_texture =
            TerrainTexture::new(loader.load_texture("res/textures/path.png").unwrap());

        let texture_pack = TerrainTexturePack::new(background_texture, r_texture, g_texture, b_texture);
        let blend_map =
            TerrainTexture::new(loader.load_texture("res/textures/blendMap.png").unwrap());

        let terrain =
            Terrain::new(0, -1, &mut loader, texture_pack.clone(), blend_map.clone(),
                         "res/textures/heightmap.png");
// ----------------------------- player 0 ------------------------------------------------------
        let mut obj_loader = OBJLoader::new();
        let model = obj_loader.load_obj_model("res/models/stanfordBunny.obj", &mut loader);

        let stanford_bunny =
            TexturedModel::new(model, ModelTexture::new(loader.load_texture
            ("res/textures/white.png").unwrap()));
// ----------------------------- arbol 1 -------------------------------------------------------
        let mut obj_loader = OBJLoader::new();
        let model = obj_loader.load_obj_model("res/models/tree.obj", &mut loader);

        let static_model =
            TexturedModel::new(model, ModelTexture::new(loader.load_texture
            ("res/textures/tree.png").unwrap()));

// ----------------------------- hierbas 2 -----------------------------------------------------
        let mut obj_loader = OBJLoader::new();
        let model = obj_loader.load_obj_model("res/models/grassModel.obj", &mut loader);

        let grass =
            TexturedModel::new(model, ModelTexture::new(loader.load_texture
            ("res/textures/grassTexture.png").unwrap()));

        grass.get_texture().set_has_transparency(true);
        grass.get_texture().set_use_fake_lighting(true);

// ----------------------------- helecho 3 -- (atlas) ------------------------------------------
        let mut obj_loader = OBJLoader::new();
        let model = obj_loader.load_obj_model("res/models/fern.obj", &mut loader);

        let mut fern_texture_atlas = ModelTexture::new(loader.load_texture(
            "res/textures/fern.png").unwrap()); // hojas (atlas)

        fern_texture_atlas.set_number_of_rows(2);

        let fern = TexturedModel::new(model, fern_texture_atlas);

        fern.get_texture().set_has_transparency(true);
// ----------------------------- low_poly_tree 4 -----------------------------------------------
        let mut obj_loader = OBJLoader::new();
        let model = obj_loader.load_obj_model("res/models/lowPolyTree.obj", &mut loader);

        let low_poly_tree =
            TexturedModel::new(model, ModelTexture::new(loader.load_texture
            ("res/textures/lowPolyTree.png").unwrap()));
// ----------------- flores, usa el mismo obj que hierbas pero otra textura 5 ------------------
        let mut obj_loader = OBJLoader::new();
        let model = obj_loader.load_obj_model("res/models/grassModel.obj", &mut loader);

        let grass =
            TexturedModel::new(model, ModelTexture::new(loader.load_texture
            ("res/textures/flower.png").unwrap()));

        grass.get_texture().set_has_transparency(true);
        grass.get_texture().set_use_fake_lighting(true);
// ---------------------------------------- lampara --------------------------------------------
        let mut obj_loader = OBJLoader::new();
        let model = obj_loader.load_obj_model("res/models/lamp.obj", &mut loader);

        let lamp =
            TexturedModel::new(model, ModelTexture::new(loader.load_texture
            ("res/textures/lamp.png").unwrap()));
// ---------------------------------------------------------------------------------------------
        let mut entities: Vec<Entity> = vec![];
        let mut rng = rand::thread_rng();

        //crea 500 arboles, helechos y hierbajos, ojo salteados
        for i in 0..500 {
            if i % 20 == 0 { // 25 arboles
                // ------------------------- arbol ------------------------------------
                let x = rng.gen_range(0.0, 1.0) * 800.0 - 400.0;
                let z = rng.gen_range(0.0, 1.0) * -600.0;
                let y = terrain.get_height_of_terrain(x, z);

                entities.push(Entity::new(1,                     // ID, creado por mi (player = 0)
                                          static_model,          // arbol
                                          vec3(x, y, z),         // Posición
                                          vec3(0.0, 0.0, 0.0),   // Rotación
                                          vec3(8.0, 8.0, 8.0))); // Escala
            }
            if i % 5 == 0 { // 100 hierbas
                // ------------------------- hierbas -----------------------------------
                let x = rng.gen_range(0.0, 1.0) * 800.0 - 400.0;
                let z = rng.gen_range(0.0, 1.0) * -600.0;
                let y = terrain.get_height_of_terrain(x, z);
                entities.push(Entity::new(2,                    // ID, creado por mi
                                          grass,                // hierbas
                                          vec3(x, y, z),        // Posición
                                          vec3(0.0, 0.0, 0.0),  // Rotación
                                          vec3(1.0, 1.0, 1.0)));// Escala
            }

            // ------------------------- helecho -----------------------------------
            let x = rng.gen_range(0.0, 1.0) * 800.0 - 400.0;
            let z = rng.gen_range(0.0, 1.0) * -600.0;
            let y = terrain.get_height_of_terrain(x, z);
            entities.push(Entity::new2(3,                    // ID, creado por mi
                                       fern,                 // helecho
                                       rng.gen_range(0, 4),
                                       vec3(x, y, z),        // Posición
                                       vec3(0.0, 0.0, 0.0),  // Rotación
                                       vec3(1.0, 1.0, 1.0)));// Escala
            // ------------------------- low_poly_tree ---------------------------
            if i % 50 == 0 { // 10 low_poly_tree
                let x = rng.gen_range(0.0, 1.0) * 800.0 - 400.0;

                let z = rng.gen_range(0.0, 1.0) * -600.0;
                let y = terrain.get_height_of_terrain(x, z);
                entities.push(Entity::new(4,                    // ID, creado por mi
                                          low_poly_tree,                 // helecho
                                          vec3(x, y, z),        // Posición
                                          vec3(0.0, 0.0, 0.0),  // Rotación
                                          vec3(3.0, 3.0, 3.0)));// Escala
            }
            // ------------------------- flores -----------------------------------
            let x = rng.gen_range(0.0, 1.0) * 800.0 - 400.0;
            let z = rng.gen_range(0.0, 1.0) * -600.0;
            let y = terrain.get_height_of_terrain(x, z);
            entities.push(Entity::new(5,                    // ID, creado por mi
                                      grass,                // hierbas
                                      vec3(x, y, z),        // Posición
                                      vec3(0.0, 0.0, 0.0),  // Rotación
                                      vec3(1.0, 1.0, 1.0)));// Escala
        }
        //texture.set_shine_damper(10.0);
        //texture.set_reflectivity(1.0);

        let mut _texture = static_model.get_texture();
// -------------------------- LUCES --- 6,7,8 ------------------------------------------------
        // Luz x = IZQU/DERE   ---  y = ARRI/ABAJ   ---   z = CERCA/LEJOS
        let mut lights: Vec<Light> = vec![];
        let light1 = Light::new(0, vec3(0.0, 10000.0, -7000.0), vec3(0.4, 0.4, 0.4));

        // luces: posicion,color y atenuación

        let light2 =
            Light::new2(1, vec3(185.0, 100.0, -293.0), vec3(2.0, 0.0, 0.0), vec3(1.0, 0.01, 0.002));
        let light3 =
            Light::new2(2, vec3(370.0, 100.0, -300.0), vec3(0.0, 2.0, 2.0), vec3(1.0, 0.01, 0.002));
        let light4 =
            Light::new2(3, vec3(293.0, 100.0, -305.0), vec3(2.0, 2.0, 0.0), vec3(1.0, 0.01, 0.002));


        lights.push(light1);
        lights.push(light2);
        lights.push(light3);
        lights.push(light4);

// -- Objetos lámpara --
        let x = 185.0;
        let z = -293.0;
        let y = terrain.get_height_of_terrain(x, z);

        entities.push(Entity::new(6,                    // ID, creado por mi
                                  lamp,                // lámpara
                                  vec3(x, y, z),        // Posición
                                  vec3(0.0, 0.0, 0.0),  // Rotación
                                  vec3(1.0, 1.0, 1.0)));// Escala


        let x = 370.0;
        let z = -300.0;
        let y = terrain.get_height_of_terrain(x, z);
        entities.push(Entity::new(7,                    // ID, creado por mi
                                  lamp,                // lámpara
                                  vec3(x, y, z),        // Posición
                                  vec3(0.0, 0.0, 0.0),  // Rotación
                                  vec3(1.0, 1.0, 1.0)));// Escala
        let x = 293.0;
        let z = -305.0;
        let y = terrain.get_height_of_terrain(x, z);
        entities.push(Entity::new(8,                    // ID, creado por mi
                                  lamp,                // lámpara
                                  vec3(x, y, z),        // Posición
                                  vec3(0.0, 0.0, 0.0),  // Rotación
                                  vec3(1.0, 1.0, 1.0)));// Escala


// ---------------------------- player y cámara -----------------------------------------

        let player = Player::new(stanford_bunny,
                                 vec3(0.0, 0.0, 0.0),
                                 vec3(0.0, 0.0, 0.),
                                 vec3(1.0, 1.0, 1.0));


        let mut camera = Camera::new();
// ------------------------- GUI --------------------------------------------------------
        let mut guis: Vec<GuiTexture> = vec![];
        let gui = GuiTexture::new(loader.load_texture("res/textures/Marmntrans.png").unwrap(),
                                  vec2(-0.8, -0.5), vec2(0.2, 0.4));
        let gui2 = GuiTexture::new(loader.load_texture("res/textures/Barcelona.png").unwrap(),
                                   vec2(0.8, 0.8), vec2(0.05, 0.1));
        guis.push(gui);
        guis.push(gui2);

        let gui_renderer = GuiRenderer::new(&mut loader);
// --------------------------------------------------------------------------------------
        let renderer = MasterRenderer::new(&dm, &mut loader);
// ------------------------------ Para picar con el ratón -------------------------------
        let picker = MousePicker::new(&mut camera, renderer.get_projection_matrix(), &terrain);

        MainGameLoop {
            dm,
            renderer,
            gui_renderer,
            guis,
            model,
            loader,
            camera,
            lights,
            terrain,
            //terrain2,
            entities,
            player,
            picker,

        }
    }

    pub fn main_game_loop(&mut self) {
        while !self.dm.window.should_close() {
            //self.dm.procesa_eventos(&self.dm.events, self.camera);
            //self.entity.increase_position(vec3(0.01, 0.0, 0.0));
            //self.entity.increase_rotation(vec3(0.0, 1.0, 0.0));
            //self.entity.increase_position(vec3(0.05, 0.0, 0.0));
            //self.camera.mover(&mut self.dm.window);
            self.camera.mover_camara(&self.player);
            self.player.mover(&mut self.dm, &self.terrain);

            self.picker.update(&mut self.camera);
            let terrain_point: Option<V3CG> = self.picker.get_current_terrain_point();
            if terrain_point.is_some() {
                let p_terrain = terrain_point.unwrap();

                for lampara in &mut self.entities {
                    if lampara.id == 6 {
                        lampara.set_position(p_terrain);
                        self.lights[1].set_position(vec3(p_terrain.x, p_terrain.y + 15.0, p_terrain.z));
                    }
                }
            } else {
                //dbg!(self.picker.get_current_ray());
            }

            self.renderer.process_entity(&self.player.entity);

            self.renderer.process_terrain(&self.terrain);
            //self.renderer.process_terrain(&self.terrain2);
            for entity in &mut self.entities {
                self.renderer.process_entity(entity);
            }
            //self.renderer.process_entity(&self.entity);
            //self.renderer.prepare();
            //self.shader.start();

            //self.shader.load_view_matrix(&mut self.camera);
            // lógica del juego
            //self.shader.load_light(&self.light);
//            for cube in &mut self.all_cubes {
//                self.renderer.process_entity(cube);
//            }
            self.renderer.render(&self.lights, &mut self.camera, &self.dm);
            self.gui_renderer.render(&self.guis);

            //self.shader.stop();
            self.dm.update_display(&mut self.camera);
        }
        self.gui_renderer.cleanup();
        self.renderer.cleanup();
        self.loader.cleanup();
        self.loader.unbind_vao();
        self.dm.close_display();
    }
}
