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

    fn read_key(&mut self) {
        let key_code = ncurses::getch();
        static KEY_TABLE: [i32; 16] = [
            '1' as i32, '2' as i32, '3' as i32, '4' as i32,
            'q' as i32, 'w' as i32, 'e' as i32, 'r' as i32,
            'a' as i32, 's' as i32, 'd' as i32, 'f' as i32,
            'z' as i32, 'x' as i32, 'c' as i32, 'v' as i32,
        ];

        let key = KEY_TABLE.iter().find(|&&key| key == key_code);
        match key {
            Some(&code) => {
                self.ch8.key = (code & 0xFF) as i8;
            }
            None => {}
        };
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
