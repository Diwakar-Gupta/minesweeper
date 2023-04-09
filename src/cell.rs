use termion::{color, style};

#[derive(PartialEq, Clone)]
pub enum Cell {
    Hidden,
    Marked,
    Mine,
    Num(u8),
}

impl Cell {
    pub fn get_view(&self) -> String {
        match self {
            Cell::Hidden => format!("{} {}", style::Bold, style::Reset),
            Cell::Marked => format!("{}.{}", style::Bold, style::Reset),
            Cell::Mine => format!(
                "{}{}*{}{}",
                style::Bold,
                color::Bg(color::Red),
                color::Bg(color::Reset),
                style::Reset
            ),
            Cell::Num(i) => format!(
                "{}{}{}",
                style::Bold,
                Self::get_icon(i.to_owned()),
                style::Reset
            ),
        }
    }
    fn get_icon(i: u8) -> &'static str {
        match i {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            _ => unreachable!(),
        }
    }
}
