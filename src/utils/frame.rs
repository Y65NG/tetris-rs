pub mod block;
pub use block::*;

pub struct Frame {
    frame: Vec<u16>,
    pub block: Option<Block>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            frame: {
                let mut upper: Vec<u16> = vec![0b100000000001; 24];
                upper.push(0b111111111111);
                upper
                // <Vec<u16> as TryInto<[u16; 25]>>::try_into(upper).unwrap()
            },
            block: None,
        }
    }

    pub fn generate_block(&mut self) {
        if self.block == None {
            self.block = Some(Block::new(rand::random(), Direction::Up))
        }
    }

    pub fn print_frame(&self) {
        println!("",);
        fn print_row(row: u16) {
            for i in 0..12 {
                if row & (1 << (11 - i)) != 0 {
                    print!("⬜️");
                } else {
                    print!("⬛️");
                }
            }
            println!();
        }

        if let Some(block) = self.block.clone() {
            let (row, col) = block.pos;
            let shape = block.draw();

            for f_row in 0..(self.frame.len() as i16) {
                if (row..(row + 4)).contains(&(f_row)) {
                    print_row(
                        self.frame[f_row as usize]
                            | shape[(f_row - row) as usize] << (11 - 4 - col),
                    );
                } else {
                    print_row(self.frame[f_row as usize]);
                }
            }
        } else {
            for f_row in 0..self.frame.len() {
                println!("{:b}", self.frame[f_row]);
            }
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

            if self.frame[r as usize] & (shape[(r - row) as usize] << (11 - 4 - col)) != 0 {
                return true;
            }
        }
        false
    }

    pub fn is_game_over(&self) -> bool {
        self.frame[3] != 0
    }

    pub fn fill_row(&mut self, row: usize) {
        self.frame[row] = 0b111111111111;
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
                        self.set_block();
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
        }
    }

    pub fn collapse(&mut self) {
        for r in 0..24 {
            // println!("{}", self.frame[r]);
            if self.frame[r] == 0b111111111111 {
                self.frame.remove(r);
                // println!("yes {}", row);
                self.frame.insert(0, 0b100000000001);
            }
        }
    }
}
