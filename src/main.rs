mod screen;

fn main() {
    let mut code_screen = screen::Screen::new(80, 24);
    screen::Screen::start_render(code_screen);
}
