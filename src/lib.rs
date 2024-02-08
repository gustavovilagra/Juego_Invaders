pub mod invaders;
pub mod player;
pub mod shot;
pub mod game;

// Con SDL2, puedes manejar las dimensiones de manera diferente. 
// Estas constantes pueden representar dimensiones de la ventana o ser usadas para escalar entidades.
pub const PLAYER_SIZE: i32 = 20;
pub const PLAYER_SPEED: f32 = 19.5; // Ajusta según sea necesario
pub const WIDTH: u32 = 1200;
pub const HEIGHT: u32 = 697;
pub const MAX_LEVEL: u32 = 4;

pub const SHOT_WIDTH: i32 = 5; // Ancho del disparo
pub const SHOT_HEIGHT: i32 = 10; // Alto del disparo
pub const INVADER_WIDTH: i32 = 20; // Ancho del invasor
pub const INVADER_HEIGHT: i32 = 20; // Alto del invasor
// pub const MAX_SPEED: f32 =1000.0;

// Otros ajustes globales o utilidades pueden ser agregados aquí si es necesario.
