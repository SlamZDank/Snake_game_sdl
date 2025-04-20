use clap::{command, Arg};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::Window;
use sdl2::video::WindowContext;
use snake_game_sdl::model::Direction;
use snake_game_sdl::*;
use std::env;
use std::str::from_utf8;
use std::time::Duration;

const WINDOW_TITLE: &str = "Snake 🍎 🐍";
const SQUARE_SIZE: u32 = 16;
const PIXEL_PADDING: u32 = 3;

struct WindowProperties {
    window_height: u32,
    window_width: u32,
    window_title: String,
}

fn is_within_padded_area(x: u32, y: u32, small: bool) -> bool {
    let mut padding = PIXEL_PADDING;
    if small {
        padding = PIXEL_PADDING + 2;
    }
    let padded_start = padding;
    let padded_end = SQUARE_SIZE - padding;

    (x >= padded_start && x <= padded_end) && (y >= padded_start && y <= padded_end)
}

fn dummy_texture<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<(Texture<'a>, Texture<'a>, Texture<'a>), String> {
    enum TextureColors {
        Red,
        Green,
        White,
    }

    let mut snake_body = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut apple = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut collision = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;

    {
        let textures = [
            (&mut snake_body, TextureColors::Green),
            (&mut apple, TextureColors::Red),
            (&mut collision, TextureColors::White),
        ];

        // to maybe add some margins to make it prettier
        canvas
            .with_multiple_texture_canvas(textures.iter(), |texture_canvas, user_context| {
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                texture_canvas.clear();
                match *user_context {
                    TextureColors::Green => {
                        // we draw pixel by pixel the snake body (it's just a rectangle)
                        let small = false;
                        for y in 0..SQUARE_SIZE {
                            for x in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(0, 255, 0));
                                if is_within_padded_area(x, y, small) {
                                    texture_canvas
                                        .draw_point(Point::new(x as i32, y as i32))
                                        .expect("failed to draw point");
                                }
                            }
                        }
                    }

                    TextureColors::Red => {
                        // we draw pixel by pixel the apple (it's just a rectangle)
                        let small = true;
                        for y in 0..SQUARE_SIZE {
                            for x in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(255, 0, 0));
                                if is_within_padded_area(x, y, small) {
                                    texture_canvas
                                        .draw_point(Point::new(x as i32, y as i32))
                                        .expect("failed to draw point");
                                }
                            }
                        }
                    }

                    TextureColors::White => {
                        // we draw pixel by pixel the collition (it's just a rectangle)
                        for y in 0..SQUARE_SIZE {
                            for x in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
                                texture_canvas
                                    .draw_point(Point::new(x as i32, y as i32))
                                    .expect("failed to draw point");
                            }
                        }
                    }
                }
            })
            .map_err(|e| e.to_string())?;
    }
    Ok((snake_body, apple, collision))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matched_args = command!()
        .version("1.0.1 CyberTrace Edition")
        .author("SlamZDank")
        .about("A simple to use yet configurable snake game using sdl, CyberTrace Edition.")
        .arg(
            Arg::new("width")
                .long("width")
                .help("Sets the width of the game board by grid square size.")
                .required(false)
                .value_parser(clap::value_parser!(u32).range(21..=1000)),
        )
        .arg(
            Arg::new("height")
                .long("height")
                .help("Sets the of the game board by grid square size.")
                .required(false)
                .value_parser(clap::value_parser!(u32).range(21..=1000)),
        )
        .arg(
            Arg::new("border")
                .long("border")
                .help("Sets the border of the game board by grid square size.")
                .required(false)
                .value_parser(clap::value_parser!(bool)),
        )
        .arg(
            Arg::new("obstacles")
                .long("obstacles")
                .help("Sets the number of obstacles generated randomly throughout the board.")
                .required(false)
                .value_parser(clap::value_parser!(u32).range(0..=100)),
        )
        .arg(
            Arg::new("tickspeed")
                .long("tickspeed")
                .help("Sets the tick speed to a value in milliseconds.")
                .required(false)
                .value_parser(clap::value_parser!(u32).range(10..=5000)),
        )
        .get_matches();

    let width: u32 = match matched_args.get_one::<u32>("width") {
        Some(selected_width) => *selected_width,
        None => 40,
    };

    let height: u32 = match matched_args.get_one::<u32>("height") {
        Some(selected_height) => *selected_height,
        None => 40,
    };

    let obstacles: u32 = match matched_args.get_one::<u32>("obstacles") {
        Some(selected_obstacles) => *selected_obstacles,
        None => 42,
    };

    let border: bool = match matched_args.get_one::<bool>("border") {
        Some(is_border_allowed) => *is_border_allowed,
        None => false,
    };

    let tickspeed: u32 = match matched_args.get_one::<u32>("tickspeed") {
        Some(selected_tickspeed) => *selected_tickspeed,
        None => 150,
    };

    // this variable makes sdl run it in wayland
    let is_wayland = env::var("WAYLAND_DISPLAY").is_ok();
    if is_wayland {
        env::set_var("SDL_VIDEODRIVER", "wayland");
    } // set the Operating system environment to run on wayland if possible

    let window_property = WindowProperties {
        window_height: height * SQUARE_SIZE,
        window_width: width * SQUARE_SIZE,
        window_title: WINDOW_TITLE.to_string(),
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            &window_property.window_title,
            window_property.window_width,
            window_property.window_height,
        )
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let (texture_snake, texture_apple, texture_collision) =
        dummy_texture(&mut canvas, &texture_creator)?;

    let mut game = SnakeGame::from(width, height, border, (obstacles > 0, obstacles)); // the initialization of the game
    print!("{}[2J", 27_u8 as char);
    println!("SDL Renderer: \"{}\"", canvas.info().name);
    println!("Welcome!\nPress <P> to pause!\nPress <R> to restart!");

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        // there's a delay when the key is pressed to add some natural tactility feel
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running, // to escape the window
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    game = SnakeGame::from(width, height, border, (obstacles > 0, obstacles));

                    print!("{}[2J", 27_u8 as char);
                    println!("Welcome!\nPress <P> to pause!\nPress <R> to restart!");
                    // the initialization of the game
                } // The R key restarts the game
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => game.toggle_pause(),
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    game.resume();
                    game.change_head_direction(Direction::UP);
                    std::thread::sleep(Duration::from_millis(50));
                } // UP Key
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    game.resume();
                    game.change_head_direction(Direction::RIGHT);
                    std::thread::sleep(Duration::from_millis(50));
                } // Right Key
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    game.resume();
                    game.change_head_direction(Direction::DOWN);
                    std::thread::sleep(Duration::from_millis(50));
                } // Down Key
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    game.resume();
                    game.change_head_direction(Direction::LEFT);
                    std::thread::sleep(Duration::from_millis(50));
                } // Left key
                _ => {}
            }
        }
        // the start of the game loop

        let mut old_score = game.snake.body.len();

        game.update();

        if game.state != GameState::GameOver && game.snake.body.len() != old_score {
            print!("{}[2J", 27_u8 as char);
            if width == 55
                && height == 35
                && border
                && game.snake.body.len() == 25
                && tickspeed == 150
            {
                println!(
                    "{}",
                    from_utf8(&xor_obfuscate(&ENCRYPTED_MESSAGE, &KEY)).unwrap()
                );
            } else {
                println!("Welcome!");
            }
            println!("Press <P> to pause!\nPress <R> to restart!");
            println!("Score: {}", game.snake.body.len());
            old_score = game.snake.body.len();
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        //canvas.set_draw_color(Color::RGB(255, 255, 255)); // not needed since the textures have
        //their own color applied to them

        // use the game provider to draw the entities
        let _ = canvas.copy(
            &texture_apple,
            None,
            Rect::new(
                game.apple.position.0 as i32 * SQUARE_SIZE as i32,
                game.apple.position.1 as i32 * SQUARE_SIZE as i32,
                SQUARE_SIZE,
                SQUARE_SIZE,
            ),
        );

        for snake_part in game.snake.body.iter() {
            let _ = canvas.copy(
                &texture_snake,
                None,
                Rect::new(
                    snake_part.get().position.0 as i32 * SQUARE_SIZE as i32,
                    snake_part.get().position.1 as i32 * SQUARE_SIZE as i32,
                    SQUARE_SIZE,
                    SQUARE_SIZE,
                ),
            );
        }
        if game.field.obstacles.is_some() {
            for obstacle_part in game
                .field
                .obstacles
                .as_ref()
                .expect("Illegal game instruction")
            {
                let _ = canvas.copy(
                    &texture_collision,
                    None,
                    Rect::new(
                        obstacle_part.0 as i32 * SQUARE_SIZE as i32,
                        obstacle_part.1 as i32 * SQUARE_SIZE as i32,
                        SQUARE_SIZE,
                        SQUARE_SIZE,
                    ),
                );
            }
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(tickspeed as u64));

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // sloppy FPS limit for
                                                                       // weird stuff
    }

    Ok(())
}
