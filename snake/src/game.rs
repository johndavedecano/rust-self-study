use piston_window::types::Color;
use piston_window::*;

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};
use rand::{thread_rng, Rng};

const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const MOVING_PERIOD: f64 = 0.2;
const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,
    waiting_time: f64,
    game_over: bool,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            width,
            height,
            food_exists: false,
            food_x: 0,
            food_y: 0,
            waiting_time: 0.0,
            game_over: false,
        }
    }

    pub fn key_press(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir: Option<Direction> = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction()),
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn add_food(&mut self) {
        let mut rnd = thread_rng();

        let mut new_x = rnd.gen_range(1..self.width - 1);
        let mut new_y = rnd.gen_range(1..self.height - 1);

        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rnd.gen_range(1..self.width - 1);
            new_y = rnd.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = false;
        self.food_x = 0;
        self.food_y = 0;
        self.game_over = false;
    }
}
