pub mod frame;

use crate::ui::*;
use crate::ui2::*;
use device_query::{DeviceQuery, DeviceState, Keycode};
use frame::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

static FRAME_WIDTH: i32 = 300;
static FRAME_HEIGHT: i32 = 600;
static BLOCK_WIDTH: i32 = 10;

pub fn create_game() {
    // thread::sleep(Duration::from_millis(2000));
    let (tx_control, rx_control) = mpsc::channel();
    let (tx_game, rx_game) = mpsc::channel();
    let (tx_falling, rx_falling) = mpsc::channel();
    let (tx_level, rx_level) = mpsc::channel();
    let mut frame = Frame::new();
    // let mut stdout = stdout();
    let mut terminal = init_terminal().unwrap();
    let mut level = 1;

    let l1 = thread::spawn(move || loop {
        // generate new blocks
        if frame.next_block == None {
            frame.generate_block();
        }

        // check if the game is over
        if frame.is_game_over() {
            println!("Game Over");
            tx_game.send("Game Over").expect("Failed to send game over");
            end_terminal(&mut terminal).unwrap();
            return;
        }

        // check and increase level
        frame.set_level();
        tx_level.send(frame.level).expect("Failed to send level");

        // draw ui
        draw_ui(&frame, &mut terminal, frame.level).unwrap();

        // receive control message
        match rx_control.try_recv() {
            Ok(key) => match key {
                Keycode::A | Keycode::Left => frame.set_move(Direction::Left),
                Keycode::D | Keycode::Right => frame.set_move(Direction::Right),
                Keycode::S | Keycode::Down => frame.set_move(Direction::Down),
                Keycode::W | Keycode::Up => frame.set_move(Direction::Up),
                Keycode::Q => {
                    tx_game.send("Game Over").expect("Failed to send game over");
                    end_terminal(&mut terminal).unwrap();
                    return;
                }
                _ => (),
            },
            Err(_) => (),
        };
        // receive falling message
        match rx_falling.try_recv() {
            Ok(_) => {
                frame.set_move(Direction::Down);
            }
            Err(_) => (),
            // Err(_) => frame.set_move(Direction::Down),
        };
    });

    let l2 = thread::spawn(move || {
        user_control(tx_control, rx_game);
    });

    let l3 = thread::spawn(move || {
        block_falling(tx_falling, rx_level);
    });

    l1.join().expect("Failed to join l1");
    l2.join().expect("Failed to join l2");
    l3.join().expect("Failed to join l3");
}

fn block_falling(tx_falling: mpsc::Sender<u32>, rx_level: mpsc::Receiver<u32>) {
    loop {
        let level = match rx_level.try_recv() {
            Ok(l) => l,
            Err(_) => 1,
        };

        let time = (400 - level * 100) as u64;
        thread::sleep(Duration::from_millis(time));
        tx_falling
            .send(time as u32)
            .expect("Failed to send falling");
    }
}

fn user_control(tx: mpsc::Sender<Keycode>, rx1: mpsc::Receiver<&str>) {
    let device_state = DeviceState::new();
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys {
            tx.send(key).expect("Failed to send key");
            thread::sleep(Duration::from_millis(150));
            continue;
        }
        match rx1.try_recv() {
            Ok(_) => return,
            Err(_) => (),
        }
    }
}
