pub mod block;
pub use block::*;
pub use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
pub use std::io::{stdout, Write};

pub struct Frame {
    pub frame: Vec<u16>,
    pub block: Option<Block>,
    pub next_block: Option<Block>,
    pub score: u32,
    pub level: u32,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            frame: {
                let mut upper: Vec<u16> = vec![0b100000000001; 24];
                upper.push(0b111111111111);
                upper.push(0b111111111111);
                upper
            },
            block: Some(Block::new(rand::random(), Direction::Up)),
            next_block: None,
            score: 0,
            level: 1,
        }
    }

    pub fn set_level(&mut self) {
        match self.score {
            0..=1000 => self.level = 1,
            1001..=3000 => self.level = 2,
            3001..=5000 => self.level = 3,
            5001..=10000 => self.level = 4,
            _ => self.level = 5,
        }
    }

    pub fn generate_block(&mut self) {
        if self.next_block == None {
            self.next_block = Some(Block::new(rand::random(), Direction::Up))
        }
    }

    pub fn print_next_block(&self) -> String {
        let mut result = String::new();
        if let Some(block) = self.next_block.clone() {
            let shape = block.draw();
            for f_row in 0..4 {
                for f_col in 0..4 {
                    if shape[f_row] & (1 << (3 - f_col)) != 0 {
                        result.push_str("⚪️"); // ⚪️
                    } else {
                        result.push_str("  ");
                    }
                }
                result.push_str("\n");
            }
        }
        result
    }

    pub fn print_frame(&self) -> String {
        let mut result = String::new();
        if let Some(block) = self.block.clone() {
            let (row, mut col) = block.pos;
            if 11 - 4 - col < 0 {
                col = 11 - 4;
            }
            let shape = block.draw();

            for f_row in 3..(self.frame.len() as i16 - 1) {
                if (row..(row + 4)).contains(&(f_row)) {
                    result.push_str(&row_str(
                        self.frame[f_row as usize]
                            | shape[(f_row - row) as usize] << (11 - 4 - col),
                    ));
                } else {
                    result.push_str(&row_str(self.frame[f_row as usize]));
                }
            }
        }

        return result;

        fn row_str(row: u16) -> String {
            // let mut stdout = stdout();
            let mut result = String::new();
            for i in 0..12 {
                if row & (1 << (11 - i)) != 0 {
                    result.push_str("⚪️"); // ⚪️██
                                           // print!("⬜️");
                } else {
                    result.push_str("⬛️");
                    // print!("⬛️");
                }
            }
            result.push('\n');
            result
        }
    }

    // check if a block is collided
    fn is_collided(&self, block: &Block) -> bool {
        let (row, col) = block.pos;
        for r in row..(row + 4) {
            let shape = block.draw();
            if shape[(r - row) as usize] == 0 {
                continue;
            }
            if r == 24 {
                return true;
            }
            if 11 - 4 - col > 0 {
                if self.frame[r as usize] & (shape[(r - row) as usize] << (11 - 4 - col)) != 0 {
                    return true;
                }
            } else {
                if self.frame[r as usize] & (shape[(r - row) as usize] >> (col - 7)) != 0 {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_game_over(&self) -> bool {
        self.frame[3] != 0b100000000001
    }

    // try to move a block
    fn move_block(&self, dir: Direction) -> Option<Block> {
        if let Some(mut block) = self.block.clone() {
            match dir {
                Direction::Left => block.pos.1 -= 1,
                Direction::Right => block.pos.1 += 1,
                Direction::Down => block.pos.0 += 1,
                Direction::Up => block.rotate(),
            }
            return Some(block);
        }
        None
    }

    // confirm one move
    pub fn set_move(&mut self, dir: Direction) {
        if let Some(block) = self.move_block(dir) {
            match self.is_collided(&block) {
                false => self.block = Some(block),
                true => {
                    // panic!("collided");
                    if dir == Direction::Down {
                        while self.is_collided(&mut self.block.as_ref().expect("no block")) {
                            self.set_move(Direction::Up);
                        }
                        self.set_block();
                        self.collapse();
                    }
                }
            }
        }
    }

    // set a block into frame
    pub fn set_block(&mut self) {
        if let Some(block) = self.block.take() {
            let (row, col) = block.pos;
            for r in row..(row + 4) {
                let shape = block.draw();
                self.frame[r as usize] |= shape[(r - row) as usize] << (11 - 4 - col);
            }
            self.block = self.next_block.take();
        }
    }

    pub fn collapse(&mut self) {
        let mut count = 0;
        for r in 0..24 {
            // println!("{}", self.frame[r]);
            if self.frame[r] == 0b111111111111 {
                self.score += 1000 + count * 400;
                count += 1;
                self.frame.remove(r);
                self.frame.insert(0, 0b100000000001);
            }
        }
    }
}
