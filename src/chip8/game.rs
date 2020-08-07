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
        self.ch8.load_binary(bin, super::cpu::GAME_MEM_OFFSET);
    }

    pub fn run(&mut self) {
        // setup ncurses
        ncurses::initscr();
        ncurses::noecho();
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        let mut maxyx = (0, 0);
        ncurses::getmaxyx(ncurses::stdscr(), &mut maxyx.0, &mut maxyx.1);

        let centery = ((maxyx.0 as usize).saturating_sub(self.height)) / 2;
        let centerx = ((maxyx.1 as usize).saturating_sub(self.width)) / 2;

        self.window = ncurses::newwin(
            self.height as i32,
            self.width as i32,
            centery as i32,
            centerx as i32,
        );
        ncurses::keypad(self.window, true);

        ncurses::refresh();
        ncurses::timeout(3);

        self.draw();

        let mut last_timer = std::time::SystemTime::now();

        loop {
            let word = self.ch8.fetch();
            self.ch8.execute(word);

            if self.ch8.drew {
                self.draw();
            }

            // beep while sound timer is greater than 0
            if self.ch8.cpu.sound_timer > 0 {
                self.ch8.cpu.sound_timer -= 1;
                ncurses::beep();
            }


            let now = std::time::SystemTime::now();
            if now > last_timer + std::time::Duration::from_secs_f64(1. / 60.) {
                // self.ch8.key = -1;
                let delay = self.ch8.cpu.delay_timer;
                self.ch8.cpu.delay_timer = delay.saturating_sub(1);
                last_timer = std::time::SystemTime::now();
            }
        }

        // delwin(window);
        // endwin();
    }

    fn read_key(&mut self) {
        let key_code = ncurses::getch();

        self.ch8.key = match key_code as u16 as u8 as char {
            '1' => 0x0,
            '2' => 0x1,
            '3' => 0x2,
            '4' => 0x3,
            'q' => 0x4,
            'w' => 0x5,
            'e' => 0x6,
            'r' => 0x7,
            'a' => 0x8,
            's' => 0x9,
            'd' => 0xA,
            'f' => 0xB,
            'z' => 0xC,
            'x' => 0xD,
            'c' => 0xE,
            'v' => 0xF,
            _ => self.ch8.key,
        }
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
