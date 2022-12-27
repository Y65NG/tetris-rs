mod frame;

use device_query::{DeviceQuery, DeviceState, Keycode};
use frame::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn create_game() {
    let (tx, rx) = mpsc::channel();
    let mut frame = Frame::new();
    let (tx1, rx1) = mpsc::channel();
    let l1 = thread::spawn(move || loop {
        if frame.block == None {
            frame.generate_block();
        }
        if frame.is_game_over() {
            println!("Game Over");
            tx1.send("Game Over").expect("Failed to send game over");
            return;
        }
        print_screen(&frame);
        block_falling(&mut frame);
        match rx.try_recv() {
            Ok(key) => match key {
                Keycode::A => frame.set_move(Direction::Left),
                Keycode::D => frame.set_move(Direction::Right),
                Keycode::S => frame.set_move(Direction::Down),
                Keycode::W => frame.set_move(Direction::Up),
                _ => (),
            },
            Err(_) => (),
        };
    });
    let l2 = thread::spawn(move || {
        user_control(tx, rx1);
    });
    l1.join().unwrap();
    l2.join().unwrap();
}

fn block_falling(frame: &mut Frame) {
    frame.set_move(Direction::Down);
}

fn user_control(tx: mpsc::Sender<Keycode>, rx1: mpsc::Receiver<&str>) {
    let device_state = DeviceState::new();
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys {
            tx.send(key).expect("Failed to send key");
            thread::sleep(Duration::from_millis(300));
            continue;
        }
        match rx1.try_recv() {
            Ok(_) => return,
            Err(_) => (),
        }
    }
}

fn print_screen(frame: &Frame) {
    frame.print_frame();
    thread::sleep(Duration::from_millis(300));
    clear_screen();

    fn clear_screen() {
        print!("{}[2J", 27 as char);
    }
}
