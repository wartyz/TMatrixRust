use cgmath::{InnerSpace, vec2, vec3, vec4};
use cgmath::SquareMatrix;
use cgmath::Transform;

use std::ops::Add;

use crate::entities::camera::Camera;
use crate::render_engine::display_manager::*;
use crate::terrains::terrain::Terrain;
use crate::toolbox::maths::*;
use crate::toolbox::mouse::Mouse;

type P3CG = cgmath::Point3<f32>;
type V2CG = cgmath::Vector2<f32>;
type V3CG = cgmath::Vector3<f32>;
type V4CG = cgmath::Vector4<f32>;
type M4CG = cgmath::Matrix4<f32>;


const RECURSION_COUNT: i32 = 200;
const RAY_RANGE: f32 = 600.0;

pub struct MousePicker {
    current_ray: V3CG,
    projection_matrix: M4CG,
    view_matrix: M4CG,

    terrain: Option<Terrain>,
    current_terrain_point: Option<V3CG>,

}

impl MousePicker {
    pub fn new(camera: &mut Camera, projection_matrix: M4CG, terrain: &Terrain) -> MousePicker {
        MousePicker {
            current_ray: vec3(0.0, 0.0, 0.0),
            projection_matrix,
            view_matrix: create_view_matrix(camera),
            terrain: Some(terrain.clone()),
            current_terrain_point: None,
        }
    }


    pub fn get_current_terrain_point(&self) -> Option<V3CG> {
        self.current_terrain_point
    }
    pub fn get_current_ray(&self) -> V3CG {
        self.current_ray
    }

    pub fn update(&mut self, camera: &mut Camera) {
        self.view_matrix = create_view_matrix(camera);
        self.current_ray = self.calculate_mouse_ray(&camera.mouse);

        if self.intersection_in_range(0.0, RAY_RANGE, self.current_ray, camera) {
            self.current_terrain_point =
                self.binary_search(0, 0.0, RAY_RANGE, self.current_ray, camera);
        } else {
            self.current_terrain_point = None;
        }
    }

    // Pasos hacia atras en la creación de matrices
    pub fn calculate_mouse_ray(&self, mouse: &Mouse) -> V3CG {
        let mouse_x = mouse.get_x();
        let mouse_y = mouse.get_y();
        //dbg!(mouse_x,mouse_y);
        let normalized_coords = self.get_normalized_device_coords(mouse_x, mouse_y);
        let clip_coords = vec4(normalized_coords.x, normalized_coords.y, -1.0, 1.0);
        let eye_coords = self.to_eye_coords(clip_coords);
        let world_ray = self.to_world_coords(eye_coords);
        world_ray
    }

    pub fn to_world_coords(&self, eye_coords: V4CG) -> V3CG {
        let inverted_view = self.view_matrix.invert().unwrap();
        //let ray_world = transform(inverted_view, eye_coords);
        let ray_world =
            inverted_view.transform_vector(vec3(eye_coords.x, eye_coords.y, eye_coords.z));
        let mouse_ray = vec3(ray_world.x, ray_world.y, ray_world.z);
        mouse_ray.normalize();
        mouse_ray
    }

    pub fn to_eye_coords(&self, clip_coords: V4CG) -> V4CG {
        let inverted_projection = self.projection_matrix.invert().unwrap();
        let eye_coords =
            inverted_projection.transform_vector(vec3(clip_coords.x, clip_coords.y, clip_coords.z));

        vec4(eye_coords.x, eye_coords.y, -1.0, 0.0)
    }

    pub fn get_normalized_device_coords(&self, mouse_x: f32, mouse_y: f32) -> V2CG {
        let x = (2.0 * mouse_x) / ANCHO as f32 - 1.0;
        let y = (2.0 * mouse_y) / ALTO as f32 - 1.0;
        vec2(x, y)
    }


    //**********************************************************

    fn get_point_on_ray(&self, ray: V3CG, distance: f32, camera: &Camera) -> V3CG {
        let cam_pos: P3CG = camera.get_position();
        let start: V3CG = vec3(cam_pos.x, cam_pos.y, cam_pos.z);
        let scaled_ray: V3CG = vec3(ray.x * distance, ray.y * distance, ray.z * distance);
        start.add(scaled_ray)
    }

    fn binary_search(
        &self, count: i32, start: f32, finish: f32, ray: V3CG, camera: &Camera) -> Option<V3CG> {
        let half: f32 = start + ((finish - start) / 2.0);
        if count >= RECURSION_COUNT {
            let end_point = self.get_point_on_ray(ray, half, camera);
            let terrain: Option<&Terrain> = self.get_terrain(end_point.x, end_point.z);
            if terrain.is_some() {
                return Some(end_point);
            } else {
                return None;
            }
        }
        if self.intersection_in_range(start, half, ray, camera) {
            return self.binary_search(count + 1, start, half, ray, camera);
        } else {
            return self.binary_search(count + 1, half, finish, ray, camera);
        }
    }

    fn intersection_in_range(&self, start: f32, finish: f32, ray: V3CG, camera: &Camera) -> bool {
        let start_point: V3CG = self.get_point_on_ray(ray, start, camera);
        let end_point: V3CG = self.get_point_on_ray(ray, finish, camera);
        if !self.is_under_ground(start_point) && self.is_under_ground(end_point) {
            return true;
        } else {
            return false;
        }
    }

    fn is_under_ground(&self, test_point: V3CG) -> bool {
        let terrain: Option<&Terrain> = self.get_terrain(test_point.x, test_point.z);
        let mut height: f32 = 0.0;
        if terrain.is_some() {
            height = terrain.unwrap().get_height_of_terrain(test_point.x, test_point.z);
        }
        if (test_point.y < height) {
            return true;
        } else {
            return false;
        }
    }

    fn get_terrain(&self, world_x: f32, world_z: f32) -> Option<&Terrain> {
        self.terrain.as_ref()
    }
}