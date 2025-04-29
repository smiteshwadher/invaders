use crate::{NUMBER_OF_COLS, NUMBER_OF_ROWS};

pub type Frame = Vec<Vec<String>>;

pub fn new_frame() -> Frame {
    let mut cols: Vec<Vec<String>> = Vec::with_capacity(NUMBER_OF_COLS);
    for _ in 0..NUMBER_OF_COLS {
        let mut col = Vec::with_capacity(NUMBER_OF_ROWS);
        for _ in 0..NUMBER_OF_ROWS {
            col.push(" ".to_string());
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}