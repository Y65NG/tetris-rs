mod frame;

use device_query::{DeviceQuery, DeviceState, Keycode};
use frame::*;
use std::io::Stdout;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn create_game() {
    let (tx_control, rx_control) = mpsc::channel();
    let (tx_game, rx_game) = mpsc::channel();
    let (tx_falling, rx_falling) = mpsc::channel();
    let mut frame = Frame::new();
    let l1 = thread::spawn(move || loop {
        if frame.block == None {
            frame.generate_block();
        }
        if frame.is_game_over() {
            println!("Game Over");
            tx_game.send("Game Over").expect("Failed to send game over");
            stdout().execute(cursor::Show).unwrap();
            return;
        }
        // terminal_setting();
        print_screen(&frame);
        // block_falling(&mut frame);
        match rx_control.try_recv() {
            Ok(key) => match key {
                Keycode::A => frame.set_move(Direction::Left),
                Keycode::D => frame.set_move(Direction::Right),
                Keycode::S => frame.set_move(Direction::Down),
                Keycode::W => frame.set_move(Direction::Up),
                _ => (),
            },
            Err(_) => (),
        };
        match rx_falling.try_recv() {
            Ok(_) => frame.set_move(Direction::Down),
            Err(_) => (),
        };
    });

    let l2 = thread::spawn(move || {
        user_control(tx_control, rx_game);
    });

    let l3 = thread::spawn(move || {
        block_falling(tx_falling);
    });
    l1.join().unwrap();
    l2.join().unwrap();
    l3.join().unwrap();
}

fn block_falling(sx: mpsc::Sender<bool>) {
    // frame.set_move(Direction::Down);
    loop {
        thread::sleep(Duration::from_millis(300));
        sx.send(true).expect("Failed to send falling");
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

// fn terminal_setting() {
//     let mut stdout = stdout();
//     stdout.execute(terminal::EnterAlternateScreen).unwrap();

//     stdout.execute(cursor::MoveTo(0, 0)).unwrap();
//     stdout.flush().unwrap();
// }

fn print_screen(frame: &Frame) {
    let mut stdout = stdout();
    stdout.execute(cursor::Hide).unwrap();
    frame.print_frame(&mut stdout);

    stdout.queue(cursor::RestorePosition).unwrap();
    stdout.flush().unwrap();
    thread::sleep(Duration::from_millis(20));
    clear_screen(&mut stdout);

    fn clear_screen(stdout: &mut Stdout) {
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
            .unwrap();
    }
}
