use std::io::{self, Result, Stdout};

use tui::{
    self,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    text::Text,
    widgets::{Block, Borders, Clear, Paragraph},
    Terminal,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::utils::frame;

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

pub fn end_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    terminal.show_cursor()?;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.backend_mut().flush()
}

pub fn draw_ui(
    frame: &frame::Frame,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    level: u32,
) -> Result<Vec<Rect>> {
    let mut result: Vec<Rect> = Vec::new();
    terminal.draw(|f| {
        let size = f.size();

        // all chunks
        let all_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
            .split(size);
        result = all_chunks.clone();

        // upper chunk
        f.render_widget(Block::default(), all_chunks[0]);
        let upper_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ]
                .as_ref(),
            )
            .split(all_chunks[0]);

        // upper left score
        f.render_widget(
            Block::default().title("Score").borders(Borders::ALL),
            upper_chunk[0],
        );
        let score_chunk = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .margin(1)
            .split(upper_chunk[0]);
        f.render_widget(
            Paragraph::new(Text::raw(format!("{}", frame.score))),
            score_chunk[0],
        );

        // upper middle level
        f.render_widget(
            Block::default().title("Level").borders(Borders::ALL),
            upper_chunk[1],
        );
        let level_chunk = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(1)
            .split(upper_chunk[1]);

        // f.render_widget(Clear, level_chunk[0]);

        f.render_widget(
            Paragraph::new(Text::raw(format!("{}", level))),
            level_chunk[0],
        );

        // upper right next block
        f.render_widget(
            Block::default().title("Next Block").borders(Borders::ALL),
            upper_chunk[2],
        );
        let next_block_chunk = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .vertical_margin(1)
            .horizontal_margin(2)
            .split(upper_chunk[2]);

        f.render_widget(
            Paragraph::new(Text::raw(frame.print_next_block())),
            next_block_chunk[0],
        );

        // lower chunk
        f.render_widget(Block::default().borders(Borders::ALL), all_chunks[1]);

        // frame part
        let frame_chunk = Layout::default()
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(67),
                    Constraint::Percentage(1),
                ]
                .as_ref(),
            )
            .vertical_margin(5)
            .direction(Direction::Horizontal)
            .split(all_chunks[1]);

        let result = frame.print_frame();
        // f.render_widget(Clear, frame_chunk[0]);
        f.render_widget(Paragraph::new(Text::raw(result)), frame_chunk[1]);
    })?;
    Ok(result)
}

// fn draw_frame(frame: &frame::Frame, f: &mut tui::Frame<CrosstermBackend<Stdout>>, area: Rect) {
//     let row_chunk = Layout::default()
//         .constraints([Constraint::Length(2); 22].as_ref())
//         .split(area);
//     let
//     for row in &frame.frame {
//         for i in 0..12 {
//             if row & (1 << (11 - i)) != 0 {
//             } else {
//             }
//         }
//     }
// }

// fn draw_
