use rand::Rng;
use std::{io::stdin, vec};
use termion::{event::Key, input::TermRead};

use crate::{cell::Cell, terminal::Terminal};

const ROW_OFFSET: usize = 2;
const COL_OFFSET: usize = 2;

#[derive(Clone, PartialEq)]

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
    size: usize,
    mines: Vec<Vec<i8>>,
    board: Vec<Vec<Cell>>,
    curr_pos: Position,
    count_hidden_cells: usize,
    mine_count: usize,
    flag_count: usize,
    status: GameStatus,
    terminal: Terminal,
}

impl Game {
    pub fn new(size: usize) -> Self {
        let required_mines = size;
        Self {
            size,
            flag_count: 0,
            mine_count: required_mines,
            mines: Self::generate_mines(size, required_mines),
            board: vec![vec![Cell::Hidden; size]; size],
            curr_pos: Position {
                row: ROW_OFFSET,
                col: COL_OFFSET,
            },
            count_hidden_cells: size * size,
            status: GameStatus::Progress,
            terminal: Terminal::new(),
        }
    }

    fn reset_game(&mut self) {
        let state = Self::new(self.size);

        self.flag_count = state.flag_count;
        self.mine_count = state.mine_count;
        self.mines = state.mines;
        self.board = state.board;
        self.curr_pos = state.curr_pos;
        self.count_hidden_cells = state.count_hidden_cells;
        self.status = state.status;
    }

    fn generate_mines(size: usize, required_mines: usize) -> Vec<Vec<i8>> {
        let mut mines = vec![vec![0; size]; size];
        let mut count = 0;

        while count < required_mines {
            let row: usize = rand::thread_rng().gen_range(0..size);
            let col: usize = rand::thread_rng().gen_range(0..size);

            if mines[row][col] != -1 {
                count += 1;
                mines[row][col] = -1;
            }
        }

        for row in 0..size {
            for col in 0..size {
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
        self.terminal.enter_alternate_screen();
        self.set_cursor_pos(ROW_OFFSET, COL_OFFSET);
        let stdin = stdin();

        self.update();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    break;
                }
                Key::Char('r') => {
                    self.reset_game();
                }
                Key::Char(key) => self.process_char(key),
                Key::Up => {
                    if self.curr_pos.row > ROW_OFFSET {
                        self.curr_pos.row = self.curr_pos.row.saturating_sub(1);
                    }
                    self.set_cursor_pos(self.curr_pos.row, self.curr_pos.col);
                }
                Key::Down => {
                    if self.curr_pos.row < (ROW_OFFSET + self.size - 1) {
                        self.curr_pos.row = self.curr_pos.row.saturating_add(1);
                    }
                    self.set_cursor_pos(self.curr_pos.row, self.curr_pos.col);
                }
                Key::Left => {
                    if self.curr_pos.col > COL_OFFSET {
                        self.curr_pos.col = self.curr_pos.col.saturating_sub(1);
                    }
                    self.set_cursor_pos(self.curr_pos.row, self.curr_pos.col);
                }
                Key::Right => {
                    if self.curr_pos.col < (COL_OFFSET + self.size - 1) {
                        self.curr_pos.col = self.curr_pos.col.saturating_add(1);
                    }
                    self.set_cursor_pos(self.curr_pos.row, self.curr_pos.col);
                }
                _ => {}
            }
            if self.count_hidden_cells == 0 && self.status == GameStatus::Progress {
                self.status = GameStatus::Won;
                self.reveal_mines();
            }
            self.update();
        }
        self.terminal.leave_alternate_screen();
    }

    fn process_char(&mut self, c: char) {
        match c {
            '\n' => {
                if self.status == GameStatus::Progress {
                    let pos = self.get_curr_cell_pos();
                    self.reveal_cell(pos.row, pos.col);
                }
            }
            'f' => {
                if self.status == GameStatus::Progress {
                    let pos = self.get_curr_cell_pos();
                    self.mark_flag(pos.row, pos.col);
                }
            }
            _ => {}
        }
    }

    fn get_curr_cell_pos(&self) -> Position {
        let row: usize = self.curr_pos.row - ROW_OFFSET;
        let col: usize = self.curr_pos.col - COL_OFFSET;
        Position { row, col }
    }

    fn update(&mut self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        self.render();
        self.set_cursor_pos(self.curr_pos.row, self.curr_pos.col);
    }

    fn render(&mut self) {
        let mut frame: Vec<String> = vec![];

        let header = match self.status {
            GameStatus::Progress => format!(
                "{}/{} {}",
                self.flag_count, self.mine_count, self.count_hidden_cells
            ),
            GameStatus::Won => "Won".to_string(),
            GameStatus::Lost => "Lost".to_string(),
        };

        frame.push(header);

        frame.push("_".repeat(self.size + 2));

        for v in self.board.iter() {
            let mut r = String::new();
            r.push('|');
            for c in v.iter() {
                r.push_str(c.get_view().as_str());
            }
            r.push('|');
            frame.push(r);
        }
        frame.push("¯".repeat(self.size + 2));
        // frame.push_str(format!("\n\r({}/{})", self.curr_pos.row, self.curr_pos.col).as_str());
        if self.status == GameStatus::Progress {
            frame.push("Reveal: Enter".to_string());
            frame.push("Flag  : f".to_string());
        } else {
            frame.push("Play Again: r".to_string());
            frame.push("Quit Game : q".to_string());
        }

        println!("{}", frame.join("\r\n"));
    }

    fn set_cursor_pos(&mut self, row: usize, col: usize) {
        println!("{}", termion::cursor::Goto(col as u16, row as u16));
    }

    fn can_flag(&self) -> bool {
        self.flag_count < self.mine_count
    }

    fn mark_flag(&mut self, row: usize, col: usize) {
        if !self.can_flag() {
            return;
        }
        match self.board[row][col] {
            Cell::Hidden => {
                self.count_hidden_cells -= 1;
                self.flag_count += 1;
                self.board[row][col] = Cell::Marked;
            }
            Cell::Marked => {
                self.flag_count -= 1;
                self.board[row][col] = Cell::Hidden;
                self.count_hidden_cells += 1;
            }
            Cell::Mine => todo!(),
            Cell::Num(_) => {}
        }
    }

    fn reveal_cell(&mut self, row: usize, col: usize) {
        if self.status != GameStatus::Progress {
            return;
        }
        match self.board[row][col] {
            Cell::Hidden => {
                if self.mines[row][col] == -1 {
                    self.game_lost();
                } else {
                    self.count_hidden_cells -=
                        flood_fill(self.mines.as_ref(), &mut self.board, row as i32, col as i32);
                }
            }
            Cell::Marked => {
                self.flag_count -= 1;
                self.board[row][col] = Cell::Hidden;
                if self.mines[row][col] == -1 {
                    self.game_lost();
                } else {
                    self.count_hidden_cells += 1;
                    self.count_hidden_cells -=
                        flood_fill(self.mines.as_ref(), &mut self.board, row as i32, col as i32);
                }
            }
            Cell::Mine => todo!(),
            Cell::Num(_) => {}
        }
    }

    fn game_lost(&mut self) {
        self.status = GameStatus::Lost;

        self.reveal_mines();
    }

    fn reveal_mines(&mut self) {
        for row in 0..self.size {
            for col in 0..self.size {
                if self.mines[row][col] == -1 {
                    self.board[row][col] = Cell::Mine;
                }
            }
        }
    }
}

const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn flood_fill(mines: &Vec<Vec<i8>>, board: &mut [Vec<Cell>], row: i32, col: i32) -> usize {
    if mines[row as usize][col as usize] == -1 {
        return 0;
    }

    let mut count = 1;
    board[row as usize][col as usize] = Cell::Num(mines[row as usize][col as usize] as u8);

    if mines[row as usize][col as usize] != 0 {
        return count;
    }

    for (i, j) in DIR {
        let r = row + i;
        let c = col + j;

        if !(r < 0 || c < 0 || r >= (mines.len() as i32) || c >= (mines.len() as i32))
            && mines[r as usize][c as usize] == 0
            && board[r as usize][c as usize] == Cell::Hidden
        {
            count += flood_fill(mines, board, r, c);
        }
    }
    count
}
