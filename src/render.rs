#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Char(char),
    Continuation,
}

pub struct Canvas {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(waidth: usize, height: usize) -> Self {
        Self {
            cells: vec![vec![Cell::Empty; width]; height],
            width,
            height,
        }
    }

    pub fn render(&self) -> String {
        self.cells
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(|c| match c {
                        Cell::Char(ch) => Some(*ch),
                        Cell::Empty => Some(' '),
                        Cell::Continuation => None,
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn print(&self) {
        println!("{}", self.render());
    }
}
