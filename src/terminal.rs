use std::{io::stdout, io::Write};

use termion::{
    raw::IntoRawMode,
    screen::{IntoAlternateScreen, ToAlternateScreen, ToMainScreen},
};

pub struct Terminal {
    screen: termion::screen::AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>,
}

impl Terminal {
    pub fn new() -> Self {
        let screen = stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap();

        Self { screen }
    }
    pub fn leave_alternate_screen(&mut self) {
        write!(self.screen, "{}", ToMainScreen).unwrap();
    }
    pub fn enter_alternate_screen(&mut self) {
        write!(self.screen, "{}", ToAlternateScreen).unwrap();
    }
}
