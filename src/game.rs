use std::time::{Duration, Instant};

use sdl2::{
    image::LoadTexture,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use crate::{
    invaders::Invaders, player::Player, INVADER_HEIGHT, INVADER_WIDTH, PLAYER_SIZE, SHOT_HEIGHT,
    SHOT_WIDTH,
};

pub struct Game<'a> {
    pub is_game_over: bool,
    pub is_player_exploding: bool,
    pub explosion_start: Instant,
    pub explosion_texture: Texture<'a>,
}

impl<'a> Game<'a> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> Result<Game, String> {
        let explosion_texture = texture_creator.load_texture("resource/explosion.png")?;

        Ok(Game {
            is_game_over: false,
            is_player_exploding: false,
            explosion_start: Instant::now(),
            explosion_texture,
            // ... inicialización de otros campos ...
        })
    }

    pub fn check_game_over(&mut self, player: &mut Player, invaders: &Invaders) {
        if self.is_game_over {
            self.reset(); // Reinicia el juego si ya estaba en estado de "Game Over"
            player.is_hit = false;
        } else {
            let player_rect = Rect::new(
                (player.x * PLAYER_SIZE as f32).round() as i32 - 17,
                player.y * PLAYER_SIZE - 70, // Ajustar según la posición real del jugador
                PLAYER_SIZE as u32 + 50,     // Ancho del jugador
                PLAYER_SIZE as u32 + 60,     // Altura del jugador
            );

            // Verificar si algún invasor toca al jugador
            for invader in invaders.army.iter() {
                if invader.active {
                    let invader_rect = Rect::new(
                        invader.x * 20,            // Ajustar según la posición real del invasor
                        invader.y * 20,            // Ajustar según la posición real del invasor
                        INVADER_WIDTH as u32 * 2,  // Ancho del invasor
                        INVADER_HEIGHT as u32 * 2, // Altura del invasor
                    );

                    if player_rect.has_intersection(invader_rect) {
                        self.is_game_over = true;
                        player.is_hit = true;
                        return;
                    }
                }
            }

            for shot in invaders.shots.iter() {
                if shot.active {
                    let shot_rect =
                        Rect::new(shot.x, shot.y, SHOT_WIDTH as u32, SHOT_HEIGHT as u32);
                    if player_rect.has_intersection(shot_rect) {
                        self.is_game_over = true;
                        self.is_player_exploding = true;
                        self.explosion_start = Instant::now();

                        player.is_hit = true;
                        return;
                    }
                }
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, player: &mut Player) {
        // dibujar player

        if self.is_player_exploding && self.explosion_start.elapsed() < Duration::from_millis(500) {
            let explosion_rect = Rect::new(
                (player.x * PLAYER_SIZE as f32).round() as i32 - 17,
                // player.x * PLAYER_SIZE - 17, // Ajusta según la posición real del jugador
                player.y * PLAYER_SIZE - 70, // Ajusta según la posición real del jugador
                PLAYER_SIZE as u32 + 50, // Asumiendo que tienes un tamaño definido para la explosión
                PLAYER_SIZE as u32 + 60,
            );
            canvas
                .copy(&self.explosion_texture, None, Some(explosion_rect))
                .unwrap();
        }
    }

    pub fn reset(&mut self) {
        self.is_game_over = false;
        self.is_player_exploding = false;
        self.explosion_start = Instant::now();
    }
}
