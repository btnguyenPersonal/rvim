pub struct Screen {
    data: Vec<Vec<char>>
}

impl Screen {
    pub fn new(c: usize, r: usize) -> Self {
        Screen {
            data: vec![vec![' '; c]; r]
        }
    }
    pub fn display(&mut self) {
        for r in &self.data {
            for c in r {
                print!("{}", c.to_string());
            }
            println!();
        }
    }
    pub fn change(&mut self, c: usize, r: usize, newval: char) {
        self.data[r][c] = newval;
    }
}
