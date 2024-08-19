use yew::prelude::*;
pub struct Cell {
    value: String,
}

pub struct Spreadsheet {
    cells: Vec<Vec<Option<Cell>>>
}

impl Spreadsheet {
    fn new(rows: usize, cols: usize) -> Self {
        let cells = vec![vec![None; cols]; rows];
        Spreadsheet { cells }
    }

    fn set_cell(&mut self, row: usize, col: usize, value: String) {
        let cell = Cell { value };
        self.cells[row][col] = Some(cell);
    }

    fn get_cell(&self, row:usize, col: usize) -> Option<&Cell> {
        self.cells[row][col].as_ref()
    }
}

pub mod cell {
    use yew::Html;
    use super::Cell;
    pub fn createCell() -> Cell {
        Cell {value: "".to_string()}
    }
}