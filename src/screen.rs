pub struct Screen {
    data: Vec<Vec<char>>
}

impl Screen {
    pub fn new(c: usize, r: usize) -> Self {
        Screen {
            data: vec![vec![' '; c]; r]
        }
    }
    pub fn display(&mut self) -> String{
        let mut output = "".to_string();
        for r in &self.data {
            for c in r {
                output += &c.to_string();
            }
            output += "\n";
        }
        output
    }
    pub fn set(&mut self, c: usize, r: usize, newval: char) {
        if c >= self.data[r].len() || r >= self.data.len() {
            panic!("Tried to set element with bigger index than screen length");
        }
        self.data[r][c] = newval;
    }
    pub fn get(self, c: usize, r: usize) -> char {
        if c >= self.data[r].len() || r >= self.data.len() {
            panic!("Tried to get element with bigger index than screen length");
        }
        self.data[r][c]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_0_length_screen() {
        let test_screen = Screen::new(0, 0);
        assert_eq!(test_screen.data.len(), 0);
    }

    #[test]
    fn new_creates_correct_dimensions() {
        let test_screen = Screen::new(5, 4);
        assert_eq!(test_screen.data[0].len(), 5);
        assert_eq!(test_screen.data.len(), 4);
    }

    #[test]
    fn new_creates_80_24_screen() {
        let test_screen = Screen::new(80, 24);
        assert_eq!(test_screen.data[0].len(), 80);
        assert_eq!(test_screen.data.len(), 24);
    }

    #[test]
    fn set_default_case() {
        let mut test_screen = Screen::new(80, 24);
        Screen::set(&mut test_screen, 10, 10, '$');
        assert_eq!(test_screen.data[10][10], '$')
    }

    #[test]
    fn display_default_case() {
        let mut test_screen = Screen::new(80, 24);
        let mut correct_output = "".to_string();
        for _i in 0..24 {
            for _j in 0..80 {
                correct_output += &" ".to_string();
            }
            correct_output += "\n";
        }
        assert_eq!(Screen::display(&mut test_screen), correct_output)
    }

    #[test]
    fn display_0_by_0_case() {
        let mut test_screen = Screen::new(0, 0);
        assert_eq!(Screen::display(&mut test_screen), "")
    }

    #[test]
    fn display_after_set_case() {
        let mut test_screen = Screen::new(4, 4);
        Screen::set(&mut test_screen, 0, 0, '$');
        Screen::set(&mut test_screen, 1, 1, '$');
        Screen::set(&mut test_screen, 2, 2, '$');
        Screen::set(&mut test_screen, 3, 3, '$');
        let correct_output = "$   \n $  \n  $ \n   $\n";
        assert_eq!(Screen::display(&mut test_screen), correct_output)
    }

    #[test]
    #[should_panic]
    fn set_panic_larger_col() {
        let mut test_screen = Screen::new(80, 24);
        Screen::set(&mut test_screen, 10, 24, '$');
    }

    #[test]
    #[should_panic]
    fn set_panic_larger_row() {
        let mut test_screen = Screen::new(80, 24);
        Screen::set(&mut test_screen, 80, 10, '$');
    }

    #[test]
    fn get_returns_correct_default_val() {
        let test_screen = Screen::new(80, 24);
        assert_eq!(Screen::get(test_screen, 10, 10), ' ');
    }

    #[test]
    fn get_returns_correct_val_after_set() {
        let mut test_screen = Screen::new(80, 24);
        Screen::set(&mut test_screen, 10, 10, '*');
        assert_eq!(Screen::get(test_screen, 10, 10), '*');
    }
}
