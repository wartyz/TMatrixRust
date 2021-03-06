#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawModel {
    vao_id: u32,
    vertex_count: i32,
}

impl RawModel {
    pub fn new(vao_id: u32, vertex_count: i32) -> RawModel {
        RawModel {
            vao_id,
            vertex_count,
        }
    }

    pub fn get_vao_id(&self) -> u32 {
        self.vao_id
    }

    pub fn get_vertex_count(&self) -> i32 {
        self.vertex_count
    }
}