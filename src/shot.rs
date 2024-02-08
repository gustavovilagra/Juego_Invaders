
   
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;


use crate::{HEIGHT, PLAYER_SIZE};

pub struct Shot {
    pub x: i32,
    pub y: i32,
    pub active: bool,
    pub direction: i32, // -1 para arriba (jugador), 1 para abajo (invasores)
    pub speed: i32,    // Velocidad del disparo
}

impl Shot {
    pub fn new(x: i32, y: i32, direction: i32,speed:i32) -> Shot {
        Shot {
            x,
            y,
            active: true,
            direction,
            speed,
        }
    }

    pub fn update(&mut self) {
        self.y += self.direction * self.speed; // Mueve el disparo según su dirección y velocidad
        if self.y < 0 || self.y > HEIGHT as i32 {
            self.active = false;
        }
    
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        if self.active {
            let rect = Rect::new(self.x * PLAYER_SIZE + 14, self.y * PLAYER_SIZE - 80, 6, 30);
            canvas.set_draw_color(Color::RGB(255, 255, 255)); // Color blanco
            canvas.fill_rect(rect).unwrap();
        }
    }

    pub fn draw_shot(&self, canvas: &mut Canvas<Window>) {
        if self.active {
            let rect = Rect::new(self.x, self.y, 6, 30);
            canvas.set_draw_color(Color::RGB(255, 0, 0)); // Color rojo
            canvas.fill_rect(rect).unwrap();
        }
    }
}
