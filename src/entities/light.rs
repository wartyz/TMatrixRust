use cgmath::vec3;

type V3CG = cgmath::Vector3<f32>;

pub struct Light {
    pub id: usize,
    position: V3CG,
    color: V3CG,
    attenuation: V3CG,
}

impl Light {
    pub fn new(id: usize, position: V3CG, color: V3CG) -> Light {
        Light {
            id,
            position,
            color,
            attenuation: vec3(1.0, 0.0, 0.0),
        }
    }
    pub fn new2(id: usize, position: V3CG, color: V3CG, attenuation: V3CG) -> Light {
        Light {
            id,
            position,
            color,
            attenuation,
        }
    }

    pub fn get_attenuation(&self) -> V3CG {
        self.attenuation
    }

    pub fn get_position(&self) -> V3CG {
        self.position
    }

    pub fn set_position(&mut self, position: V3CG) {
        self.position = position;
    }

    pub fn get_color(&self) -> V3CG {
        self.color
    }

    pub fn _set_color(&mut self, color: V3CG) {
        self.color = color;
    }
}

