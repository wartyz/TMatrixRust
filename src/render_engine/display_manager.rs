use cgmath::vec3;
use glfw::{Context, Window};
use glfw::Action::{Press, Release};
use glfw::Glfw;
use glfw::MouseButton::*;
use glfw::MouseButton::Button2;

use std::sync::mpsc::Receiver;

use crate::entities::camera::Camera;

type V3CG = cgmath::Vector3<f32>;

pub const ANCHO: u32 = 1280;
pub const ALTO: u32 = 720;

pub struct DisplayManager {
    pub window: Window,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
    pub width: u32,
    pub height: u32,
    pub glfw: Glfw,
    pub last_frame_time: f32,
    pub delta: f32,
}

impl DisplayManager {
    pub fn new() -> DisplayManager {
        let (width, height) = (ANCHO, ALTO);

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
//        unsafe { // Por si quiero averiguar scancodes de teclas
//            let scancode_x = glfw::ffi::glfwGetKeyScancode(KEY_MENU);
//            dbg!(scancode_x);
//        }

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3)); // OpenGL 3.3
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        // Solo en pantalla total
        //glfw.window_hint(glfw::WindowHint::RefreshRate(Some(1)));


        let (mut window, events) = glfw
            .create_window(ANCHO, ALTO, "Ventana de Ejemplo", glfw::WindowMode::Windowed)
            .expect("Fallo al crear window GLFW.");

        window.make_current();
        window.set_key_polling(true); // pra obtener eventos de teclado
        window.set_mouse_button_polling(true);
        window.set_framebuffer_size_polling(true);

        // OJO sin esto no controlamos el ratón ------------------------
        // tell GLFW to capture our mouse
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);

        //window.set_cursor_mode(glfw::CursorMode::Disabled);


        // gl: cargar todos los punteros de funciones OpenGL
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            println!("{:#?}", gl::GetString(gl::VERSION));
        }
        DisplayManager {
            window,
            events,
            width,
            height,
            glfw,
            last_frame_time: 0.0,
            delta: 0.0,
        }
    }
    pub fn create_display(&mut self) {
        unsafe {
            gl::Viewport(0, 0, ANCHO as i32, ALTO as i32);
            gl::ClearColor(0.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.last_frame_time = self.get_current_time();
    }

    pub fn update_display(&mut self, camera: &mut Camera) {
        // Aqui debe ir   Display.sync(FPS_CAP); //Sincroniza frames/segundo
        //self.glfw.set_swap_interval(SwapInterval::Sync(2));
//
//        // ************* pba
//        let target_fps = 60.0;
//        let mut last_time = self.glfw.get_time();
//        while (self.glfw.get_time() < last_time + 1.0 / target_fps) {
//            // TODO: Put the thread to sleep, yield, or simply do nothing
//        }
//        last_time += 1.0 / target_fps;
//        // ****************

        let current_frame_time = self.get_current_time();
        self.delta = current_frame_time - self.last_frame_time;
        self.last_frame_time = current_frame_time;

        self.window.swap_buffers();


        self.procesa_eventos(camera);

        self.glfw.poll_events();
    }

    pub fn get_frame_time_seconds(&self) -> f32 {
        self.delta
    }

    pub fn close_display(&mut self) {}

    pub fn get_current_time(&self) -> f32 {
        // Devuelve en milisegundos
        (self.glfw.get_time()) as f32
    }

    // Recibe eventos y cambia parámetros de clase Mouse, para que puedan luego leerse por el programa
    pub fn procesa_eventos(&mut self, camera: &mut Camera) {
        let _first_mouse: &mut bool = &mut true;
        // yaw is initialized to -90.0 degrees since a yaw of 0.0
        let mut _yaw: &mut f32 = &mut -90.0;

        // results in a direction vector pointing to the right so we initially rotate a bit to the left.
        let mut _pitch: &mut f32 = &mut 0.0;
        let mut _last_x: &mut f32 = &mut (ANCHO as f32 / 2.0);
        let mut _last_y: &mut f32 = &mut (ALTO as f32 / 2.0);
        let mut _fov: &mut f32 = &mut 45.0;

        let mut _camera_front: &mut V3CG = &mut vec3(0.0, 0.0, -1.0);
        //dbg!("ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ");
        for (_, event) in glfw::flush_messages(&self.events) {
            //dbg!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
            match event {
                glfw::WindowEvent::MouseButton(mouse_button, action, _c) => {
                    //dbg!("BBBBBBBBBBBBBBBBBBBBBBBBBBBB");
                    //dbg!(mouse_button,action,c);
                    let mut lrmb = 0;
                    if mouse_button == Button2 { // Botón derecho
                        if action == Press {
                            lrmb = 2;
                        }
                        if action == Release {
                            lrmb = 0;
                        }
                    } else if mouse_button == Button1 {
                        if action == Press {
                            lrmb = 1;
                        }
                        if action == Release {
                            lrmb = 0;
                        }
                    }
                    camera.mouse.set_mouse_buttons(lrmb);
                }
                glfw::WindowEvent::Pos(a, b) => {
                    dbg!("CCCCCCCCCCCCCCCCCCCCCCCCCCC");
                    dbg!(a,b);
                }
                glfw::WindowEvent::Key(_tecla, _codigo_tecla, _estado, _d) => {
                    //dbg!("DDDDDDDDDDDDDDDDDDDDDDDDDDD");
                    //dbg!(tecla,codigo_tecla,estado,d);
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    dbg!("Modifica Ventanaaaaaaaaaaaaaaa");
                    // asegurarse de que el viewport coincida con las nuevas dimensiones de la ventana; tener en cuenta que el ancho y
                    // alto será significativamente mayor que el especificado en las pantallas de retina.
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    //dbg!("Posssssssssss ={}{}",xpos,ypos);
                    camera.mouse.mouse_position_input(xpos as f32, ypos as f32);
                }
                glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                    camera.mouse.mouse_wheel_input(yoffset as f32);
                }
                _ => { dbg!("------------------------------------------"); }
            }
        }
    }
}


