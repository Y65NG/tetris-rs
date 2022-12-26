mod frame;

use device_query::{DeviceQuery, DeviceState, Keycode};
use frame::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn create_game() {
    let (tx, rx) = mpsc::channel();
    let mut frame = Frame::new();
    let l1 = thread::spawn(move || loop {
        if frame.block == None {
            frame.generate_block();
        }
        print_screen(&frame);
        block_falling(&mut frame);
        // frame.set_move(Direction::Right);
        frame.collapse();
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
    // let tx = tx.clone();
    let l2 = thread::spawn(move || user_control(tx));
    l1.join().unwrap();
    l2.join().unwrap();
}

fn block_falling(frame: &mut Frame) {
    frame.set_move(Direction::Down);
}

fn user_control(tx: mpsc::Sender<Keycode>) {
    let device_state = DeviceState::new();
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys {
            tx.send(key).expect("Failed to send key");
            thread::sleep(Duration::from_millis(200));
            continue;
        }
    }
}

fn print_screen(frame: &Frame) {
    frame.print_frame();
    thread::sleep(Duration::from_millis(300));
    clear_screen();
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}
