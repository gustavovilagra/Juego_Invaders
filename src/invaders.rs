use rand::Rng;
use sdl2::image::LoadTexture;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::rect::Rect;




use crate::player:: Player;
use crate::{shot::Shot, INVADER_HEIGHT, INVADER_WIDTH, PLAYER_SIZE, WIDTH};
use rand::seq::IteratorRandom;
use std::time::{Duration, Instant};

pub struct Invader<'a> {
    pub x: i32,
    pub y: i32,
    pub active: bool,
    pub exploding: bool,
    pub explosion_start: Instant,
    pub texture: Texture<'a>, 
    pub hit_count: u32, // Contador de impactos 
  
     
}

impl<'a> Invader<'a> {
    pub fn new(x: i32, y: i32, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Invader<'a>, String> {
        let texture = texture_creator.load_texture("resource/invaders.png")?;
        Ok(Invader {
            x,
            y,
            active: true,
            exploding: false,
            explosion_start: Instant::now(),
            texture,
            hit_count: 0,
            
            
        })
    
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, explosion_texture: &Texture, level: u32) {
        if self.active {
            let rect = if level == 4 {
                // Ajusta el tamaño para el nivel 1 (nave suprema)
                Rect::new(self.x * 20, self.y * 20, INVADER_WIDTH as u32 * 12, INVADER_HEIGHT as u32 * 12)
            } else if level == 3 {
                Rect::new(self.x * 20, self.y * 20, INVADER_WIDTH as u32 * 3, INVADER_HEIGHT as u32 * 3)
            }else {
                Rect::new(self.x * 20, self.y * 20, INVADER_WIDTH as u32 * 2, INVADER_HEIGHT as u32 * 2)
            };
    
            if self.exploding {
                canvas.copy(explosion_texture, None, Some(rect)).unwrap();
            } else {
                canvas.copy(&self.texture, None, Some(rect)).unwrap();
            }
        }
    }
}


pub struct Invaders<'a> { 
    pub army: Vec<Invader<'a>>, 
    pub shots: Vec<Shot>,
    pub last_update: Instant,
    pub move_timer: Duration,
    pub shot_timer: Instant,
    direction: i32,
    pub speed_increase_factor: f32, 
    pub explosion_texture: Texture<'a>, // Campo añadido para la textura de explosión
    pub level: u32,  // Nivel actual
  
   
}

impl<'a> Invaders<'a> { // Añadir el tiempo de vida aquí
    
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, level: u32) -> Result<Invaders<'a>, String> {
        let mut army = Vec::new();
   
        let (move_timer_duration, shot_timer_duration) = match level {
            1 => {
              

                // Configuración para el nivel 1
                let rows =  3;
                let cols = 10;
                let invader_spacing_x = 4;
                let invader_spacing_y = 2;

                for row in 0..rows {
                    for col in 0..cols {
                        let x = col * invader_spacing_x;
                        let y = row * invader_spacing_y;
                        let invader = Invader::new(x as i32, y as i32, texture_creator)?;

                        army.push(invader);
                    }
                }

                (1000, 5000) // Duración del temporizador de movimiento y disparo
            },
            2 => {
              

               // Configuración para el nivel 1
               let rows = 5;
               let cols = 14;
               let invader_spacing_x = 4;
               let invader_spacing_y = 2;

               for row in 0..rows {
                   for col in 0..cols {
                       let x = col * invader_spacing_x;
                       let y = row * invader_spacing_y;
                       let invader = Invader::new(x as i32, y as i32, texture_creator)?;

                       army.push(invader);
                   }
               }

               (700, 5500) // Duración del temporizador de movimiento y disparo
            },
            3 => {
              
                let rows = 5;
                let cols = 10;
                let invader_spacing_x = 4;
                let invader_spacing_y = 3;

                for row in 0..rows {
                    for col in 0..cols {
                        let x = col * invader_spacing_x;
                        let y = row * invader_spacing_y;
                        let invader = Invader::new(x as i32, y as i32, texture_creator)?;

                        army.push(invader);
                    }
                }

                (500, 500) // Duración del temporizador de movimiento y disparo
            },
            4 => {
                let rows = 1;
                let cols = 1;
                let invader_spacing_x = 1;
                let invader_spacing_y = 1;

                for row in 0..rows {
                    for col in 0..cols {
                        let x = col * invader_spacing_x;
                        let y = row * invader_spacing_y;
                        let invader = Invader::new(x as i32, y as i32, texture_creator)?;

                        army.push(invader);
                    }
                }
                (20, 1000) // Duración del temporizador de movimiento y disparo
            },
            // Agrega más casos para otros niveles aquí
            _ => {
                // Configuración por defecto o para niveles desconocidos
                (1000, 1500) // Duración del temporizador de movimiento y disparo por defecto
            }
        };

        let explosion_texture = texture_creator.load_texture("resource/explosion.png")?;

        Ok(Invaders {
            army,
            shots: Vec::new(),
            last_update: Instant::now(),
            move_timer: Duration::from_millis(move_timer_duration),
            shot_timer: Instant::now().checked_add(Duration::from_millis(shot_timer_duration)).unwrap_or(Instant::now()), // Inicializa shot_timer según el nivel
            direction: 1,
            speed_increase_factor: 1.0,  // Inicializa con un factor de 1.0 (sin aumento)
            explosion_texture, // Guarda la textura cargada aquí
            level, 
        })
    }

    pub fn all_invaders_inactive(&self) -> bool {
        self.army.iter().all(|invader| !invader.active)
    }

    fn calculate_bounds(&self) -> (i32, i32) {
        let min_x = self.army.iter()
                             .filter(|invader| invader.active)
                             .map(|invader| invader.x)
                             .min()
                             .unwrap_or(0);

        let max_x = self.army.iter()
                             .filter(|invader| invader.active)
                             .map(|invader| invader.x)
                             .max()
                             .unwrap_or(0);

        (min_x, max_x)
    }


 
        pub fn update(&mut self) {
            
              // Lógica de disparo
            if Instant::now().duration_since(self.shot_timer) > Duration::from_millis(800) {
                let mut rng = rand::thread_rng();

                if self.level == 4 {
                    // Lógica de disparo específica para el nivel 4
                    if let Some(supreme_invader) = self.army.first() {
                        if supreme_invader.active && !supreme_invader.exploding {
                            // Ajustar las coordenadas y la velocidad del disparo
                            let shot_x = supreme_invader.x * 20 + (INVADER_WIDTH as i32 * 7); // Asumiendo que el invasor es 4 veces más grande
                            let shot_y = supreme_invader.y * 20 + (INVADER_HEIGHT as i32 * 8);
                            let invader_shot = Shot::new(shot_x, shot_y, 1, 9); // Velocidad de disparo para el invasor supremo
                            self.shots.push(invader_shot);
                        }
                    }
                } else {
                    // Lógica de disparo regular para otros niveles
                    if rng.gen::<f32>() < 0.50 {
                        if let Some(invader) = self.army.iter().filter(|invader| invader.active).choose(&mut rng) {
                            
                           
                                let shot_x = invader.x * 20 + INVADER_WIDTH as i32 / 2;
                                let shot_y = invader.y * 20 + INVADER_HEIGHT as i32;
                            
                           
                            let shot_speed = match self.level {
                                1 => 2,
                                2 => 3,
                                3 => 7,
                                _ => 3,
                            };
                            let invader_shot = Shot::new(shot_x, shot_y, 1, shot_speed);
                            self.shots.push(invader_shot);

                           
                           
                        }
                    }
                }
                self.shot_timer = Instant::now(); // Reiniciar el temporizador de disparo
            }
           
            // Lógica de movimiento de los invasores
            if self.last_update.elapsed() > self.move_timer {
                let (min_x, max_x) = self.calculate_bounds();
                
                let mut move_down = false;
                let mut vertical_down=false;

                let size_factor4 = if self.level == 4 { 12 } else { 1 }; // Tamaño aumentado 
                let step_factor4 = if self.level == 4 { 0.10 } else { 1.0 }; // Pasos más cortos

                let size_factor3 = if self.level == 3 { 3 } else { 1 }; // Tamaño aumentado 
                let step_factor3 = if self.level == 3 { 0.50 } else { 1.0 }; // Pasos más cortos

                
            if self.level == 3 {  
                // Cambiar la dirección y decidir si mover hacia abajo
                if self.direction == 1 && (max_x + INVADER_WIDTH * size_factor3 / 20) >= WIDTH as i32 / 20 - 1 {
                    self.direction = -1;
                    move_down = true;

                    
                } else if self.direction == -1 && min_x <= 0 {
                    self.direction = 1;
                    move_down = true;
                }


                for invader in &mut self.army {
                        if vertical_down{
                            invader.y -= 1;
                        }

                        if move_down {
                            invader.y += 1;
                        } else {
                            // Convierte self.direction a f32 antes de multiplicar
                            let movement = (self.direction as f32) * size_factor3 as f32 * step_factor3;
                            // Luego convierte el resultado final de vuelta a i32
                            invader.x += movement as i32;
                        }
                }
            }
            else if self.level == 4 {   
                // Cambiar la dirección y decidir si mover hacia abajo
                if self.direction == 1 && (max_x + INVADER_WIDTH * size_factor4 / 20) >= WIDTH as i32 / 20 - 1 {
                    self.direction = -1;
                    //move_down = true;
                 
                    vertical_down = true;
                    move_down = false;
                    
                    
                } else if self.direction == -1 && min_x <= 0 {
                    self.direction = 1;
                    move_down = true;
                }


                for invader in &mut self.army {
                        if vertical_down{
                            invader.y -= 1;
                        }

                        if move_down {
                            invader.y += 1;
                        } else {
                            // Convierte self.direction a f32 antes de multiplicar
                            let movement = (self.direction as f32) * size_factor4 as f32 * step_factor4;
                            // Luego convierte el resultado final de vuelta a i32
                            invader.x += movement as i32;
                        }
                }
            }else  {

                if self.direction == 1 && (max_x + INVADER_WIDTH * size_factor3 / 20) >= WIDTH as i32 / 20 - 1 {
                    self.direction = -1;
                    move_down = true;

                    
                } else if self.direction == -1 && min_x <= 0 {
                    self.direction = 1;
                    move_down = true;
                }


                for invader in &mut self.army {
                        if vertical_down{
                            invader.y -= 1;
                        }

                        if move_down {
                            invader.y += 1;
                        } else {
                            // Convierte self.direction a f32 antes de multiplicar
                            let movement = (self.direction as f32) * size_factor3 as f32 * step_factor3;
                            // Luego convierte el resultado final de vuelta a i32
                            invader.x += movement as i32;
                        }
                    }

            }



            if self.level == 4 { 
                 
                // Ajustar el temporizador de movimiento para el nivel 4
                if move_down {
                    self.speed_increase_factor *= 1.2;
                    self.move_timer = Duration::from_millis((200f32 / self.speed_increase_factor) as u64);
                }
            } else if self.level == 3 {
                if move_down {
                    self.speed_increase_factor *= 1.1;
                    self.move_timer = Duration::from_millis((700f32 / self.speed_increase_factor) as u64);
                }
            } else {
                if move_down {
                    // Ajustar el temporizador de movimiento para otros niveles
                    self.speed_increase_factor *= 1.1;
                    self.move_timer = Duration::from_millis((500f32 / self.speed_increase_factor) as u64);
                }
            }  
              
            self.last_update = Instant::now();          
        }
    
            // Actualizar los disparos independientemente del movimiento de los invasores
            for shot in &mut self.shots {
                shot.update();
            }
            self.shots.retain(|shot| shot.active);
    
            // Gestionar invasores explotados
            for invader in &mut self.army {
                if invader.exploding && invader.explosion_start.elapsed() > Duration::from_millis(300) {
                    invader.active = false;
                    invader.exploding = false;
                }
            }
        }

    
      // Método para restablecer los invasores
      pub fn reset(&mut self, texture_creator: &'a TextureCreator<WindowContext>, level: u32, player: &mut Player) -> Result<(), String> {
        
        // Limpiar la lista de invasores y disparos
        self.army.clear();
        self.shots.clear();

        // Volver a crear los invasores según el nuevo nivel
        self.create_army(texture_creator, level, player)?;

        // Restablecer otras variables de estado si las hay
        self.direction = 1;
        self.speed_increase_factor = 1.0;
        self.last_update = Instant::now();

        Ok(())
    }

    fn create_army(&mut self, texture_creator: &'a TextureCreator<WindowContext>, level: u32, player: &mut Player) -> Result<(), String> {
        let mut army = Vec::new();
        player.reset_player_position();
        // Configuración específica de cada nivel
        let (rows, cols, invader_spacing_x, invader_spacing_y, move_timer_duration, shot_timer_duration) = match level {
            1 => (3, 10, 4, 2, 1000, 5000),
            2 => (5, 14, 4, 2, 700, 5500),
            3 => (5, 10, 4, 3, 500, 500),
            4 => (1, 1, 1, 1, 20, 1000),
            _ => return Err("Unknown level".to_string()), // Manejar niveles desconocidos
        };
    
        for row in 0..rows {
            for col in 0..cols {
                let x = col * invader_spacing_x;
                let y = row * invader_spacing_y;
                let invader = Invader::new(x as i32, y as i32, texture_creator)?;
    
                army.push(invader);
            }
        }
    
        self.army = army;
        self.move_timer = Duration::from_millis(move_timer_duration);
        self.shot_timer = Instant::now().checked_add(Duration::from_millis(shot_timer_duration)).unwrap_or(Instant::now());
    
        Ok(())
    }
    



    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for invader in &self.army {
            invader.draw(canvas, &self.explosion_texture,self.level);
        }
        // Dibuja los disparos
        for shot in &self.shots {
            shot.draw_shot(canvas);
        }
    }

    pub fn check_player_hits(&mut self, player_shots: &mut Vec<Shot>) -> bool {
        let mut hit_something = false;
    
        for shot in player_shots.iter_mut() {
            if !shot.active { continue; }

            for invader in self.army.iter_mut() {
                // Lógica específica para el nivel 4
                if self.level == 4 && invader.active && shot_collides_with_invader(shot, invader,self.level) {
                    invader.hit_count += 1;
                    shot.active = false;
                    if invader.hit_count >= 10 {
                        invader.exploding = true;
                        invader.explosion_start = Instant::now();
                    }
                    hit_something = true;
                    break; // Detener más colisiones para este disparo
                }if self.level == 3 && invader.active && shot_collides_with_invader(shot, invader,self.level) {
                    invader.hit_count += 1;
                    shot.active = false;
                    if invader.hit_count >= 3 {
                        invader.exploding = true;
                        invader.explosion_start = Instant::now();
                        shot.active = false;
                    }
                    
                    hit_something = true;
                    break; // Detener más colisiones para este disparo
                }
                // Lógica para niveles diferentes de 4
                else if self.level != 4 && invader.active && !invader.exploding && shot_collides_with_invader(shot, invader,self.level) {
                    invader.exploding = true;
                    invader.explosion_start = Instant::now();
                    shot.active = false;
                    hit_something = true;
                    break; // Dejar de comprobar más colisiones para este disparo
                }
            }
        }
    
        player_shots.retain(|shot| shot.active); // Limpiar los disparos inactivos
        hit_something
    }

}

fn shot_collides_with_invader(shot: &Shot, invader: &Invader, level: u32) -> bool {
    let shot_rect = Rect::new(shot.x * PLAYER_SIZE + 17, shot.y * PLAYER_SIZE - 80, 6, 30);

    let invader_rect = if level == 4 {
        Rect::new(invader.x * 20, invader.y * 20, INVADER_WIDTH as u32 * 12, INVADER_HEIGHT as u32 * 9)  // Tamaño más grande para el invasor del nivel 4
    } else if level == 3 {
        Rect::new(invader.x * 20, invader.y * 20, INVADER_WIDTH as u32 * 3, INVADER_HEIGHT as u32 * 3)
    }else {
        Rect::new(invader.x * 20, invader.y * 20, INVADER_WIDTH as u32 * 2, INVADER_HEIGHT as u32 * 2)
    };

    shot_rect.has_intersection(invader_rect)
}

