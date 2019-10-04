use std::fmt::{Display, Formatter};
extern crate pancurses;

enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right,
}

struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

struct Frame {
    width: u32,
    height: u32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new(window: &pancurses::Window) -> Result<Game, String> {
        let (max_y, max_x) = window.get_max_yx();

        if max_y < 10 || max_x < 10 {
            return Err(String::from("Window is too small, exiting"));
        }

        let frame = Frame {
            width: max_x as u32 - 2,
            height: max_y as u32 - 2,
        };

        let ball = Ball {
            x: 2,
            y: 4,
            vert_dir: VertDir::Up,
            horiz_dir: HorizDir::Left,
        };
        Ok(Game {frame, ball})
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 0 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x == frame.width - 1 {
            self.horiz_dir = HorizDir::Left;
        }

        if self.y == 0 {
            self.vert_dir = VertDir::Down;
        } else if self.y == frame.height - 1 {
            self.vert_dir = VertDir::Up;
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }
        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let top_bottom = |fmt: &mut Formatter| {
            write!(fmt, "+")?;
            for _ in 0..self.frame.width {
                write!(fmt, "-")?;
            }
            write!(fmt, "+\n")
        };

        top_bottom(fmt)?;

        for row in 0..self.frame.height {
            write!(fmt, "|")?;

            for column in 0..self.frame.width {
                let c = if row == self.ball.y && column == self.ball.x {
                    'o'
                } else {
                    ' '
                };
                write!(fmt, "{}", c)?;
            }

            write!(fmt, "|\n")?;
        }

        top_bottom(fmt)
    }
}

fn main () {
    let window = pancurses::initscr();

    let mut game = Game::new(&window)?;
    let sleep_duration = std::time::Duration::from_millis(33);
    loop {
        window.clear(); // get rid of old content
        //window.printw(game.to_string());    // write to the buffer
        window.border(
            '|', // left side
            '|', // right side
            '-', // top side
            '-', // bottom side
            '+', // top left corner
            '+', // top right corner
            '+', // bottom left corner
            '+');// bottom  right corner
        window.refresh();   // update the screen
        match window.getch() {
            Some(pancurses::Input::Character('q')) => {
                pancurses::endwin();
                println!("Thanks for playing!");
                return Ok(());
            }

            Some(pancurses::Input::KeyResize) => {
                game = Game::new(&window);
            }

            _ => {
                game.step();
            }
        }

        std::thread::sleep(sleep_duration);
    }
}
