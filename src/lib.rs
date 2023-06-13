use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "www/utils/random.js")]
extern "C" {
    fn random(max: usize) -> usize;
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}


#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(name);
}

#[wasm_bindgen]
pub struct World {
    width:usize,
    size:usize,
    reward_cell: Option<usize>,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    status: Option<GameStates>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width:usize, snake_idx:usize) -> Self{
        let size = width * width;
        let snake = Snake::new(snake_idx,3);
        Self {
            width,
            size:width * width,
            reward_cell: Some(World::gen_reward_cell(size, &snake.body.clone())),
            snake,
            next_cell:None,
            status:None
        }
    }

    pub fn start_game(&mut self) {
        self.status = Some(GameStates::Played);
    }

    pub fn get_game_status(&self) -> Option<GameStates> {
        self.status
    }

    pub fn get_game_status_info(&self) -> String {
        match self.status {
            Some(GameStates::Lose) => "You Lose!".to_string(),
            Some(GameStates::Win) => "You Win!".to_string(),
            Some(GameStates::Played) => "You're Playing!".to_string(),
            None => "None!".to_string(),
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);
        if self.snake.body[1].0 == next_cell.0 { return; }

        self.snake.direction = direction;
    }

    pub fn update(&mut self) {
        let tmp = self.snake.body.clone();
        //Optimizing
        match self.next_cell {
            None => {
                self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
            }
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None;
            }
        }
        let len = self.snake.body.len();
        for i in 1..len {
            self.snake.body[i] = SnakeCell(tmp[i-1].0);
        }

        if self.snake.body[1..len].contains(&self.snake.body[0]) {
            self.status = Some(GameStates::Lose);
        }
        if self.reward_cell == Some(self.snake_head_idx()) {
            if self.get_snake_len() < self.size {
                self.reward_cell = Some(World::gen_reward_cell(self.size, &self.snake.body));
                self.snake.body.push(SnakeCell(self.snake.body[1].0))
            } else {
                self.reward_cell = None;
                self.status = Some(GameStates::Win)
            }
        }
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell{
        let snake_idx = self.snake_head_idx();
        let row = snake_idx/self.width;
        let col = snake_idx % self.width;
        return match direction {
            Direction::Up => {
                let border = col;
                if snake_idx == border {
                    SnakeCell((self.size - self.width) + border)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            }
            Direction::Down => {
                let border = (self.size - self.width) + col;
                if snake_idx == border {
                    SnakeCell(border - self.width *(row+1))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            }
            Direction::Left => {
                let border = row * self.width;
                if snake_idx == border {
                    SnakeCell(border + self.width - 1)
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
            Direction::Right => {
                let border = (row+1) * self.width;
                if snake_idx + 1 == border {
                    SnakeCell(border - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            }
        }
    }

    fn gen_reward_cell(max:usize, snake_body: &Vec<SnakeCell>) -> usize {
        let mut reward_cell;
        loop {
            reward_cell = random(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break reward_cell;
            }
        }
    }

    pub fn get_reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn get_snake_len(&self) -> usize {
        self.snake.body.len()
    }
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum GameStates{
    Win,
    Lose,
    Played
}


#[derive(Copy, Clone, PartialEq)]
pub struct SnakeCell(usize);


struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_idx:usize, size:usize) -> Self{
        let mut body = Vec::new();
        for i in 0..size {
            body.push(SnakeCell(spawn_idx - i))
        }
        Self{
            body,
            direction: Direction::Down,
        }
    }
}
