use crate::*;

impl ColumnSegment {
    /// Spawns a new column segment with random parameters.
    pub fn spawn(col_idx: TermCoord, rows: TermCoord, rng: &mut impl Rng) -> ColumnSegment {
        ColumnSegment {
            col_idx,
            spawn_delay: rng.random_range(1..=rows),
            segment_len: rng.random_range(3..rows),
            update_frequency: rng.random_range(1..=3),
        }
    }

    pub fn index(&self) -> usize {
        self.col_idx as usize
    }
}

impl<'a, R: Rng> ColumnProcessor<'a, R> {
    pub fn from_column(
        column: &'a mut ColumnSegment,
        matrix: &'a mut Vec<MatrixCell>,
        rows: usize,
        cols: usize,
        counter: UpdateCounter,
        rng: &'a mut R,
    ) -> Option<Self> {
        if counter < column.update_frequency {
            return None;
        }

        let mut p = ColumnProcessor {
            column,
            matrix,
            rows,
            cols,
            rng,
            first_segment_done: false,
            state: ProcState::Initial,
        };

        p.advance_stream();

        Some(p)
    }

    fn advance_stream(&mut self) {
        let x = self.column.index();
        let fst_is_null = self.matrix[x] == MatrixCell::Null;
        let snd_is_space = self.matrix[self.cols + x] == MatrixCell::Space;

        if fst_is_null && snd_is_space && self.column.spawn_delay > 0 {
            self.column.spawn_delay -= 1;
        } else if fst_is_null && snd_is_space {
            // start new stream
            self.matrix[x] = MatrixCell::Symbol(rand_char(self.rng));

            *self.column =
                ColumnSegment::spawn(self.column.col_idx, self.rows as TermCoord, self.rng);
        }
    }

    fn skip_whitespaces(&mut self, i: &mut usize) {
        while *i < self.rows {
            match self.matrix[self.cols * *i + self.column.index()] {
                MatrixCell::Null | MatrixCell::Space => *i += 1,
                _ => break
            }
        }
    }
}

impl<'a, R: Rng> Iterator for ColumnProcessor<'a, R> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.column.index();

        match self.state {
            ProcState::Initial => {
                self.state = ProcState::Skipping(0);
                Some(())
            }

            ProcState::Skipping(mut i) => {
                // Skipping spaces until the next continuous segment
                self.skip_whitespaces(&mut i);

                if i < self.rows {
                    self.state = ProcState::InSegment {
                        i,
                        segment_start: i,
                        segment_len: 0
                    };
                    Some(())
                } else {
                    None
                }
            }


            ProcState::InSegment {
                i,
                segment_start,
                segment_len,
            } => {
                if i < self.rows {
                    let idx = self.cols * i + x;

                    match self.matrix[idx] {
                        MatrixCell::Null | MatrixCell::Space => {
                            self.state = ProcState::EndOfSegment {
                                i,
                                segment_start,
                                segment_len,
                            };
                            return Some(());
                        }

                        MatrixCell::Head(ch) => {
                            // demote the head to a normal symbol
                            self.matrix[idx] = MatrixCell::Symbol(ch);
                        }

                        MatrixCell::Symbol(_) => {}
                    }

                    self.state = ProcState::InSegment {
                        i: i + 1,
                        segment_start,
                        segment_len: segment_len + 1,
                    };
                    Some(())
                } else {
                    self.matrix[segment_start * self.cols + x] = MatrixCell::Space;
                    None
                }
            }

            ProcState::EndOfSegment {
                i,
                segment_start,
                segment_len,
            } => {
                // Place a new head one cell below the segment
                let head_idx = i * self.cols + x;
                self.matrix[head_idx] = MatrixCell::Head(rand_char(self.rng));

                if segment_len > self.column.segment_len as usize || self.first_segment_done {
                    self.matrix[segment_start * self.cols + x] = MatrixCell::Space;
                    self.matrix[x] = MatrixCell::Null;
                }

                self.first_segment_done = true;
                self.state = ProcState::Skipping(i + 1);
                Some(())
            }
        }
    }
}
