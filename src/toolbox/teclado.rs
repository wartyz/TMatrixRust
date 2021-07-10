pub struct Teclado {
    //    a: EstadoTecla,
//    d: EstadoTecla,
//    s: EstadoTecla,
//    w: EstadoTecla,
//    escape: EstadoTecla,
//    espacio: EstadoTecla,
    teclas: [bool; 350],

}

enum EstadoTecla {
    Apretada,
    NoApretada,
}

impl Teclado {
    pub fn new() -> Teclado {
        Teclado {
//            a: NoApretada,
//            d: NoApretada,
//            s: NoApretada,
//            w: NoApretada,
//            escape: NoApretada,
//            espacio: NoApretada,
            teclas: [false; 350],
        }
    }

    pub fn presionada_tecla(&mut self, scancode: i32) {
        self.teclas[scancode as usize] = true;
    }
}