pub struct Screen {
    row: usize,
    col: usize,
    data: Vec<Vec<char>>
}

impl Screen {
    pub fn new(r: usize, c: usize) -> Self {
        Screen {
            row: r,
            col: c,
            data: vec![vec!['#'; c]; r]
        }
    }
}
