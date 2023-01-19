use crate::utils::frame;
use piston_window::*;

static FRAME_WIDTH: u32 = 300;
static FRAME_HEIGHT: u32 = 600;
static BLOCK_WIDTH: u32 = 15;

static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub fn draw_ui(frame: &frame::Frame) {
    let mut window: PistonWindow = WindowSettings::new("Hello World!", [FRAME_WIDTH, FRAME_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            let draw_block = |x: u32, y: u32, color: [f32; 4]| {
                rectangle(
                    color,
                    [x as f64, y as f64, BLOCK_WIDTH as f64, BLOCK_WIDTH as f64],
                    context.transform,
                    graphics,
                );
            };
            clear([1.0; 4], graphics);

            for (i, row) in frame.frame.iter().enumerate() {
                for j in 0..12 {
                    if row & (1 << (11 - j)) != 0 {
                        draw_block(j * BLOCK_WIDTH, i as u32 * BLOCK_WIDTH, WHITE);
                    } else {
                        draw_block(j * BLOCK_WIDTH, i as u32 * BLOCK_WIDTH, BLACK);
                    }
                }
            }
        });
    }
}
