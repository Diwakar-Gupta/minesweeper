use rand::Rng;
use std::{
    io::{stdin, stdout},
    vec,
};
use termion::{color, event::Key, input::TermRead, raw::IntoRawMode, style};

#[derive(Clone, PartialEq)]
enum Cell {
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
            Cell::Num(i) => format!("{}{}{}", style::Bold, i, style::Reset),
        }
    }
}

const ROW_OFFSET: i8 = -1;
const COL_OFFSET: i8 = -1;

struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(PartialEq)]
enum GameStatus {
    Progress,
    Won,
    Lost,
}

pub struct Game {
    size: u8,
    mines: Vec<Vec<i8>>,
    board: Vec<Vec<Cell>>,
    curr_pos: Position,
    count_hidden_cells: usize,
    mine_count: usize,
    flag_count: usize,
    status: GameStatus,
}

impl Game {
    pub fn new(size: u8) -> Self {
        let required_mines = size;
        Self {
            size,
            flag_count: 0,
            mine_count: required_mines as usize,
            mines: Self::generate_mines(size, required_mines),
            board: vec![vec![Cell::Hidden; size.into()]; size.into()],
            curr_pos: Position { row: 1, col: 1 },
            count_hidden_cells: (size * size) as usize,
            status: GameStatus::Progress,
        }
    }

    fn generate_mines(size: u8, required_mines: u8) -> Vec<Vec<i8>> {
        let mut mines = vec![vec![0; size.into()]; size.into()];
        let mut count = 0;

        while count < required_mines {
            let row: usize = rand::thread_rng().gen_range(0..size).into();
            let col: usize = rand::thread_rng().gen_range(0..size).into();

            if mines[row][col] != -1 {
                count += 1;
                mines[row][col] = -1;
            }
        }

        for row in 0..(size as usize) {
            for col in 0..(size as usize) {
                if row == 0 && col == 0 {
                } else if row == 0 {
                    let left = (row, col - 1);
                    if mines[row][col] == -1 && mines[left.0][left.1] != -1 {
                        mines[left.0][left.1] += 1;
                    }

                    if mines[row][col] == 0 && mines[left.0][left.1] == -1 {
                        mines[row][col] += 1;
                    }
                } else if col == 0 {
                    let above = (row - 1, col);
                    if mines[row][col] == -1 && mines[above.0][above.1] != -1 {
                        mines[above.0][above.1] += 1;
                    }
                    if mines[row][col] == 0 && mines[above.0][above.1] == -1 {
                        mines[row][col] += 1;
                    }
                } else {
                    let above = (row - 1, col);
                    let left = (row, col - 1);

                    if mines[row][col] == -1 {
                        if mines[above.0][above.1] != -1 {
                            mines[above.0][above.1] += 1;
                        }
                        if mines[left.0][left.1] != -1 {
                            mines[left.0][left.1] += 1;
                        }
                    } else {
                        if mines[above.0][above.1] == -1 {
                            mines[row][col] += 1;
                        }
                        if mines[left.0][left.1] == -1 {
                            mines[row][col] += 1;
                        }
                    }
                }
            }
        }

        mines
    }

    pub fn run(&mut self) {
        self.set_cursor_pos(1, 1);
        let _stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();

        self.update();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    break;
                }
                Key::Up => {
                    if self.curr_pos.row > 1 {
                        self.curr_pos.row = self.curr_pos.row.saturating_sub(1);
                    }
                    self.set_cursor_pos(self.curr_pos.row, self.curr_pos.col);
                }
                Key::Down => {
                    if self.curr_pos.row < self.size.into() {
                        self.curr_pos.row = self.curr_pos.row.saturating_add(1);
                    }
                    self.set_cursor_pos(self.curr_pos.row, self.curr_pos.col);
                }
                Key::Left => {
                    if self.curr_pos.col > 1 {
                        self.curr_pos.col = self.curr_pos.col.saturating_sub(1);
                    }
                    self.set_cursor_pos(self.curr_pos.row, self.curr_pos.col);
                }
                Key::Right => {
                    if self.curr_pos.col < self.size.into() {
                        self.curr_pos.col = self.curr_pos.col.saturating_add(1);
                    }
                    self.set_cursor_pos(self.curr_pos.row, self.curr_pos.col);
                }
                Key::Char('\n') => {
                    if self.status == GameStatus::Progress {
                        let pos = self.get_curr_cell_pos();
                        self.reveal_cell(pos.row, pos.col);
                    }
                }
                Key::Char(_) => {
                    if self.status == GameStatus::Progress {
                        let pos = self.get_curr_cell_pos();
                        self.mark_flag(pos.row, pos.col);
                    }
                }
                _ => {}
            }
            if self.count_hidden_cells == 0 && self.status == GameStatus::Progress {
                self.status = GameStatus::Won;
            }
            self.update();
        }
    }

    fn get_curr_cell_pos(&self) -> Position {
        let row: usize = ((self.curr_pos.row as i8) + ROW_OFFSET) as usize;
        let col: usize = ((self.curr_pos.col as i8) + COL_OFFSET) as usize;
        Position { row, col }
    }

    fn update(&mut self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        self.render();
        self.set_cursor_pos(self.curr_pos.row, self.curr_pos.col);
    }

    fn render(&mut self) {
        let mut frame = String::new();

        let header = match self.status {
            GameStatus::Progress => format!("{}/{} {}", self.flag_count, self.mine_count, self.count_hidden_cells),
            GameStatus::Won => format!("Won"),
            GameStatus::Lost => format!("Lost"),
        };

        frame.push_str(format!("{header}\n\r").as_str());

        for v in self.board.iter() {
            for c in v.iter() {
                frame.push_str(c.get_view().as_str());
            }
            frame.push_str("\n\r");
        }
        println!("{}", frame);
    }

    fn set_cursor_pos(&mut self, row: usize, col: usize) {
        println!("{}", termion::cursor::Goto(col as u16, row as u16));
    }

    fn can_flag(&self) -> bool {
        return self.flag_count < self.mine_count;
    }

    fn mark_flag(&mut self, row: usize, col: usize) {
        if !self.can_flag() {
            return;
        }
        match self.board[row][col]{
            Cell::Hidden => {
                self.count_hidden_cells -= 1;
                self.flag_count += 1;
                self.board[row][col] = Cell::Marked;
            },
            Cell::Marked => {
                self.flag_count -= 1;
                self.board[row][col] = Cell::Hidden;
                self.count_hidden_cells += 1;
            },
            Cell::Mine => todo!(),
            Cell::Num(_) => {},
        }
    }

    fn reveal_cell(&mut self, row: usize, col: usize) {
        if self.status != GameStatus::Progress{
            return;
        }
        match self.board[row][col]{
            Cell::Hidden => {
                self.count_hidden_cells -= 1;
                if self.mines[row][col] == -1 {
                    self.game_lost();
                } else {
                    flood_fill(self.mines.as_ref(), &mut self.board, row as i32, col as i32);
                }
            },
            Cell::Marked => {
                self.flag_count -= 1;
                self.board[row][col] = Cell::Hidden;
                if self.mines[row][col] == -1 {
                    self.game_lost();
                } else {
                    flood_fill(self.mines.as_ref(), &mut self.board, row as i32, col as i32);
                }
            },
            Cell::Mine => todo!(),
            Cell::Num(_) => {
                return;
            },
        }
    }

    fn game_lost(&mut self) {
        self.status = GameStatus::Lost;

        for row in 0..(self.size as usize) {
            for col in 0..(self.size as usize) {
                if self.mines[row][col] == -1 {
                    self.board[row][col] = Cell::Mine;
                }
            }
        }
    }
}

const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn flood_fill(mines: &Vec<Vec<i8>>, board: &mut [Vec<Cell>], row: i32, col: i32) {
    if mines[row as usize][col as usize] == -1 {
        return;
    }

    board[row as usize][col as usize] = Cell::Num(mines[row as usize][col as usize] as u8);

    if mines[row as usize][col as usize] != 0 {
        return;
    }

    for (i, j) in DIR {
        let r = row + i;
        let c = col + j;

        if !(r < 0 || c < 0 || r >= (mines.len() as i32) || c >= (mines.len() as i32))
            && mines[r as usize][c as usize] == 0
            && board[r as usize][c as usize] == Cell::Hidden
        {
            flood_fill(mines, board, r, c);
        }
    }
}
