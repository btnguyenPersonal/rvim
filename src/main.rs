mod screen;

fn main() {
    let mut code_screen = screen::Screen::new(80, 24);
    screen::Screen::set(&mut code_screen, 50, 5, 'h');
    println!("{}", screen::Screen::display(&mut code_screen));
    println!("{}", screen::Screen::get(code_screen, 4, 5));
}
