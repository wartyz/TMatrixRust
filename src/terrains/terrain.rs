use cgmath::{InnerSpace, vec2, vec3, Vector3};

use crate::models::raw_model::RawModel;
use crate::render_engine::loader::Loader;
use crate::textures::terrain_texture::TerrainTexture;
use crate::textures::terrain_texture_pack::TerrainTexturePack;
use crate::toolbox::maths::*;
use crate::toolbox::png_loader::PngLoader;

const SIZE: f32 = 800.0;
const MAX_HEIGHT: f32 = 40.0;
const MAX_PIXEL_COLOUR: f32 = 256.0 * 256.0 * 256.0;
//Esto arregla los bordes de altura de terrains
const GRAIN: f32 = 0.99;

pub struct Terrain {
    x: f32,
    z: f32,
    model: RawModel,
    texture_pack: TerrainTexturePack,
    blend_map: TerrainTexture,
    vertex_count: usize,
    png_loader: PngLoader,
    heights: Vec<Vec<f32>>,
    //alturas: Vec<u8>,
}

impl Clone for Terrain {
    fn clone(&self) -> Self {
        Terrain {
            x: self.x,
            z: self.z,
            model: self.model,
            texture_pack: self.texture_pack,
            blend_map: self.blend_map,
            vertex_count: self.vertex_count,
            png_loader: self.png_loader.clone(),
            heights: self.heights.clone(),
            //alturas: self.alturas,
        }
    }
}

impl Terrain {
    pub fn new(grid_x: i32, grid_z: i32,
               loader: &mut Loader,
               texture_pack: TerrainTexturePack,
               blend_map: TerrainTexture,
               heightmap: &str) -> Terrain {
        let mut t = Terrain {
            x: grid_x as f32 * SIZE,
            z: grid_z as f32 * SIZE,
            model: RawModel::new(0, 0),
            texture_pack,
            blend_map,
            vertex_count: 0,
            png_loader: PngLoader::new(),
            heights: vec![vec![]],

        };

        t.model = t.generate_terrain(loader, heightmap).unwrap();
        t
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_z(&self) -> f32 {
        self.z
    }

    pub fn get_model(&self) -> RawModel {
        self.model
    }

    pub fn get_texture_pack(&self) -> TerrainTexturePack {
        self.texture_pack
    }

    pub fn get_blend_map(&self) -> TerrainTexture {
        self.blend_map
    }

    pub fn get_height_of_terrain(&self, world_x: f32, world_z: f32) -> f32 { // Devuelve altura del
        // player
        // coordenadas x,z relativas en terrain (será 0,0 la esquina superior izquierda y SIZE,SIZE la esq inf der
        let terrain_x = world_x - self.x;
        let terrain_z = world_z - self.z;
        // Tamaño de cuadrado de la malla (-1 porque cuadrados es vertices por lado - 1)
        let grid_square_size = SIZE / (self.heights.len() - 1) as f32;
        // gridX y gridZ son las coordenadas de cuadrados en la malla
        let grid_x = (terrain_x / grid_square_size).floor() as usize;
        let grid_z = (terrain_z / grid_square_size).floor() as usize;

        // Comprueba que no estamos fuera de los límites
        if grid_x >= self.heights.len() - 1 || grid_z >= self.heights.len() - 1 {
            return 0.0;
        }
        let x_coord = (terrain_x % grid_square_size) / grid_square_size;
        let z_coord = (terrain_z % grid_square_size) / grid_square_size;

        // Averiguamos en que triángulo de los dos posibles está el player y dentro del triángulo  la altura del player
        let answer: f32;
        if x_coord <= (1.0 - z_coord) { // Primer triángulo
            answer = barry_centric(vec3(0.0, self.heights[grid_x][grid_z], 0.0),
                                   vec3(1.0, self.heights[grid_x + 1][grid_z], 0.0),
                                   vec3(0.0, self.heights[grid_x][grid_z + 1], 1.0),
                                   vec2(x_coord, z_coord));
        } else { // Segundo triángulo
            answer = barry_centric(vec3(1.0, self.heights[grid_x + 1][grid_z], 0.0),
                                   vec3(1.0, self.heights[grid_x + 1][grid_z + 1], 1.0),
                                   vec3(0.0, self.heights[grid_x][grid_z + 1], 1.0),
                                   vec2(x_coord, z_coord));
        }

        return answer;
    }

    pub fn generate_terrain(&mut self, loader: &mut Loader, height_map: &str)
                            -> Result<RawModel, String> {
        let alturas = self.png_loader.load_image(height_map)?;

        self.vertex_count = self.png_loader.get_height();
        self.heights = vec![vec![0.0; self.vertex_count]; self.vertex_count];

        //let alturas = img.raw_pixels(); //convierte a Vec<u8>

        let count = self.vertex_count * self.vertex_count;

        //vertices[] almacena coordenadas x,y,z
        let mut vertices: Vec<f32> = vec![0.0; count * 3];
        //normals[] almacena normals x,y,z
        let mut normals: Vec<f32> = vec![0.0; count * 3];
        //textureCoords[] almacena  textura x,y
        let mut texture_coords: Vec<f32> = vec![0.0; count * 2];
        let mut indices: Vec<u32> = vec![0; 6 * (self.vertex_count - 1) * (self.vertex_count - 1)];

        let mut vertex_pointer = 0;
        for i in 0..self.vertex_count {
            for j in 0..self.vertex_count {
                vertices[vertex_pointer * 3] = j as f32 / (self.vertex_count - 1) as f32 * SIZE; //X

                let height = self.get_height(j, i, &alturas);
                self.heights[j][i] = height;
                //altura de terrain Y ahora usamos heightMap.png
                vertices[vertex_pointer * 3 + 1] = height;


                vertices[vertex_pointer * 3 + 2] = i as f32 / (self.vertex_count - 1) as f32 * SIZE; //Z

                let normal = self.calculate_normal(j as f32, i as f32, &alturas);//para montañas
                normals[vertex_pointer * 3] = normal.x; //X Normal
                normals[vertex_pointer * 3 + 1] = normal.y; //Y Normal apunta arriba
                normals[vertex_pointer * 3 + 2] = normal.z; //Z Normal

                //X de textura
                texture_coords[vertex_pointer * 2] = j as f32 / (self.vertex_count - 1) as f32;

                //Y de textura
                texture_coords[vertex_pointer * 2 + 1] = i as f32 / (self.vertex_count - 1) as f32;


                vertex_pointer += 1;
            }
        }
        let mut pointer = 0;
        for gz in 0..(self.vertex_count - 1) {
            for gx in 0..(self.vertex_count - 1) {
                let top_left = (gz * self.vertex_count) + gx;
                let top_right = top_left + 1;
                let bottom_left = ((gz + 1) * self.vertex_count) + gx;
                let bottom_right = bottom_left + 1;

                indices[pointer] = top_left as u32;
                pointer += 1;
                indices[pointer] = bottom_left as u32;
                pointer += 1;
                indices[pointer] = top_right as u32;
                pointer += 1;
                indices[pointer] = top_right as u32;
                pointer += 1;
                indices[pointer] = bottom_left as u32;
                pointer += 1;
                indices[pointer] = bottom_right as u32;
                pointer += 1;
            }
        }
        Ok(loader.load_to_vao(&vertices.to_vec(),
                              &texture_coords.to_vec(),
                              &normals.to_vec(),
                              &indices.to_vec()))
    }

    //usado para montañas en terrain, retorna vector normal en una coordenada de terrain
    pub fn calculate_normal(&self, x: f32, z: f32, alturas: &Vec<u8>)
                            -> Vector3<f32> {

        // Calcula las alturas de las coordenadas vecinas
        let height_l = self.get_height((x - GRAIN) as usize, z as usize, &alturas);
        let height_r = self.get_height((x + GRAIN) as usize, z as usize, &alturas);
        let height_d = self.get_height(x as usize, (z - GRAIN) as usize, &alturas);
        let height_u = self.get_height(x as usize, (z + GRAIN) as usize, &alturas);

        // Usando las alturas de los vertices vecinos, calcula el vector normal
        let normal = vec3(height_l - height_r, 2.0, height_d - height_u);
        normal.normalize(); // normaliza vector normal
        return normal;
    }


    // retorna altura de terrain según el color de heightMap.png
    pub fn get_height(&self, x: usize, z: usize, _alturas: &Vec<u8>) -> f32 {

        // Ojo vertex_count es el ancho de la pantalla
        // Salir si fuera de márgenes
        //if x < 0 || x >= self.vertex_count || z < 0 || z >= self.vertex_count {
        if x >= self.vertex_count || z >= self.vertex_count {
            return 0.0;
        }
        //let mut height = image.get_rgb(x, z);

        //let mut height = alturas[x as usize * 4 + z as usize * self.vertex_count] as f32 *
        // 65025.0;
        //let mut g = self.alturas[(x * 3) + 1 + z * self.vertex_count];
        //let mut b = self.alturas[(x * 3) + 2 + z * self.vertex_count];

//        let r = alturas[4 * z as usize * self.vertex_count + 4 * x as usize + 0] as i32;
//        let g = alturas[4 * z as usize * self.vertex_count + 4 * x as usize + 1] as i32;
//        let b = alturas[4 * z as usize * self.vertex_count + 4 * x as usize + 2] as i32;


//        let h = (r << 16) | (g << 8) | (b << 0);
//        let mut height = h as f32;

        let mut height = self.png_loader.get_rgb(x, z);
        //dbg!(height);
        height += MAX_PIXEL_COLOUR / 2.0;
        height /= MAX_PIXEL_COLOUR / 2.0;
        height *= MAX_HEIGHT;

        //let height = 1.0;
        height
    }
}