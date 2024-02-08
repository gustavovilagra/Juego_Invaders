use std::time::{Duration, Instant};

use invaders_sld2::{game::Game, invaders::Invaders, player::Player, HEIGHT, MAX_LEVEL, WIDTH};
use rand::Rng;
use rusty_audio::Audio;
use sdl2::{event::Event, image::{InitFlag, LoadTexture}, keyboard::Keycode, pixels::Color, rect::{Point, Rect}, render::Canvas, ttf::Font, video::Window , EventPump};

extern crate sdl2;
extern crate rand; 


fn ask_continue(event_pump: &mut EventPump, canvas: &mut Canvas<Window>) -> bool {
    // Inicializar SDL2_ttf y cargar la fuente
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = "Sixtyfour-Regular.ttf";
    let font_size = 24;
    let font = ttf_context.load_font(font_path, font_size).unwrap();

    // Resto de la función
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    render_text(canvas, &font, "¿Quieres continuar? (Sí/No)", 100, 200,None);
    canvas.present();

     loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Y),
                    ..
                } => {
                   
                    return true; // Si se presiona la tecla 'Y', retornar true
                }
                Event::KeyDown {
                    keycode: Some(Keycode::N),
                    ..
                } => {
                    
                    return false; // Si se presiona la tecla 'N', retornar false
                }
                Event::Quit { .. } => {
                    std::process::exit(0); // Salir del programa si se cierra la ventana
                }
                _ => {}
            }
        }

        // Si prefieres esperar un poco entre iteraciones, puedes añadir un pequeño retardo
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn render_text(canvas: &mut Canvas<Window>,
        font: &Font,
        text: &str,
        x: i32,
        y: i32,
        color: Option<Color>) // Añadir color como un parámetro opcional 
        
    {
    // Renderizar el texto
    let surface = if let Some(c) = color {
        font.render(text).blended(c) // Utilizar el color proporcionado
    } else {
        font.render(text).blended(Color::RGB(255, 255, 255)) // Color blanco predeterminado
    }.unwrap();

    //let surface = font.render(text).blended(Color::RGB(255, 255, 255)).unwrap(); // Puedes ajustar el color según tus preferencias
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let dest = Rect::new(x, y, surface.width(), surface.height());
    canvas.copy(&texture, None, dest).unwrap();
}


fn main() -> Result<(), String> {
    let mut audio = Audio::new();
    audio.add("pew", "pew.wav");
    audio.add("move", "move.wav");
    audio.add("explode", "explode.wav");
    audio.add("loser","loser.wav");
    audio.add("win2","win2.wav");
    audio.add("win1","win1.wav");
    //audio.add("intro","abertura_invaders_sound.wav");
    audio.add("intro","intro.wav");
    audio.add("intro_win","intro_win.wav");
    audio.add("avion2","avion2.wav");
    audio.add("explosion2","explosion2.wav");
      


    let sdl_context = sdl2::init()?;

    
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Game Gustavo Villagra", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build()
        .map_err(|e| e.to_string())?;

    // Inicializar SDL2_image
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;

    let mut player = Player::new(&texture_creator)?; // Pasa el TextureCreator al constructor de Player

   
     // Carga la textura de explosión
     let mut game = Game::new(&texture_creator)?;

    let mut star_positions = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..289 { // Ajusta el número de estrellas según sea necesario
        let x = rng.gen_range(0..WIDTH as i32);
        let y = rng.gen_range(0..HEIGHT as i32);
        star_positions.push(Point::new(x, y));
    }



        // Dimensiones de la ventana
    let window_width = WIDTH as i32;
    let window_height = HEIGHT as i32;

    // Dimensiones de la imagen 
    let message_width = 850; // El ancho de tu imagen
    let message_height = 690; // El alto de tu imagen

    // Calcular la posición para centrar la imagen en la pantalla
    let message_x = (window_width - message_width) / 2;
    let message_y = (window_height - message_height) / 2;

    // let final_texture=texture_creator.load_texture("resource/felicidades.png")?;

    let win_texture = texture_creator.load_texture("resource/navedecombate.png")?;

    let start_texture = texture_creator.load_texture("resource/mujerUniverso.png")?;


 // Colores base
    let base_color_mujer = Color::RGB(0, 80, 120); // Azul oscuro
    let blend_color_mujer =Color::RGB(0, 0, 0); // Fondo negro
    
    // Factor de mezcla (ajusta según preferencias)
    let blend_factor_mujer = 0.85;
    
    // Calcula los componentes RGB mezclados
    let blended_a = lerp(base_color_mujer.r, blend_color_mujer.r, blend_factor_mujer);
    let blended_b = lerp(base_color_mujer.g, blend_color_mujer.g, blend_factor_mujer);
    let blended_c = lerp(base_color_mujer.b, blend_color_mujer.b, blend_factor_mujer);


    let message_rect = Rect::new(message_x, message_y, message_width as u32, message_height as u32);

    canvas.set_draw_color(Color::RGB(blended_a, blended_b, blended_c));
    canvas.clear();
    canvas.copy(&start_texture, None, Some(message_rect)).unwrap();
    canvas.present(); 

    std::thread::sleep(Duration::from_secs(2)); 
 
    // Número total de imágenes
    const NUM_IMAGES: usize = 202;

    // Duración total deseada en segundos
    const TOTAL_DURATION_SECONDS: u64 = 17;
    
    // Función para generar las rutas de las imágenes
    fn generate_image_paths() -> Vec<String> {

        
        (0..NUM_IMAGES)
            .map(|i| format!("resource/abertura_invaders_{:03}.png", i))
            .collect()
    }
    

       // Obtener las rutas de las imágenes
       let image_paths = generate_image_paths();

       // Cargar las imágenes como texturas
       let mut textures = Vec::new();
       
       for path in &image_paths {
           let texture = texture_creator.load_texture(path)?;
           textures.push(texture);
       }

    

       fn lerp(a: u8, b: u8, t: f32) -> u8 {
        ((1.0 - t) * f32::from(a) + t * f32::from(b)).round() as u8
    }
    
    // Colores base
    let base_color = Color::RGB(135, 206, 235); // Azul más intenso
    let blend_color = Color::RGB(192, 192, 192); // Blanco

    // Factor de mezcla (ajusta según preferencias)
    let blend_factor = 0.8; // Puedes ajustar este valor para controlar la intensidad del degradado

    // Calcula los componentes RGB mezclados
    let blended_r = lerp(base_color.r, blend_color.r, blend_factor);
    let blended_g = lerp(base_color.g, blend_color.g, blend_factor);
    let blended_b = lerp(base_color.b, blend_color.b, blend_factor);



    // Reproducir el sonido "intro"
    let intro_sound = "intro";
    audio.play(intro_sound);
        
      // Calcular el tiempo de espera entre cada imagen
    let time_per_image = TOTAL_DURATION_SECONDS * 1000 / NUM_IMAGES as u64;

     //Muestra cada textura en secuencia
    for (_i, texture) in textures.iter().enumerate() {
    canvas.set_draw_color(Color::RGB(blended_r, blended_g, blended_b));
    canvas.clear();
    canvas.copy(&texture, None, Some(message_rect)).unwrap();
    canvas.present();

    // Esperar un tiempo calculado para dar efecto de animación
    std::thread::sleep(Duration::from_millis(time_per_image));
    }

    let mut level = 1; // determinar el nivel actual

    let mut live=3;

    let mut lost_life = false;


    //configuracion del video loser
      // Número total de imágenes
      const NUM_IMAGES_LOSER: usize = 117;

      // Duración total deseada en segundos
      const TOTAL_DURATION_SECONDS_LOSER: u64 = 5;
      
      // Función para generar las rutas de las imágenes
      fn generate_image_paths_loser() -> Vec<String> {     
        
          (0..NUM_IMAGES_LOSER)
              .map(|i| format!("resource/WhatsApp Video 2024-02-01 at 18.34.06_{:03}.png", i))
              .collect()
      }
 
    //configuracion del video win
     // Número total de imágenes
      const NUM_IMAGES_WIN: usize = 152;

      // Duración total deseada en segundos
      const TOTAL_DURATION_SECONDS_WIN: u64 = 13;
      
      // Función para generar las rutas de las imágenes
      fn generate_image_paths_win() -> Vec<String> {     
        
          (0..NUM_IMAGES_LOSER)
              .map(|i| format!("resource/intro2_{:03}.png", i))
              .collect()
      }
 

    'game: loop { 
        let mut invaders = Invaders::new(&texture_creator, level)?;
        let mut last_update = Instant::now();

        'gameloop: loop {
            let now = Instant::now();
            let delta_time = now.duration_since(last_update).as_secs_f32();
            last_update = now;
    
           
            // Manejo de eventos
            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                        if !player.is_hit {
                            // Solo mueve a la izquierda si el jugador no ha sido alcanzado
                            audio.play("move");
                            player.move_left();
                        }
                    },
                    Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                        player.stop_left();
                        player.reset_velocity();
                    },
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                        if !player.is_hit {
                            audio.play("move");
                            player.move_right();
                        }
                    },
                    Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                        player.stop_right();
                        player.reset_velocity();
                    },
                    
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'game;
                    },                
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                          ..
                    } => {
                        if !player.is_hit {
                            audio.play("pew");
                            player.shoot();
                        }
                       
                    },
                    Event::KeyDown {
                        keycode: Some(Keycode::Y),
                        ..
                    } => {
                    
                        if game.is_game_over {
                            game.reset();
                          
                        } 
                        //player.reset_player_position();                  
                    },
                    
                    Event::KeyDown {
                        keycode: Some(Keycode::N),
                        ..
                    } => {

                        break 'game;
                       
                    },
                    _ => {}
                }
            }
    
    
            player.update(delta_time);


            canvas.set_draw_color(Color::RGB(0, 0, 0)); // Fondo negro
            canvas.clear();

            // Dibujar estrellas
            canvas.set_draw_color(Color::RGB(255, 255, 255)); // Color blanco para las estrellas
            for star in &star_positions {
                canvas.draw_point(*star).unwrap();
            }


            // Actualización de lógica del juego
            player.update_shots();

            if invaders.check_player_hits(&mut player.shots) {
                audio.play("explode");
            }

            invaders.update();
            
        
            player.draw(&mut canvas);
            

            invaders.draw(&mut canvas);
       
        
            // Verificar si todos los invasores han sido eliminados
            if invaders.all_invaders_inactive() {

                player.reset_player_position();

                audio.play("explosion2");               
                audio.wait();
                
                level += 1; // Incrementa el nivel

                if level > MAX_LEVEL {
                    audio.play("win1");
                }else{
                    audio.play("win2");
                }
                 
                let message_rect = Rect::new(message_x, message_y, message_width as u32, message_height as u32);
                canvas.copy(&win_texture, None, Some(message_rect)).unwrap();                
                canvas.present(); 
                               
               
                std::thread::sleep(Duration::from_secs(6)); // Esperar

                if level > MAX_LEVEL {
                        audio.play("intro_win");
                       // Obtener las rutas de las imágenes
                        let image_paths = generate_image_paths_win();
                
                        // Cargar las imágenes como texturas
                        let mut textures = Vec::new();
                
                        for path in &image_paths {
                            canvas.set_draw_color(Color::RGB(0, 0, 0));
                            canvas.clear();
                
                            let texture = texture_creator.load_texture(path)?;
                            textures.push(texture);
                        }
                
                        // Calcular el tiempo de espera entre cada imagen
                        let time_per_image = TOTAL_DURATION_SECONDS_WIN * 1000 / NUM_IMAGES_WIN as u64;
                
                        // Muestra cada textura en secuencia
                        for texture in textures {
                            canvas.clear();
                            canvas.copy(&texture, None, Some(message_rect)).unwrap();
                            canvas.present();
                
                            // Esperar un tiempo calculado para dar efecto de animación
                            std::thread::sleep(Duration::from_millis(time_per_image));
                        }

                        
                    // let message_rect = Rect::new(message_x, message_y, message_width as u32, message_height as u32);
                    // canvas.copy(&final_texture, None, Some(message_rect)).unwrap();                
                    // canvas.present();
                    // std::thread::sleep(Duration::from_secs(8)); // Espera 10 segundos
                    break 'game; // Salir del bucle 'game' si se han completado todos los niveles
                }

                break 'gameloop; // Salir del bucle 'gameloop' para comenzar el siguiente nivel
            }
           

            canvas.present();



            game.check_game_over(&mut player, &invaders);



            if game.is_game_over {
                player.stop_left();
                player.stop_right();
                player.reset_velocity();

                game.draw(&mut canvas, &mut player);
                canvas.present();
        
                audio.play("explosion2");
        
                live -= 1;
        
                std::thread::sleep(Duration::from_secs(1));
        
                audio.play("loser");
        
                // Obtener las rutas de las imágenes
                let image_paths = generate_image_paths_loser();
        
                // Cargar las imágenes como texturas
                let mut textures = Vec::new();
        
                for path in &image_paths {
                    canvas.set_draw_color(Color::RGB(0, 18, 0));
                    canvas.clear();
        
                    let texture = texture_creator.load_texture(path)?;
                    textures.push(texture);
                }
        
                audio.play("avion2");
        
                // Calcular el tiempo de espera entre cada imagen
                let time_per_image = TOTAL_DURATION_SECONDS_LOSER * 1000 / NUM_IMAGES_LOSER as u64;
        
                // Muestra cada textura en secuencia
                for texture in textures {
                    canvas.clear();
                    canvas.copy(&texture, None, Some(message_rect)).unwrap();
                    canvas.present();
        
                    // Esperar un tiempo calculado para dar efecto de animación
                    std::thread::sleep(Duration::from_millis(time_per_image));
                }


        
                if live > 0 {
                    lost_life = true;  
                                  
                    // Si hay vidas restantes, dar opción de continuar
                    if !ask_continue(&mut event_pump, &mut canvas) {
                        break 'game; // Salir del bucle del juego si el jugador decide no continuar
                    }
                   

                    let ttf_context = sdl2::ttf::init().unwrap();
                    let font_path = "Sixtyfour-Regular.ttf";
                    let font_size = 24;
                    let font = ttf_context.load_font(font_path, font_size).unwrap();
                    let font_color = Color::RGB(255, 0, 0); // Rojo

                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.clear();

                      render_text(
                        &mut canvas,
                        &font,
                        &format!("Vidas restantes: {}", live),
                        100,
                        300,
                        Some(font_color), // Pasa el color al renderizador de texto
                    );

                    canvas.present();
                    std::thread::sleep(Duration::from_secs(2));
             
          

                } else {

                    let ttf_context = sdl2::ttf::init().unwrap();
                    let font_path = "Sixtyfour-Regular.ttf";
                    let font_size = 64;
                    let font = ttf_context.load_font(font_path, font_size).unwrap();
                    let font_color = Color::RGB(255, 0, 0); // Rojo

                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.clear();

                    let text = "GAME OVER";
                    let text_width = font.size_of(text).map(|size| size.0).unwrap_or(0) as u32;
                    let text_height = font.size_of(text).map(|size| size.1).unwrap_or(0) as u32;

                    
                    let x = ((canvas.window().size().0 - text_width) / 2) as i32;
                    let y = ((canvas.window().size().1 - text_height) / 2) as i32;
                    
                    
                    render_text(
                        &mut canvas,
                        &font,
                        text,
                        x,
                        y,
                        Some(font_color),
                    );
                    canvas.present();
                    std::thread::sleep(Duration::from_secs(4));
                    
                    break 'game; // Salir del bucle del juego si no hay vidas restantes
                }
            }

          
            if lost_life {

                // Llamada al método reset en tu objeto game
                match invaders.reset(&texture_creator, level,&mut player) {
              
                    Ok(_) => (),
                    Err(err) => {
                        eprintln!("Error al restablecer el juego: {}", err);
                        break 'game;
                    }
                }

                lost_life = false; // Restablecer el indicador de pérdida de vida

                continue 'gameloop;              
            }
        
            std::thread::sleep(Duration::from_millis(5));
        }
        
    }

        Ok(())
}
