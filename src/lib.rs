pub mod model;
use model::{Apple, Field, Snake};
use rand::Rng;

// to make them global
pub const PLAYGROUND_WIDTH: u32 = 21;
pub const PLAYGROUND_HEIGHT: u32 = 21;
pub const HAS_BORDER: bool = true;

// to redo the xor encryption
pub const ENCRYPTED_MESSAGE: [u8; 51] = [
    0x21, 0x1e, 0x3, 0x4a, 0x58, 0x00, 0x0d, 0x07, 0x1c, 0x53, 0x54, 0x09, 0x4f, 0x32, 0x0e, 0x1c,
    0x3c, 0x55, 0x1b, 0x4f, 0x5, 0x52, 0x18, 0x5, 0x10, 0xf, 0x10, 0xc, 0x47, 0x10, 0x7, 0x45,
    0x17, 0x4f, 0x19, 0xa, 0x54, 0x4, 0xc, 0xf, 0x15, 0x4e, 0x48, 0x17, 0x7, 0x16, 0x4b, 0x1f,
    0x51, 0x50, 0x11,
];

pub const KEY: [u8; 51] = [
    0x68, 0x6d, 0x6d, 0x6d, 0x2c, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x74, 0x68, 0x6f, 0x75, 0x67, 0x68,
    0x74, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x67, 0x65, 0x74, 0x20,
    0x73, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x68, 0x65, 0x72, 0x65, 0x3f, 0x20,
    0x6e, 0x6f, 0x2e,
];

pub fn xor_obfuscate(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(idx, &byte)| byte ^ key[idx % key.len()])
        .collect()
}

pub fn get_message() -> Vec<u8> {
    Vec::from([])
}

#[derive(PartialEq, Debug)]
pub enum GameState {
    Running,
    Paused,
    GameOver,
}

pub struct SnakeGame {
    pub state: GameState,
    pub field: Field,
    pub snake: Snake, // dummy field and to chaage
    pub apple: Apple,
}

fn check_if_occupied_by_obstacle_or_snake(place: (u32, u32), snake: &Snake, field: &Field) -> bool {
    for body_part in snake.body.iter() {
        if body_part.get().position.0 == place.0 && body_part.get().position.1 == place.1 {
            return true;
        }
    }

    if field.obstacles.is_some() {
        for obstacle_part in field.obstacles.as_ref().expect("this is gay") {
            if obstacle_part.0 == place.0 && obstacle_part.1 == place.1 {
                return true;
            }
        }
    }
    false
}

fn check_if_occupied_by_apple(snake_head: (u32, u32), apple_position: (u32, u32)) -> bool {
    snake_head.0 == apple_position.0 && snake_head.1 == apple_position.1
}

// broken implementation
fn check_collision(snake: &mut Snake, field: &Field) -> bool {
    // check the snake head if it has the same position as an obstacle or one of its body parts
    let snake_head = snake.body[0].get().position;
    for body_part in &snake.body[1..] {
        if snake_head.0 == body_part.get().position.0 && snake_head.1 == body_part.get().position.1
        {
            snake.grow_snake(field.size.0, field.size.1);
            println!("COLLISION DETECTED WITH SNAKES");
            return true;
        }
    }

    if field.obstacles.is_some() {
        for obstacle_part in field.obstacles.as_ref().expect("this is gay") {
            if snake_head.0 == obstacle_part.0 && snake_head.1 == obstacle_part.1 {
                snake.grow_snake(field.size.0, field.size.1);
                return true;
            }
        }
    }
    false
}

// need to add that to the game
fn no_free_space_available(snake: &Snake, field: &Field) -> bool {
    let total_number_space_available = PLAYGROUND_WIDTH * PLAYGROUND_HEIGHT;

    if field.obstacles.is_some() {
        return total_number_space_available
            - snake.body.len() as u32
            - field.obstacles.as_ref().expect("this is gay").len() as u32
            == 0;
    }
    total_number_space_available - snake.body.len() as u32 == 0
}

impl SnakeGame {
    pub fn from(width: u32, height: u32, has_border: bool, obstacles: (bool, u32)) -> Self {
        Self {
            state: GameState::Paused,
            field: Field::from((width, height), has_border, obstacles.0, obstacles.1),
            // the initial position of the snake must be determined
            snake: Snake::add_to_field_at_start(width / 3, height / 2),
            apple: Apple::add_to_field_at_start((width * 2) / 3, height / 2),
        }
    }

    pub fn new() -> Self {
        Self {
            state: GameState::Paused,
            field: Field::from((PLAYGROUND_WIDTH, PLAYGROUND_HEIGHT), false, false, 0),
            // the initial position of the snake must be determined
            snake: Snake::add_to_field_at_start(PLAYGROUND_WIDTH / 3, PLAYGROUND_HEIGHT / 2),
            apple: Apple::add_to_field_at_start((PLAYGROUND_WIDTH * 2) / 3, PLAYGROUND_HEIGHT / 2),
        }
    }

    pub fn resume(&mut self) {
        if self.state == GameState::GameOver {
            return;
        }
        self.state = GameState::Running;
    }

    pub fn toggle_pause(&mut self) {
        if self.state == GameState::Running {
            self.state = GameState::Paused;
        } else if self.state == GameState::Paused {
            self.state = GameState::Running;
        }
    }

    pub fn change_head_direction(&mut self, direction: model::Direction) {
        self.snake.set_direction(direction);
    }

    pub fn update(&mut self) {
        if self.state != GameState::Running {
            return;
        };

        self.snake.move_snake(self.field.size.0, self.field.size.1);

        let (mut x, mut y);
        // check if the snake has eaten an apple
        if check_if_occupied_by_apple(self.snake.body[0].get().position, self.apple.position) {
            let mut rng = rand::thread_rng();
            self.snake.grow_snake(self.field.size.0, self.field.size.1);
            loop {
                (x, y) = (
                    rng.gen_range(1..self.field.size.0 - 1),
                    rng.gen_range(1..self.field.size.1 - 1),
                );
                if !check_if_occupied_by_obstacle_or_snake((x, y), &self.snake, &self.field) {
                    break;
                }
            }
            self.apple.place(x, y);
        }

        // check if the snake has hit the wall
        if check_collision(&mut self.snake, &self.field) {
            self.state = GameState::GameOver;
            print!("{}[2J", 27_u8 as char);
            println!(
                "Game Over!\nScore: {}\nPress <R> to restart!",
                self.snake.body.len() - 1
            );
            return;
        }

        if no_free_space_available(&self.snake, &self.field) {
            self.state = GameState::GameOver;
            print!("{}[2J", 27_u8 as char);
            println!(
                "You Win!\nScore: {}\nPress <R> to restart!",
                self.snake.body.len() - 1
            );
        }
    }
}

impl Default for SnakeGame {
    fn default() -> Self {
        Self::new()
    }
}
