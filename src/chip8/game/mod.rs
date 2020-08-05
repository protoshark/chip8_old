use super::Chip8;

pub struct Game {
    pub ch8: Chip8,
    width: usize,
    height: usize,
    window: *mut i8,
}

impl Game {
    pub fn new() -> Game {
        Game {
            ch8: super::Chip8::new(),
            width: super::DISPLAY_WIDTH,
            height: super::DISPLAY_HEIGHT,
            window: &mut 0,
        }
    }

    pub fn load_game(&mut self, bin: Vec<u8>) {
        //load games to offset 0x200
        self.ch8.load_binary(bin, 0x200);
    }

    pub fn run(&mut self) {
        // setup ncurses
        ncurses::initscr();
        ncurses::noecho();
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        self.window = ncurses::newwin(self.height as i32, self.width as i32, 0, 0);
        ncurses::keypad(self.window, true);

        self.window = Some(window);

        ncurses::refresh();
        ncurses::timeout(50);

        self.draw();

        loop {
            let word = self.ch8.fetch();
            self.ch8.execute(word);

            self.read_key();
            if self.ch8.drew {
                self.draw();
            }

            // std::thread::sleep(std::time::Duration::from_millis(5));
        }

        // delwin(window);
        // endwin();
    }

    fn draw(&self) {
        ncurses::werase(self.window);

        for x in 0..self.width {
            for y in 0..self.height {
                let offset = x + y * self.width;
                ncurses::mvwaddch(
                    self.window,
                    y as i32,
                    x as i32,
                    if self.ch8.vram[offset] == 1 {
                        ncurses::ACS_BLOCK()
                    } else {
                        ' ' as u32
                    },
                );
            }
        }
        ncurses::wrefresh(self.window);
    }
}
