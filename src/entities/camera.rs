use cgmath;
use cgmath::{point3, vec3};
use cgmath::prelude::*;

use crate::entities::player::Player;
use crate::toolbox::mouse::Mouse;

pub const PI: f32 = std::f64::consts::PI as f32;

type P3CG = cgmath::Point3<f32>;
type V3CG = cgmath::Vector3<f32>;

pub struct Camera {
    distance_from_player: f32,
    angle_around_player: f32,

    pub position: P3CG,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    // los pongo yo
    pub front: V3CG,
    pub right: V3CG,
    pub up: V3CG,
    pub world_up: V3CG,
    pub zoom: f32,

    pub mouse: Mouse,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            distance_from_player: 50.0,
            angle_around_player: 0.0,
            position: point3(0.0, 10.0, 20.0),
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
            front: vec3(0.0, 0.0, -1.0),
            right: V3CG::zero(),
            up: V3CG::zero(),
            world_up: V3CG::unit_y(),
            zoom: 45.0,
            mouse: Mouse::new(),
        }
    }

    pub fn mover_camara(&mut self, player: &Player) {
        //dbg!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        self.mouse.update_mouse_info();
        self.calculate_zoom();
        self.calculate_pitch();
        self.calculate_angle_around_player();
        let horizontal_distance = self.calculate_horizontal_distance();
        let vertical_distance = self.calculate_vertical_distance();
        self.calculate_camera_position(horizontal_distance, vertical_distance, player);
        self.yaw = 180.0 - (player.entity.get_rotation_y() + self.angle_around_player);
//        if window.get_key(Key::Escape) == Action::Press {
//            window.set_should_close(true)
//        }

        //let cameraSpeed = 2.5 * deltaTime;
//        if window.get_key(Key::W) == Action::Press {
//            self.position.z -= 0.2;
//        }
//        if window.get_key(Key::S) == Action::Press {
//            self.position.z += 0.2;
//        }
//        if window.get_key(Key::A) == Action::Press {
//            self.position.x -= 0.2;
//        }
//        if window.get_key(Key::D) == Action::Press {
//            self.position.x += 0.2;
//        }
//        if window.get_key(Key::Space) == Action::Press {
//            self.position.y += 0.2;
//        }
//        if window.get_key(Key::LeftShift) == Action::Press {
//            self.position.y -= 0.2;
//        }
    }

    pub fn get_position(&self) -> P3CG {
        self.position
    }

    pub fn get_pitch(&self) -> f32 {
        self.pitch
    }

    pub fn get_yaw(&self) -> f32 {
        self.yaw
    }

    pub fn _get_roll(&self) -> f32 {
        self.roll
    }

    pub fn calculate_camera_position(&mut self, horiz_distance: f32, vertic_distance: f32,
                                     player: &Player) {
        let theta = player.entity.get_rotation_y() + self.angle_around_player;
        let zoom_level = self.mouse.get_d_wheel() * 0.1;
        self.distance_from_player -= zoom_level;
        let offset_x = horiz_distance * theta.to_radians().sin();
        let offset_z = horiz_distance * theta.to_radians().cos();
        //dbg!(self.position);
        self.position.x = player.entity.get_position().x - offset_x;
        self.position.z = player.entity.get_position().z - offset_z;
        self.position.y = player.entity.get_position().y + vertic_distance;
    }


    pub fn calculate_horizontal_distance(&self) -> f32 {
        self.distance_from_player * (self.pitch * (PI / 180.0)).cos()
    }

    pub fn calculate_vertical_distance(&self) -> f32 {
        self.distance_from_player * (self.pitch * (PI / 180.0)).sin()
    }

    pub fn calculate_zoom(&mut self) {
        let zoom_level: f32 = self.mouse.get_d_wheel() * 0.1;
        self.distance_from_player -= zoom_level;
    }

    // 1 es botón izquierdo del ratón
    pub fn calculate_pitch(&mut self) {
        if self.mouse.get_mouse_buttons() == 1 {
            let pitch_change = self.mouse.get_dy() * 0.2;
            self.pitch -= pitch_change;
        }
    }
    // 2 es botón derecho del ratón
    pub fn calculate_angle_around_player(&mut self) {
        if self.mouse.get_mouse_buttons() == 2 {
            let angle_change = -self.mouse.get_dx() * 0.3;
            self.angle_around_player -= angle_change;
        }
    }
}