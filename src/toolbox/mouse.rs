pub struct Mouse {
    m_wheel: f32,
    x_offset: f32,
    y_offset: f32,
    x: f32,
    y: f32,
    last_x: f32,
    last_y: f32,
    mouse_state: i32,
    m_cursor_x_state: f32,
    m_cursor_y_state: f32,
    last_wheel_offset: f32,
    wheel_counter: f32,
    wheel_level: f32,
    c_offset: f32,

}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            m_wheel: 0.0,
            x_offset: 0.0,
            y_offset: 0.0,
            x: 0.0,
            y: 0.0,
            last_x: 0.0,
            last_y: 0.0,
            mouse_state: 0,  // apretados 2 derecho, 1 izquierdo
            m_cursor_x_state: 0.0,
            m_cursor_y_state: 0.0,
            last_wheel_offset: 0.0,
            wheel_counter: 0.0,
            wheel_level: 20.0,
            c_offset: 0.0,
        }
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn get_dx(&self) -> f32 {
        self.m_cursor_x_state
    }
    pub fn get_dy(&self) -> f32 {
        self.m_cursor_y_state
    }

    pub fn mouse_cursor_state(&mut self, dx: f32, dy: f32) {
        self.m_cursor_x_state = 0.0;
        self.m_cursor_y_state = 0.0;

        if self.get_mouse_buttons() > 0 {
            if self.last_x < dx {
                self.m_cursor_x_state = -1.0;
            } else if self.last_x > dx {
                self.m_cursor_x_state = 1.0;
            }

            if self.last_y < dy {
                self.m_cursor_y_state = -1.0;
            } else if self.last_y > dy {
                self.m_cursor_y_state = 1.0;
            }
        }

        self.last_x = dx;
        self.last_y = dy;
    }
    pub fn get_mouse_buttons(&self) -> i32 {
        self.mouse_state
    }
    // Apreta derecho lrmb = 2     Apreta izquierdo lrmb = 1
    pub fn set_mouse_buttons(&mut self, lr_mouse: i32) {
        self.mouse_state = lr_mouse;
    }

    // Al recibir evento de posicion x e y se llama este método
    pub fn mouse_position_input(&mut self, x: f32, y: f32) {

        //dbg!(x);
        self.x = x;
        self.y = y;
    }

    pub fn get_d_wheel(&self) -> f32 {
        self.m_wheel
    }

    pub fn mouse_wheel_input(&mut self, y_offset: f32) {
        self.y_offset = y_offset;
    }

    pub fn return_wheel_offset(&mut self) -> f32 {     // This needs to be fixed
        self.wheel_counter += 1.0;
        if self.last_wheel_offset != self.y_offset && self.y_offset != 0.0 {
            self.c_offset = self.y_offset;
        }

        if self.wheel_counter >= self.wheel_level {
            self.last_wheel_offset = self.y_offset;
            self.wheel_counter = 0.0;
            self.c_offset = 0.0;
        }

        self.c_offset
    }

    pub fn update_mouse_info(&mut self) {
        self.mouse_cursor_state(self.get_x(), self.get_y());
        self.m_wheel = self.return_wheel_offset();
    }

    pub fn _set_wheel_level(&mut self, wheel_level: f32) {
        self.wheel_level = wheel_level;
    }

//    pub fn xy_offset_input(&mut self, x_offset: f32, y_offset: f32) {
//        self.x_offset = x_offset;
//        self.y_offset = y_offset;
//    }
}