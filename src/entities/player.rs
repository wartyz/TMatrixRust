use cgmath::vec3;
use glfw::{Action, Key, Window};
use glfw::ffi::KEY_ESCAPE;

use crate::entities::entity::Entity;
use crate::models::textured_model::TexturedModel;
use crate::render_engine::display_manager::DisplayManager;
use crate::terrains::terrain::Terrain;
use crate::toolbox::teclado::Teclado;

type V3CG = cgmath::Vector3<f32>;

const RUN_SPEED: f32 = 20.0;
const TURN_SPEED: f32 = 160.0;
const GRAVITY: f32 = -50.0;
const JUMP_POWER: f32 = 30.0;


pub struct Player {
    pub entity: Entity,
    current_speed: f32,
    current_turn_speed: f32,
    upwards_speed: f32,
    is_in_air: bool,
}

impl Player {
    pub fn new(model: TexturedModel, position: V3CG, rotation: V3CG, scale: V3CG) -> Player {
        let id = 0; // Inventado por mi, ojo no poner 0 a otros objetos
        let entity = Entity::new(id, model, position, rotation, scale);
        Player {
            entity,
            current_speed: 0.0,
            current_turn_speed: 0.0,
            upwards_speed: 0.0,
            is_in_air: false, // Para que no llegue al cielo al saltar
        }
    }
    pub fn mover(&mut self, dm: &mut DisplayManager, terrain: &Terrain) {
        self.check_inputs(&mut dm.window);
        self.entity.increase_rotation(
            vec3(0.0, self.current_turn_speed * dm.get_frame_time_seconds(), 0.0));

        let distance: f32 = self.current_speed * dm.get_frame_time_seconds();

        // nueva x,z donde se moverá el player
        let dx: f32 = distance * (self.entity.get_rotation_y().to_radians()).sin();
        let dz: f32 = distance * (self.entity.get_rotation_y().to_radians()).cos();

        self.entity.increase_position(vec3(dx, 0.0, dz));

        self.upwards_speed += GRAVITY * dm.get_frame_time_seconds();
        self.entity.increase_position(
            vec3(0.0, self.upwards_speed * dm.get_frame_time_seconds(), 0.0));

        let terrain_height =
            terrain.get_height_of_terrain(self.entity.get_position().x, self.entity.get_position().z);

        if self.entity.get_position().y < terrain_height {
            self.upwards_speed = 0.0;
            self.is_in_air = false;
            self.entity.set_position_y(terrain_height);
        }
        //println!("Posición del player: {:#?} ", self.entity.get_position());
    }
    pub fn jump(&mut self) {
        if !self.is_in_air {
            self.upwards_speed = JUMP_POWER;
            self.is_in_air = true;
        }
    }
    pub fn check_inputs(&mut self, window: &mut Window) {
        if window.get_key(Key::Escape) == Action::Press {
            window.set_should_close(true)
        }
        if window.get_key(Key::W) == Action::Press {
            self.current_speed = RUN_SPEED;
        } else if window.get_key(Key::S) == Action::Press {
            self.current_speed = -RUN_SPEED;
        } else {
            self.current_speed = 0.0;
        }
        //let cameraSpeed = 2.5 * deltaTime;
        if window.get_key(Key::D) == Action::Press {
            self.current_turn_speed = -TURN_SPEED;
        } else if window.get_key(Key::A) == Action::Press {
            self.current_turn_speed = TURN_SPEED;
        } else {
            self.current_turn_speed = 0.0;
        }

        if window.get_key(Key::Space) == Action::Press {
            self.jump();
        }
    }

    pub fn _check_inputs2(&mut self, window: &mut Window, teclado: &mut Teclado) {
        if window.get_key(Key::Escape) == Action::Press {
            //window.set_should_close(true)
            teclado.presionada_tecla(KEY_ESCAPE);
        }
        //       if window.get_key(Key::W) == Action::Press {
//            self.current_speed = RUN_SPEED;
//        } else if window.get_key(Key::S) == Action::Press {
//            self.current_speed = -RUN_SPEED;
//        } else {
//            self.current_speed = 0.0;
//        }
        //let cameraSpeed = 2.5 * deltaTime;
//            if window.get_key(Key::D) == Action::Press {
//            self.current_turn_speed = -TURN_SPEED;
//        } else if window.get_key(Key::A) == Action::Press {
//            self.current_turn_speed = TURN_SPEED;
//        } else {
//            self.current_turn_speed = 0.0;
//        }
//
//        if window.get_key(Key::Space) == Action::Press {
//            self.jump();
//        }
    }
}