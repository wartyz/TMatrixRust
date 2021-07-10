use cgmath;
use cgmath::{vec2, vec3};
//use cgmath::prelude::*;
use gl::types::*;

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::models::raw_model::RawModel;
use crate::render_engine::loader::Loader;

//use std::io::prelude::*;
//use std::path::Path;


type V2CG = cgmath::Vector2<f32>;
type V3CG = cgmath::Vector3<f32>;

pub struct OBJLoader {
    vertices: Vec<V3CG>,
    textures: Vec<V2CG>,
    normals: Vec<V3CG>,
    indices: Vec<GLuint>,
    vertices_array: Vec<GLfloat>,
    normals_array: Vec<GLfloat>,
    texture_array: Vec<GLfloat>,
    indices_array: Vec<GLuint>,
}

impl OBJLoader {
    pub fn new() -> OBJLoader {
        OBJLoader {
            vertices: vec![],
            textures: vec![],
            normals: vec![],
            indices: vec![],

            vertices_array: vec![],
            normals_array: vec![],
            texture_array: vec![],
            indices_array: vec![],
        }
    }
    pub fn load_obj_model(&mut self, filename: &str, loader: &mut Loader) -> RawModel {
        let fichero = match File::open(filename) {
            Err(error) => panic!("no se puede abrir {:?}: {}", filename, error),
            Ok(f) => f,
        };
        let reader = BufReader::new(fichero);

        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (_index, line) in reader.lines().enumerate() {
            let line: String = line.unwrap(); // Ignore errors.
            match line.get(..2) {
                Some("o ") => println!("{}", line),
                Some("v ") => {
                    let mut kkk = line.split_whitespace();
                    //println!("VVVVVVVVVVVVVVVVVVVV");
                    kkk.next();
                    let x: f32 = kkk.next().unwrap().parse().unwrap();
                    let y: f32 = kkk.next().unwrap().parse().unwrap();
                    let z: f32 = kkk.next().unwrap().parse().unwrap();
                    let vertex = vec3(x, y, z);
                    self.vertices.push(vertex);
                    //println!("vertices = {:#?}", self.vertices);
                    // pongo aqui esto por mi parte
                    for _ in 0..2 {
                        self.texture_array.push(0.0);
                    }
                    for _ in 0..3 {
                        self.normals_array.push(0.0);
                    }
                }
                Some("vt") => {
                    let mut kkk = line.split_whitespace();
                    //println!("VTVTVTVTVT");
                    kkk.next();
                    let x: f32 = kkk.next().unwrap().parse().unwrap();
                    let y: f32 = kkk.next().unwrap().parse().unwrap();
                    let texture = vec2(x, y);
                    self.textures.push(texture);
                    //println!("textures = {:#?}", self.textures);
                }
                Some("vn") => {
                    let mut kkk = line.split_whitespace();
                    //println!("VNVNVNVNVN");
                    kkk.next();
                    let x: f32 = kkk.next().unwrap().parse().unwrap();
                    let y: f32 = kkk.next().unwrap().parse().unwrap();
                    let z: f32 = kkk.next().unwrap().parse().unwrap();
                    let normal = vec3(x, y, z);
                    self.normals.push(normal);
                    //println!("normals = {:#?}", self.normals);
                }
                Some("f ") => {
                    //println!("Caraaaaaaaaaaa");

//                    // No entiendo porque debo poner ..= en un objeto
//                    for _ in 0..=self.vertices.len() {
//                        self.texture_array.push(0.0);
//                        self.normals_array.push(0.0);
//                    }
                    //println!("line = {:#?}", line);

                    // v es un vector de 4 elementos. por ej: v[0]= "f",
                    // v[1] = "3/2/1", v[2] = "2/2/1", v[3] = "3/3/1"
                    let v: Vec<&str> = line.split_whitespace().collect();
                    //println!("v = {:#?}", v);


                    for i in 1..4 {
                        // vv es un vector de 3 elementos. por ej:
                        // vv[0]=3, vv[1]=2, vv[2]=1
                        let vv: Vec<&str> = v[i].split("/").collect();
                        //println!("vv = {:#?}", vv);

                        self.process_vertex(vv);//vértice 1 de triángulo
                    }


                    //break;
                }
                _ => println!("{}", line),
            };
        }
        for vertex in self.vertices.iter() {
            self.vertices_array.push(vertex.x);
            self.vertices_array.push(vertex.y);
            self.vertices_array.push(vertex.z);
        }

        for indice in self.indices.iter() {
            self.indices_array.push(*indice);
        }


        //println!("vertices_array.len = {}", self.vertices_array.len());
        //println!("vertices.len = {}", self.vertices.len());
        //println!("indices.len = {}", self.indices.len());
        //println!("indices = {:#?}", self.indices);
        // Show the line and its number.

        loader.load_to_vao(
            &self.vertices_array,
            &self.texture_array,
            &self.normals_array,
            &self.indices_array)
        //RawModel::new(0, 0)
    }

    //////////// Recibe un vector de 3  elementos. por ej: ////////////
    //////////// vv[0]=3, vv[1]=2, vv[2]=1                 ////////////
    pub fn process_vertex(&mut self, v: Vec<&str>) {
        //println!("v en process_vertex = {:#?}", v);
        let current_vertex_pointer = v[0].parse::<usize>().unwrap() - 1;
        //println!("current_vertex_pointer en process_vertex = {}", current_vertex_pointer);
        self.indices.push(current_vertex_pointer as u32);
        //println!("indices.len = {}", self.indices.len());

        if v[1] != "" { // "" -> Objetos que no tienen textura  Ej:   f 7//1 8//2 9//3
            // Índice de textura (-1 empieza por 0)
            let current_tex: V2CG = self.textures[v[1].parse::<usize>().unwrap() - 1];
            self.texture_array[current_vertex_pointer * 2] = current_tex.x;
            self.texture_array[current_vertex_pointer * 2 + 1] = 1.0 - current_tex.y;
        } else {
            println!("----> Objeto sin textura ")
        }

        //Índice de normal (-1 empieza por 0)
        let current_norm: V3CG = self.normals[v[2].parse::<usize>().unwrap() - 1];
        self.normals_array[current_vertex_pointer * 3] = current_norm.x;
        self.normals_array[current_vertex_pointer * 3 + 1] = current_norm.y;
        self.normals_array[current_vertex_pointer * 3 + 2] = current_norm.z;
    }
}
