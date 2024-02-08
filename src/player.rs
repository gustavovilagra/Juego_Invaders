use sdl2::image::LoadTexture;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::rect::Rect;
use crate::PLAYER_SPEED;
use crate::{shot::Shot, HEIGHT, PLAYER_SIZE, WIDTH}; 
use std::time::{Duration, Instant};

pub struct Player<'a> {
    pub x: f32, // Cambiado a f32
    pub y: i32,
    velocity: f32, //variable para la velocidad
    pub shots: Vec<Shot>, // Disparos del jugador
    pub texture: Texture<'a>,
    shot_delay: Duration, // Retraso entre disparos
    last_shot_time: Instant, // Instante del último disparo
    moving_left: bool,
    moving_right: bool,

    pub is_hit: bool,
}

impl<'a> Player<'a> {
    // Constructor de Player
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Player<'a>, String> {
        let texture = texture_creator.load_texture("resource/f30.png")?;
        Ok(Player {
            x: (WIDTH as i32 / 40) as f32, // Convertido a f32
            y: HEIGHT as i32 / 20 - 1, // Posición inicial en la parte inferior
            shots: Vec::new(),
            shot_delay: Duration::from_millis(300), // 500 ms de retraso entre disparos
            last_shot_time: Instant::now(),
            texture,
            velocity: 0.0, // Inicializa la velocidad a 0
            moving_left: false,
            moving_right: false,

            is_hit:false,
        })
    }
    pub fn reset_velocity(&mut self) {
        self.velocity = 0.0;
    }
      // Actualiza los métodos move_left y move_right para cambiar los campos moving_left y moving_right
      pub fn move_left(&mut self) {
        self.moving_left = true;
    }
    pub fn stop_left(&mut self) {
        self.moving_left = false;
     
    }
    pub fn move_right(&mut self) {
        
        self.moving_right = true;
    }
    pub fn stop_right(&mut self) {
        self.moving_right = false;
    }

   
    pub fn update(&mut self, delta_time: f32) {
        if self.moving_left {
            self.x -= PLAYER_SPEED * delta_time;
        }
        if self.moving_right {
            self.x += PLAYER_SPEED * delta_time;
        }  
        // Marca los límites de la pantalla para Player
        self.x = self.x.clamp(0.0, 57.0); 

    }
      
    pub fn shoot(&mut self) {
        if self.last_shot_time.elapsed() >= self.shot_delay {
  
            self.shots.push(Shot::new(self.x as i32 , self.y,-1,1));
           
            self.last_shot_time = Instant::now();
        }
    }
   
    // Actualizar los disparos del jugador
    pub fn update_shots(&mut self) {
        for shot in &mut self.shots {
            shot.update();
        }
        self.shots.retain(|shot| shot.active);
    }
    

    // Dibujar el jugador y sus disparos en la pantalla
    pub fn draw(&self, canvas: &mut Canvas<Window>) {

        let nave_width = PLAYER_SIZE as u32 +50;
        let nave_height = PLAYER_SIZE as u32 +60;

        
        let player_rect  = Rect::new(self.x as i32 * PLAYER_SIZE  -17, self.y * PLAYER_SIZE -70, nave_width, nave_height);
        canvas.copy(&self.texture, None, Some(player_rect)).unwrap();


        // Dibujar los disparos del jugador
        for shot in &self.shots {
            shot.draw(canvas);
        }
    }

    pub fn reset_player_position(&mut self) {
        // Reinicia la posición inicial del jugador al centro de la pantalla
        self.x = (WIDTH as i32 / 40) as f32;
        self.y = HEIGHT as i32 / 20 - 1;
        
        // Elimina todos los disparos del jugador
        self.shots.clear();

    }

}


