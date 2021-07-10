// Uso vectores columna paso a vector fila

use cgmath;
use cgmath::vec3;

use crate::entities::camera::Camera;

type V2CG = cgmath::Vector2<f32>;
type V3CG = cgmath::Vector3<f32>;
type M4CG = cgmath::Matrix4<f32>;

//pub const PI: f32 = std::f64::consts::PI as f32;
//pub const TWO_PI: f32 = (std::f64::consts::PI * 2.0) as f32;

#[derive(Debug, Clone, Copy)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl V3 {
    pub fn new(x: f32, y: f32, z: f32) -> V3 {
        V3 {
            x,
            y,
            z,
        }
    }

    // Normaliza este vector
    pub fn _normalize(&mut self) {
        let a = self.x * self.x;
        let b = self.y * self.y;
        let c = self.z * self.z;
        //let c = float64(v.Z * v.Z)

        let longitud = (a + b + c).sqrt();

        if longitud != 0.0 {
            self.x /= longitud;
            self.y /= longitud;
            self.z /= longitud;
        }
    }
    pub fn _zero() -> V3 {
        V3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn _unit_x() -> V3 {
        V3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn _unit_y() -> V3 {
        V3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }
    pub fn _unit_z() -> V3 {
        V3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    pub fn _cross() {}

    pub fn _v3mia_to_v3cg(m: V3) -> V3CG {
        vec3(m.x, m.y, m.z)
    }

    pub fn v3cg_to_v3mia(m: V3CG) -> V3 {
        V3::new(m.x, m.y, m.z)
    }
}


#[derive(Debug, Clone)]
pub struct Matrix4x4 {
    pub data: [f32; 16]
}

impl Matrix4x4 {
    pub fn identity4x4() -> Matrix4x4 {
        Matrix4x4 {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0]
        }
    }
    pub fn m4cg_to_matrix4x4(m: M4CG) -> Matrix4x4 {
        let mut r = Matrix4x4::identity4x4();
        r.data[0] = m[0][0];
        r.data[1] = m[0][1];
        r.data[2] = m[0][2];
        r.data[3] = m[0][3];

        r.data[4] = m[1][0];
        r.data[5] = m[1][1];
        r.data[6] = m[1][2];
        r.data[7] = m[1][3];

        r.data[8] = m[2][0];
        r.data[9] = m[2][1];
        r.data[10] = m[2][2];
        r.data[11] = m[2][3];

        r.data[12] = m[3][0];
        r.data[13] = m[3][1];
        r.data[14] = m[3][2];
        r.data[15] = m[3][3];
        r
    }
    pub fn matrix4x4_to_m4gc(m: Matrix4x4) -> M4CG {
        let r = M4CG::new(
            m.data[0], m.data[1], m.data[2], m.data[3],
            m.data[4], m.data[5], m.data[6], m.data[7],
            m.data[8], m.data[9], m.data[10], m.data[11],
            m.data[12], m.data[13], m.data[14], m.data[15]);

        r
    }
    //// Translación respecto a los ejes globales
    pub fn translate(vector: &V3, matriz: &Matrix4x4) -> Matrix4x4 {
        let mut m = Matrix4x4::identity4x4();

        m.data[12] = vector.x;
        m.data[13] = vector.y;
        m.data[14] = vector.z;

        //let m1 = Matrix4x4::transpone_matriz_mia(m);
        // Crea matriz de transformación
        //param.matriz_traslacion = m;

        // Mtranslacion x MTotal ->es una transformación global
        let m_resultado = Matrix4x4::product_matrix_axb(&m, &matriz);
        m_resultado
    }

    // carga la matriz de rotacion del eje x con una coordenada
    pub fn rotate_x(angulo: f32, matriz: &Matrix4x4) -> Matrix4x4 {
        let mut m = Matrix4x4::identity4x4();

        let c = angulo.cos();
        let s = angulo.sin();

        m.data[5] = c;
        m.data[6] = s;
        m.data[9] = -s;
        m.data[10] = c;

        //let m1 = Matrix4x4::transpone_matriz_mia(m);
        // Crea matriz de rotación por el eje z
        //param.matriz_rotacionz = m;

        // MRotacionZ x MTotal ->es una transformación global
        let m_resultado = Matrix4x4::product_matrix_axb(&m, &matriz);
        m_resultado
    }

    // carga la matriz de rotacion del eje y con una coordenada
    pub fn rotate_y(angulo: f32, matriz: &Matrix4x4) -> Matrix4x4 {
        let mut m = Matrix4x4::identity4x4();

        let c = angulo.cos();
        let s = angulo.sin();

        m.data[0] = c;
        m.data[2] = -s;
        m.data[8] = s;
        m.data[10] = c;

        //let m1 = Matrix4x4::transpone_matriz_mia(m);
        // Crea matriz de rotación por el eje z
        //param.matriz_rotacionz = m;

        // MRotacionZ x MTotal ->es una transformación global
        let m_resultado = Matrix4x4::product_matrix_axb(&m, &matriz);
        m_resultado
    }

    // Recibe un ángulo en radianes y carga la matriz de rotacion del eje z
    pub fn rotate_z(angulo: f32, matriz: &Matrix4x4) -> Matrix4x4 {
        let mut m = Matrix4x4::identity4x4();

        let c = angulo.cos();
        let s = angulo.sin();

        m.data[0] = c;
        m.data[1] = s;
        m.data[4] = -s;
        m.data[5] = c;

        //let m1 = Matrix4x4::transpone_matriz_mia(m);
        // Crea matriz de rotación por el eje z
        //param.matriz_rotacionz = m;

        // MRotacionZ x MTotal ->es una transformación global
        let m_resultado = Matrix4x4::product_matrix_axb(&m, &matriz);
        m_resultado
    }

    // Escala igual en los tres ejes
    pub fn scale(escala: &V3, matriz: &Matrix4x4) -> Matrix4x4 {
        let mut m = Matrix4x4::identity4x4();

        m.data[0] = escala.x;
        m.data[5] = escala.y;
        m.data[10] = escala.z;

        //let m1 = Matrix4x4::transpone_matriz_mia(m);
        // Crea matriz de transformación
        //param.matriz_traslacion = m;

        // Mtranslacion x MTotal ->es una transformación global
        let m_resultado = Matrix4x4::product_matrix_axb(&m, &matriz);
        m_resultado
    }
    /*pub fn transpone_matriz_mia(m: Matrix4x4) -> Matrix4x4 {
        let mut r = Matrix4x4::identity4x4();
        r.data[0] = m.data[0];
        r.data[1] = m.data[4];
        r.data[2] = m.data[8];
        r.data[3] = m.data[12];

        r.data[4] = m.data[1];
        r.data[5] = m.data[5];
        r.data[6] = m.data[9];
        r.data[7] = m.data[13];

        r.data[8] = m.data[2];
        r.data[9] = m.data[6];
        r.data[10] = m.data[10];
        r.data[11] = m.data[14];

        r.data[12] = m.data[3];
        r.data[13] = m.data[7];
        r.data[14] = m.data[11];
        r.data[15] = m.data[15];
        r
    }*/
    //    // Recibe una matriz 3x3 y un vector2 de P5 y devuelve su producto M x V en formato Vector2
//// El vector debe ser vector columna, devuelve vector2 raylib
//    pub fn product_matrix_v3_columna(m: &Matrix3x3, v: &Vector2) -> Vector2 {
//        //println!("en product_matrix_v3 matriz_total = {:#?}", m);
//        let x = m.data[0] * v.x + m.data[1] * v.y + m.data[2];
//        let y = m.data[3] * v.x + m.data[4] * v.y + m.data[5];
//        //Vector2_fi::new(x, y)
//        Vector2::new(x, y)
//    }
    // Recibe dos matrices 4x4 y devuelve su producto  A x B    Ojo con el orden
    pub fn product_matrix_axb(ma: &Matrix4x4, mb: &Matrix4x4) -> Matrix4x4 {
        let mut mr = Matrix4x4::identity4x4();
// -----------------------------
        let a = ma.data[0];
        let b = ma.data[1];
        let c = ma.data[2];
        let d = ma.data[3];

        let e = ma.data[4];
        let f = ma.data[5];
        let g = ma.data[6];
        let h = ma.data[7];

        let i = ma.data[8];
        let j = ma.data[9];
        let k = ma.data[10];
        let l = ma.data[11];

        let m = ma.data[12];
        let n = ma.data[13];
        let o = ma.data[14];
        let p = ma.data[15];
// -----------------------------
        let aa = mb.data[0];
        let bb = mb.data[1];
        let cc = mb.data[2];
        let dd = mb.data[3];

        let ee = mb.data[4];
        let ff = mb.data[5];
        let gg = mb.data[6];
        let hh = mb.data[7];

        let ii = mb.data[8];
        let jj = mb.data[9];
        let kk = mb.data[10];
        let ll = mb.data[11];

        let mm = mb.data[12];
        let nn = mb.data[13];
        let oo = mb.data[14];
        let pp = mb.data[15];
// -----------------------------
        mr.data[0] = a * aa + b * ee + c * ii + d * mm;
        mr.data[1] = a * bb + b * ff + c * jj + d * nn;
        mr.data[2] = a * cc + b * gg + c * kk + d * oo;
        mr.data[3] = a * dd + b * hh + c * ll + d * pp;

        mr.data[4] = e * aa + f * ee + g * ii + h * mm;
        mr.data[5] = e * bb + f * ff + g * jj + h * nn;
        mr.data[6] = e * cc + f * gg + g * kk + h * oo;
        mr.data[7] = e * dd + f * hh + g * ll + h * pp;

        mr.data[8] = i * aa + j * ee + k * ii + l * mm;
        mr.data[9] = i * bb + j * ff + k * jj + l * nn;
        mr.data[10] = i * cc + j * gg + k * kk + l * oo;
        mr.data[11] = i * dd + j * hh + k * ll + l * pp;


        mr.data[12] = m * aa + n * ee + o * ii + p * mm;
        mr.data[13] = m * bb + n * ff + o * jj + p * nn;
        mr.data[14] = m * cc + n * gg + o * kk + p * oo;
        mr.data[15] = m * dd + n * hh + o * ll + p * pp;

        mr
    }
}


pub fn barry_centric(p1: V3CG, p2: V3CG, p3: V3CG, pos: V2CG) -> f32 {
    let det = (p2.z - p3.z) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.z - p3.z);
    let l1 = ((p2.z - p3.z) * (pos.x - p3.x) + (p3.x - p2.x) * (pos.y - p3.z)) / det;
    let l2 = ((p3.z - p1.z) * (pos.x - p3.x) + (p1.x - p3.x) * (pos.y - p3.z)) / det;
    let l3 = 1.0 - l1 - l2;
    l1 * p1.y + l2 * p2.y + l3 * p3.y
}

// Ojo falta escala --------------------------------------------------
pub fn create_transformation_matrix(position: V3CG, rx: f32, ry: f32, rz: f32, scale: V3CG) -> M4CG {
    let m = Matrix4x4::identity4x4();
    //dbg!("1",&m);
    let m = Matrix4x4::translate(&V3::v3cg_to_v3mia(position), &m);
    //dbg!("2",&m);
    let m = Matrix4x4::rotate_x((rx).to_radians(), &m);
    //dbg!("3",&m);
    let m = Matrix4x4::rotate_y((ry).to_radians(), &m);
    //dbg!("4",&m);
    let m = Matrix4x4::rotate_z((rz).to_radians(), &m);

    //dbg!("5",&m);
    let m = Matrix4x4::scale(&V3::v3cg_to_v3mia(scale), &m);
    //dbg!("6",&m);

    // calculate the model matrix for each object

    /* let mut model: M4CG = M4CG::from_translation(*position);
     model = model * M4CG::from_axis_angle(vec3(1.0, 0.0, 0.0), Deg(rx));
     model = model * M4CG::from_axis_angle(vec3(0.0, 1.0, 0.0), Deg(ry));
     model = model * M4CG::from_axis_angle(vec3(0.0, 0.0, 1.0), Deg(rz));*/


    //println!("eje = {:#?}", eje);

    //println!("matriz model = {:#?}", model);
    //dbg!(&model);
    let model = Matrix4x4::matrix4x4_to_m4gc(m);
    model
}

// Para rendering GUIs --------------------------------------------------
pub fn create_transformation_matrix2(position: V2CG, scale: V2CG) -> M4CG {
    let m = Matrix4x4::identity4x4();
    //dbg!("1",&m);
    let traslacion = vec3(position.x, position.y, 0.0);
    let m = Matrix4x4::translate(&V3::v3cg_to_v3mia(traslacion), &m);
    //dbg!("2",&m);

    let escala = vec3(scale.x, scale.y, 1.0);
    let m = Matrix4x4::scale(&V3::v3cg_to_v3mia(escala), &m);
    //dbg!("6",&m);

    // calculate the model matrix for each object

    /* let mut model: M4CG = M4CG::from_translation(*position);
     model = model * M4CG::from_axis_angle(vec3(1.0, 0.0, 0.0), Deg(rx));
     model = model * M4CG::from_axis_angle(vec3(0.0, 1.0, 0.0), Deg(ry));
     model = model * M4CG::from_axis_angle(vec3(0.0, 0.0, 1.0), Deg(rz));*/


    //println!("eje = {:#?}", eje);

    //println!("matriz model = {:#?}", model);
    //dbg!(&model);
    let model = Matrix4x4::matrix4x4_to_m4gc(m);
    model
}

pub fn create_view_matrix(camera: &mut Camera) -> M4CG {
    let view_matrix = Matrix4x4::identity4x4();

    let view_matrix = Matrix4x4::rotate_x(camera.get_pitch().to_radians(), &view_matrix);
    let view_matrix = Matrix4x4::rotate_y(camera.get_yaw().to_radians(), &view_matrix);

    let camera_pos = camera.get_position();
    let negative_camera_pos = V3::new(-camera_pos.x, -camera_pos.y, -camera_pos.z);
    let view_matrix = Matrix4x4::translate(&negative_camera_pos, &view_matrix);


    /*
        // Calculate the new Front vector
        let front = V3CG {
            x: camera.yaw.to_radians().cos() * camera.pitch.to_radians().cos(),
            y: camera.pitch.to_radians().sin(),
            z: camera.yaw.to_radians().sin() * camera.pitch.to_radians().cos(),
        };
        camera.front = front.normalize();
        // Also re-calculate the Right and Up vector
        camera.right = camera.front.cross(camera.world_up).normalize(); // Normalize the vectors,
        // because their length gets closer to 0 the more you look up or down which results in slower movement.
        camera.up = camera.right.cross(camera.front).normalize();

        //println!("posicion de la camara = {:#?}", camera.get_position());
        let view_matrix = M4CG::look_at_rh(camera.get_position(),
                                           camera.get_position() + camera.front,
                                           vec3(0.0, 1.0, 0.0));
    */


    //dbg!(&view_matrix);

    Matrix4x4::matrix4x4_to_m4gc(view_matrix)
}

pub fn create_projection_matrix_perspective(ancho: f32, alto: f32) -> M4CG {
    // Matriz de proyección
    // Znear=Distancia del punto del observador al plano near
    // Zfar=Distancia del punto del observador al plano far
    // a = Aspect Ratio
    // fov = Field of view
    // zm = Zfar - Znear
    // zp = Zfar + Znear

    // (1/tan(fov/2)))/a         0            0               0
    //          0          1/(tan(fov/2))     0               0
    //          0                0          -zp/zm   -(2*Zfar*Znear))/zm
    //          0                0            -1              0)

    let aspect_ratio = ancho / alto;
    let fov1: f32 = 70.0;
    let fov = fov1.to_radians();
    let znear: f32 = 0.1;
    let zfar: f32 = 1000.0;

    let y_scale = 1.0 / (fov / 2.0).tan();
    let x_scale = y_scale / aspect_ratio;
    let frustum_length = zfar - znear;

    let mut projection_matrix = Matrix4x4::identity4x4();

    // En Java es [columna][fila]
    projection_matrix.data[0] = x_scale;
    projection_matrix.data[5] = y_scale;
    projection_matrix.data[10] = -((zfar + znear) / frustum_length);
    projection_matrix.data[14] = -((2.0 * znear * zfar) / frustum_length);
    projection_matrix.data[11] = -1.0;
    projection_matrix.data[15] = 0.0;

    /*      // pass projection matrix to shader (note that in this case it could change every frame)
          let projection_matrix: M4CG = cgmath::perspective(
              cgmath::Deg(fov),
              aspect_ratio,
              znear,
              zfar,
          );
          dbg!(&projection_matrix);*/
    //let projection_matrix = Matrix4x4::transpone_matriz_mia(projection_matrix);
    Matrix4x4::matrix4x4_to_m4gc(projection_matrix)
}

//  pub fn create_projection_matrix_orthographic(ancho: f32, alto: f32) -> Matrix4x4 {
// Matriz de proyección
// Znear=Distancia del punto del observador al plano near
// Zfar=Distancia del punto del observador al plano far

// zm = Zfar - Znear
// zp = Zfar + Znear

//      1/ancho              0            0               0
//          0             1/alto          0               0
//          0                0          2/zm          -(zp/zm)
//          0                0            0               1)


//        let znear: f32 = 0.1;
//        let zfar: f32 = 1000.0;
//
//
//        let zm = zfar - znear;
//        let zp = zfar + znear;
//
//        let mut projection_matrix = Matrix4x4::identity4x4();
//
//        projection_matrix.data[0] = 1.0 / ancho;
//        projection_matrix.data[5] = 1.0 / alto;
//        projection_matrix.data[10] = 2.0 / zm;
//        projection_matrix.data[11] = -(zp / zm);
//
//        //dbg!(&projection_matrix);
//        projection_matrix
//    }
//// Translación respecto a los ejes globales
//pub fn translate(vector: V3, matriz: &Matrix4x4) -> Matrix4x4 {
//    let mut m = Matrix4x4::identity4x4();
//
//    m.data[3] = vector.x;
//    m.data[7] = vector.y;
//    m.data[11] = vector.z;
//
//    // Crea matriz de transformación
//    //param.matriz_traslacion = m;
//
//    // Mtranslacion x MTotal ->es una transformación global
//    let m_resultado = Matrix4x4::product_matrix_axb(&matriz, &m);
//    m_resultado
//}

//// carga la matriz de rotacion del eje x con una coordenada
//pub fn rotate_x(angulo: f32, matriz: &Matrix4x4) -> Matrix4x4 {
//    let mut m = Matrix4x4::identity4x4();
//
//    let c = angulo.cos();
//    let s = angulo.sin();
//
//    m.data[5] = c;
//    m.data[6] = -s;
//    m.data[9] = s;
//    m.data[10] = c;
//
//    // Crea matriz de rotación por el eje z
//    //param.matriz_rotacionz = m;
//
//    // MRotacionZ x MTotal ->es una transformación global
//    let m_resultado = Matrix4x4::product_matrix_axb(&matriz, &m);
//    m_resultado
//}

//// carga la matriz de rotacion del eje y con una coordenada
//pub fn rotate_y(angulo: f32, matriz: &Matrix4x4) -> Matrix4x4 {
//    let mut m = Matrix4x4::identity4x4();
//
//    let c = angulo.cos();
//    let s = angulo.sin();
//
//    m.data[0] = c;
//    m.data[2] = s;
//    m.data[8] = -s;
//    m.data[10] = c;
//
//    // Crea matriz de rotación por el eje z
//    //param.matriz_rotacionz = m;
//
//    // MRotacionZ x MTotal ->es una transformación global
//    let m_resultado = Matrix4x4::product_matrix_axb(&matriz, &m);
//    m_resultado
//}

//// Recibe un ángulo en radianes y carga la matriz de rotacion del eje z
//pub fn rotate_z(angulo: f32, matriz: &Matrix4x4) -> Matrix4x4 {
//    let mut m = Matrix4x4::identity4x4();
//
//    let c = angulo.cos();
//    let s = angulo.sin();
//
//    m.data[0] = c;
//    m.data[1] = -s;
//    m.data[4] = s;
//    m.data[5] = c;
//
//    // Crea matriz de rotación por el eje z
//    //param.matriz_rotacionz = m;
//
//    // MRotacionZ x MTotal ->es una transformación global
//    let m_resultado = Matrix4x4::product_matrix_axb(&matriz, &m);
//    m_resultado
//}

//// Escala igual en los tres ejes
//pub fn scale(escala: V3, matriz: &Matrix4x4) -> Matrix4x4 {
//    let mut m = Matrix4x4::identity4x4();
//
//    m.data[0] = escala.x;
//    m.data[5] = escala.y;
//    m.data[10] = escala.z;
//
//    // Crea matriz de transformación
//    //param.matriz_traslacion = m;
//
//    // Mtranslacion x MTotal ->es una transformación global
//    let m_resultado = Matrix4x4::product_matrix_axb(&matriz, &m);
//    m_resultado
//}

//    // Recibe una matriz 3x3 y un vector2 de P5 y devuelve su producto M x V en formato Vector2
//// El vector debe ser vector columna, devuelve vector2 raylib
//    pub fn product_matrix_v3_columna(m: &Matrix3x3, v: &Vector2) -> Vector2 {
//        //println!("en product_matrix_v3 matriz_total = {:#?}", m);
//        let x = m.data[0] * v.x + m.data[1] * v.y + m.data[2];
//        let y = m.data[3] * v.x + m.data[4] * v.y + m.data[5];
//        //Vector2_fi::new(x, y)
//        Vector2::new(x, y)
//    }
// Recibe dos matrices 4x4 y devuelve su producto  A x B    Ojo con el orden
//pub fn product_matrix_axb(ma: &Matrix4x4, mb: &Matrix4x4) -> Matrix4x4 {
//    let mut mr = Matrix4x4::identity4x4();
//// -----------------------------
//    let a = ma.data[0];
//    let b = ma.data[1];
//    let c = ma.data[2];
//    let d = ma.data[3];
//
//    let e = ma.data[4];
//    let f = ma.data[5];
//    let g = ma.data[6];
//    let h = ma.data[7];
//
//    let i = ma.data[8];
//    let j = ma.data[9];
//    let k = ma.data[10];
//    let l = ma.data[11];
//
//    let m = ma.data[12];
//    let n = ma.data[13];
//    let o = ma.data[14];
//    let p = ma.data[15];
//// -----------------------------
//    let aa = mb.data[0];
//    let bb = mb.data[1];
//    let cc = mb.data[2];
//    let dd = mb.data[3];
//
//    let ee = mb.data[4];
//    let ff = mb.data[5];
//    let gg = mb.data[6];
//    let hh = mb.data[7];
//
//    let ii = mb.data[8];
//    let jj = mb.data[9];
//    let kk = mb.data[10];
//    let ll = mb.data[11];
//
//    let mm = mb.data[12];
//    let nn = mb.data[13];
//    let oo = mb.data[14];
//    let pp = mb.data[15];
//// -----------------------------
//    mr.data[0] = a * aa + b * ee + c * ii + d * mm;
//    mr.data[1] = a * bb + b * ff + c * jj + d * nn;
//    mr.data[2] = a * cc + b * gg + c * kk + d * oo;
//    mr.data[3] = a * dd + b * hh + c * ll + d * pp;
//
//    mr.data[4] = e * aa + f * ee + g * ii + h * mm;
//    mr.data[5] = e * bb + f * ff + g * jj + h * nn;
//    mr.data[6] = e * cc + f * gg + g * kk + h * oo;
//    mr.data[7] = e * dd + f * hh + g * ll + h * pp;
//
//    mr.data[8] = i * aa + j * ee + k * ii + l * mm;
//    mr.data[9] = i * bb + j * ff + k * jj + l * nn;
//    mr.data[10] = i * cc + j * gg + k * kk + l * oo;
//    mr.data[11] = i * dd + j * hh + k * ll + l * pp;
//
//
//    mr.data[12] = m * aa + n * ee + o * ii + p * mm;
//    mr.data[13] = m * bb + n * ff + o * jj + p * nn;
//    mr.data[14] = m * cc + n * gg + o * kk + p * oo;
//    mr.data[15] = m * dd + n * hh + o * ll + p * pp;
//
//    mr
//}

//
//pub fn to_radiands(angle: f32) -> f32 {    //provisional
//    angle * PI / 180.0
//}