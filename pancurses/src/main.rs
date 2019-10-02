extern crate pancurses;


fn main() {
    let window = pancurses::initscr();
    window.printw("Hello Rust");
    window.refresh();
    window.getch();
    pancurses::endwin();
}
