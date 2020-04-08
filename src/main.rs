mod window;

fn main() {
    println!("Hello, world!");
    let w = window::Window::new(1086, 728);
    w.main_loop()
}
