use rand::{Rng, RngExt};

pub mod columns;
pub mod event;
pub mod renderer;

/// Represents a single cell in the Matrix display.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MatrixCell {
    Null,
    Space,
    Head(char),
    Symbol(char),
}

type TermCoord = u16;
type UpdateCounter = u16;

/// Configuration for a single column in the Matrix.
pub struct ColumnSegment {
    col_idx: TermCoord,
    spawn_delay: TermCoord,
    segment_len: TermCoord,
    update_frequency: TermCoord,
}

/// Internal state machine for processing a column's stream.
enum ProcState {
    Initial,
    Skipping(usize),
    InSegment {
        i: usize,
        segment_start: usize,
        segment_len: usize,
    },
    EndOfSegment {
        i: usize,
        segment_start: usize,
        segment_len: usize,
    },
}

/// Processes and advances a single column's stream.
///
/// This struct is responsible for managing the lifecycle of a falling
/// character stream. It implements Iterator to step through
/// each row of the column and update the Matrix state accordingly.
pub struct ColumnProcessor<'a, R: Rng> {
    column: &'a mut ColumnSegment,
    matrix: &'a mut Vec<MatrixCell>,
    rows: usize,
    cols: usize,
    first_segment_done: bool,
    state: ProcState,
    rng: &'a mut R,
}

/// Initializes the Matrix grid and column streams.
pub fn setup_terminal_matrix(
    rows: TermCoord,
    cols: TermCoord,
    rng: &mut impl Rng,
) -> Result<(Vec<MatrixCell>, Vec<ColumnSegment>), String> {
    if rows < 10 || cols < 10 {
        return Err(format!("Terminal too small: {rows} rows x {cols} cols"));
    }

    let mut matrix = vec![MatrixCell::Null; rows as usize * cols as usize];

    // reset streams
    let streams = (0..cols)
        .step_by(2)
        .map(|i| {
            let col = ColumnSegment::spawn(i, rows, rng);
            matrix[cols as usize + i as usize] = MatrixCell::Space;
            col
        })
        .collect();

    Ok((matrix, streams))
}

/// Generates a random printable ASCII character.
pub fn rand_char(rng: &mut impl Rng) -> char {
    const CHARMIN: u32 = '!' as u32;
    const CHARMAX: u32 = '~' as u32;
    char::from_u32(rng.random_range(CHARMIN..=CHARMAX)).unwrap()
}
