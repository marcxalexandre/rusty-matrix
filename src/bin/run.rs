use crossterm::style::Color;
use crossterm::{ExecutableCommand, cursor, event::KeyCode, execute, terminal};
use std::time::Duration;
use rusty_matrix::*;
use clap::Parser;
use humantime;

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> std::io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(std::io::stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;
        Ok(TerminalGuard)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        // restore previous terminal state
        let _ = terminal::disable_raw_mode();
        let _ = execute!(std::io::stdout(), terminal::LeaveAlternateScreen, cursor::Show);
    }
}

fn parse_color(s: &str) -> Result<Color, &'static str> {
    match s.to_lowercase().as_str() {
        "red" | "r" => Ok(Color::Red),
        "green" | "g" => Ok(Color::Green),
        "blue" | "b" => Ok(Color::Blue),
        "magenta" | "m" => Ok(Color::Magenta),
        "white" | "w" => Ok(Color::White),
        "yellow" | "y" => Ok(Color::Yellow),
        "cyan" | "c" => Ok(Color::Cyan),
        _ => Err("Unknown Color scheme")
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Config {
    #[arg(short, long, default_value = "green", value_parser = parse_color)]
    color: Color,

    #[arg(short, long, default_value = "50ms", value_parser = humantime::parse_duration)]
    delay: Duration
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Config::parse();
    let mut stdout = std::io::stdout();
    let mut rng = rand::rng();
    let (mut cols, mut rows) = terminal::size()?;

    // init matrix & column streams
    let (mut matrix, mut columns) = setup_terminal_matrix(rows, cols, &mut rng)?;
    let mut counter = 0;

    // init terminal
    let _guard = TerminalGuard::enter()?;

    // init event handler
    let event_rx = event::init_event_handler(Duration::from_millis(100));

    loop {
        counter = (counter + 1) % 5;

        if let Some(event) = event_rx.try_recv().ok() {
            match event {
                event::EventKind::Key(KeyCode::Char(c)) => match c {
                    'q' => break,
                    d if '0' <= d && d <= '9' => {
                        args.delay = Duration::from_micros(1 + 10_000 * d.to_digit(10).unwrap() as u64)
                    },
                    _ => {
                        if let Ok(color) = parse_color(c.to_string().as_str()) {
                            args.color = color
                        }
                    }
                },

                event::EventKind::Resize(c, r) => {
                    cols = c;
                    rows = r;
                    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
                    (matrix, columns) = setup_terminal_matrix(rows, cols, &mut rng)?
                }

                event::EventKind::Shutdown => break,
                _ => {}
            }
        }

        // update columns
        for state in columns.iter_mut() {
            if let Some(proc) = ColumnProcessor::from_column(
                state, &mut matrix, rows as usize, cols as usize, counter, &mut rng,
            ) {
                for _ in proc {}
            }
        }

        // render canvas
        renderer::render_frame(&matrix, rows, cols, &mut stdout, args.color)?;

        std::thread::sleep(args.delay);
    }

    Ok(())
}
