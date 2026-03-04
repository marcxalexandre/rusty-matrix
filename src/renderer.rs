use crate::*;
use crossterm::style::{Color, Stylize};
use crossterm::{cursor, style, QueueableCommand};
use std::io::{Stdout, Write};

pub fn render_frame(
    matrix: &Vec<MatrixCell>,
    rows: TermCoord,
    cols: TermCoord,
    stdout: &mut Stdout,
    tail_color: Color,
) -> std::io::Result<()> {
    for row in 0..rows {
        stdout.queue(cursor::MoveTo(0, row))?;

        for col in 0..cols {
            match matrix[cols as usize * row as usize + col as usize] {
                MatrixCell::Space | MatrixCell::Null => stdout.queue(style::Print(' '))?,

                MatrixCell::Symbol(c) => {
                    stdout.queue(style::PrintStyledContent(c.with(tail_color)))?
                }

                MatrixCell::Head(c) => {
                    stdout.queue(style::PrintStyledContent(c.with(Color::White)))?
                }
            };
        }
    }

    stdout.flush()?;

    Ok(())
}
