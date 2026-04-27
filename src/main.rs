#![allow(unused)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::collapsible_if)]
use crossterm::cursor;
use crossterm::event::KeyEventKind;
use crossterm::execute;
use crossterm::queue;
use crossterm::style;
use crossterm::terminal;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode, poll, read},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};

use std::convert::TryInto;
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

static BOARDHEIGHT: u16 = 16;
static BOARDWIDTH: u16 = 10;
static BOARDX: u16 = 25;
static BOARDY: u16 = 2;

static CHARLEN: u16 = 2;

#[derive(Clone, Copy)]
struct Tetromino {
    shape: [[bool; 4]; 4],
    color: Color,
}

struct Temp {
    shape: Vec<Vec<bool>>,
}

impl Tetromino {
    // Helper function to map 1s to true and 0s to false
    fn from_matrix(matrix: [[u8; 4]; 4], color: Color) -> Self {
        let mut shape = [[false; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                shape[y][x] = matrix[y][x] == 1;
            }
        }
        Tetromino { shape, color }
    }

    pub fn new_i() -> Self {
        Self::from_matrix(
            [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
            Color::Cyan,
        )
    }
    pub fn new_j() -> Self {
        Self::from_matrix(
            [[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            Color::Blue,
        )
    }
    pub fn new_t() -> Self {
        Self::from_matrix(
            [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            Color::Blue,
        ) // purple
    }
    pub fn new_o() -> Self {
        Self::from_matrix(
            [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            Color::Yellow,
        )
    }
    pub fn new_l() -> Self {
        Self::from_matrix(
            [[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            Color::Yellow,
        ) // Orange
    }
    pub fn new_s() -> Self {
        Self::from_matrix(
            [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            Color::Green,
        )
    }
    pub fn new_z() -> Self {
        Self::from_matrix(
            [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            Color::Red,
        )
    }
}

#[derive(Clone, Copy)]
struct MovingTetromino {
    tetromino: Tetromino,
    coords: (u16, u16),
}

fn cleanup() {
    execute!(
        stdout(),
        cursor::Show,
        terminal::LeaveAlternateScreen,
        style::ResetColor
    )
    .unwrap();
    disable_raw_mode().unwrap();
    std::process::exit(0);
}

fn start_menu(points: Vec<i32>) -> i16 {
    execute!(stdout(), cursor::MoveTo(0, 5));

    println!(
        " ███████████ ██████████ ███████████ ███████████   █████  █████████ 
▒█▒▒▒███▒▒▒█▒▒███▒▒▒▒▒█▒█▒▒▒███▒▒▒█▒▒███▒▒▒▒▒███ ▒▒███  ███▒▒▒▒▒███
▒   ▒███  ▒  ▒███  █ ▒ ▒   ▒███  ▒  ▒███    ▒███  ▒███ ▒███    ▒▒▒ 
    ▒███     ▒██████       ▒███     ▒██████████   ▒███ ▒▒█████████ 
    ▒███     ▒███▒▒█       ▒███     ▒███▒▒▒▒▒███  ▒███  ▒▒▒▒▒▒▒▒███
    ▒███     ▒███ ▒   █    ▒███     ▒███    ▒███  ▒███  ███    ▒███
    █████    ██████████    █████    █████   █████ █████▒▒█████████ 
   ▒▒▒▒▒    ▒▒▒▒▒▒▒▒▒▒    ▒▒▒▒▒    ▒▒▒▒▒   ▒▒▒▒▒ ▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒  "
    );

    loop {
        if poll(Duration::from_millis(50)).unwrap() {
            if let Event::Key(key_event) = read().unwrap() {
                match key_event.kind {
                    KeyEventKind::Press => match key_event.code {
                        KeyCode::Char('a') | KeyCode::Char('h') | KeyCode::Left => {
                            println!("wo");
                        }
                        KeyCode::Char('d') | KeyCode::Char('l') | KeyCode::Right => {
                            println!("w");
                        }
                        _ => {}
                    },
                    KeyEventKind::Release => {}
                    _ => {}
                }
            }
        }
    }

    32
}

fn main() {
    // this lets ut get raw input from the command line
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    stdout.execute(Hide).unwrap();
    stdout.execute(Clear(ClearType::All)).unwrap();

    queue!(
        stdout,
        cursor::Hide,
        terminal::EnterAlternateScreen,
        terminal::SetSize(48, 36)
    );

    let mut points: Vec<i32> = vec![10, 12, 100];
    start_menu(points);

    let mut board: [[bool; BOARDWIDTH as usize]; BOARDHEIGHT as usize] =
        [[false; BOARDWIDTH as usize]; BOARDHEIGHT as usize];

    let tetrominos = [
        Tetromino::new_i(),
        Tetromino::new_j(),
        Tetromino::new_t(),
        Tetromino::new_t(),
        Tetromino::new_l(),
        Tetromino::new_o(),
        Tetromino::new_s(),
        Tetromino::new_z(),
    ];

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let random_int = (time % 7) as usize;

    let mut current_piece = MovingTetromino {
        tetromino: tetrominos[random_int],
        coords: (27, 2),
    };

    let mut ticks = 1;
    let mut ticks_on_ground = 1;
    let mut debug_mode: bool = false;

    loop {
        if debug_mode {
            queue!(
                stdout,
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveTo(1, 1),
                style::Print(format!("current_piece X: {}", current_piece.coords.0)),
                cursor::MoveTo(1, 2),
                style::Print(format!("current_piece Y: {}", current_piece.coords.1)),
            );
            stdout.flush();
        }
        // this is to draw the board
        for by in BOARDY..(BOARDHEIGHT + BOARDY) {
            queue!(stdout, cursor::MoveTo(BOARDX, by));
            stdout.queue(Print("|")).unwrap();

            let temp_board = place_piece_in_board(board, current_piece);

            for bx in BOARDX..(BOARDWIDTH + BOARDX) {
                if temp_board[(by - BOARDY) as usize][(bx - BOARDX) as usize] {
                    stdout.queue(Print("[]"));
                } else if board[(by - BOARDY) as usize][(bx - BOARDX) as usize] {
                    stdout.queue(Print("[]"));
                } else {
                    stdout.queue(Print(" ."));
                }
            }
            stdout.queue(Print("|"));
        }
        stdout.flush();

        if poll(Duration::from_millis(50)).unwrap() {
            if let Event::Key(key_event) = read().unwrap() {
                match key_event.kind {
                    KeyEventKind::Press => {
                        match key_event.code {
                            KeyCode::Char('a') | KeyCode::Char('h') | KeyCode::Left => {
                                if check_valid_board(board, current_piece, -1, 0) {
                                    current_piece.coords.0 -= 1;
                                }
                            }
                            KeyCode::Char('d') | KeyCode::Char('l') | KeyCode::Right => {
                                if check_valid_board(board, current_piece, 1, 0) {
                                    current_piece.coords.0 += 1;
                                }
                            }
                            KeyCode::Char('s') | KeyCode::Char('j') | KeyCode::Down => {
                                if check_valid_board(board, current_piece, 0, 1) {
                                    current_piece.coords.1 += 1;
                                }
                            }
                            KeyCode::Char('w') | KeyCode::Char('k') | KeyCode::Up => {
                                if check_valid_board(board, rotate_right(current_piece), 0, 0) {
                                    current_piece = rotate_right(current_piece);
                                }
                            }
                            KeyCode::Char('r') | KeyCode::Char('R') => {
                                //reset("TODO")
                            }

                            KeyCode::End => {
                                board = place_piece_in_board(board, current_piece);
                                debug_mode = !debug_mode;
                                if !debug_mode {
                                    queue!(stdout, terminal::Clear(terminal::ClearType::All));
                                }
                            }
                            KeyCode::Char(' ') => {
                                ticks_on_ground = 20;
                                while check_valid_board(board, current_piece, 0, 1) {
                                    current_piece.coords.1 += 1
                                }
                            }
                            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => cleanup(),
                            _ => {}
                        }
                    }
                    KeyEventKind::Release => {}
                    _ => {}
                }
            }
        }

        ticks += 1;

        if ticks >= 10 {
            ticks = 0;
            if check_valid_board(board, current_piece, 0, 1) {
                current_piece.coords.1 += 1;
            }
        }

        if !check_valid_board(board, current_piece, 0, 1) {
            ticks_on_ground += 1;
        }

        if ticks_on_ground >= 20 {
            let mut time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
            let mut random_int = (time % 7) as usize;

            board = place_piece_in_board(board, current_piece);
            current_piece = MovingTetromino {
                tetromino: tetrominos[random_int],
                coords: (27, 2),
            };
            //if !check_valid_board(board,MovingTetromino {tetromino: tetrominos[1], shape: (27,2)},0,0) {
            //    cleanup();
            //}
            ticks_on_ground = 0;
        }
    }
}
fn make_bag() -> [Tetromino; 8] {
    let tetrominos = [
        Tetromino::new_i(),
        Tetromino::new_j(),
        Tetromino::new_t(),
        Tetromino::new_t(),
        Tetromino::new_l(),
        Tetromino::new_o(),
        Tetromino::new_s(),
        Tetromino::new_z(),
    ];

    tetrominos
}

fn place_piece_in_board(board: [[bool; 10]; 16], piece: MovingTetromino) -> [[bool; 10]; 16] {
    let mut tmp_board: [[bool; 10]; 16] = board;
    let piece_x = piece.coords.0;
    let piece_y = piece.coords.1;
    let piece_shape = piece.tetromino.shape;

    for x in piece_x..(piece_x + 4) {
        for y in piece_y..(piece_y + 4) {
            if piece_shape[(y - piece_y) as usize][(x - piece_x) as usize] {
                tmp_board[(y - BOARDY) as usize][(x - BOARDX) as usize] = true;
            }
        }
    }

    tmp_board = clear_lines(tmp_board);

    tmp_board
}

fn clear_lines(board: [[bool; 10]; 16]) -> [[bool; 10]; 16] {
    let mut board_tmp = board;
    let mut score: u16 = 0;
    if board[15].iter().all(|&item| item) {
        let mut board_tmp_tmp = [[false; 10]; 16];
        while board_tmp[15].iter().all(|&item| item) {
            board_tmp[15] = [false; 10];

            for i in 1..16 {
                board_tmp_tmp[i] = board_tmp[(i - 1)];
            }

            board_tmp = board_tmp_tmp
        }
        return board_tmp_tmp;
    }
    board
}

fn check_valid_board(board: [[bool; 10]; 16], piece: MovingTetromino, dx: i16, dy: i16) -> bool {
    let mut tmp_board: [[bool; 10]; 16] = board;
    let piece_x = piece.coords.0 as i16 + dx;
    let piece_y = piece.coords.1 as i16 + dy;

    let piece_shape = piece.tetromino.shape;

    if piece_x < 0 || piece_y < 0 {
        return false;
    } else {
        let upiece_x = piece_x as u16;
        let upiece_y = piece_y as u16;

        for x in upiece_x..(upiece_x + 4) {
            for y in upiece_y..(upiece_y + 4) {
                if piece_shape[(y - upiece_y) as usize][(x - upiece_x) as usize] {
                    if x < BOARDX || x >= BOARDX + BOARDWIDTH {
                        return false;
                    } else if y <= BOARDY || y >= BOARDY + BOARDHEIGHT {
                        return false;
                    }
                }
            }
        }
        let mut tmp_piece = piece;
        tmp_piece.coords = (upiece_x, upiece_y);
        let board_with_placed_piece = place_piece_in_board([[false; 10]; 16], tmp_piece);
        for x in 0..10 {
            for y in 0..16 {
                if board_with_placed_piece[y][x] && board[y][x] {
                    return false;
                }
            }
        }
        return true;
    }
    true
}

fn rotate_right(piece: MovingTetromino) -> MovingTetromino {
    let mut temp_vec_board = Vec::new();

    for i in 0..4 {
        let mut temp_vec_rows = Vec::new();
        for j in (0..4).rev() {
            temp_vec_rows.push(piece.tetromino.shape[j][i]);
        }
        temp_vec_board.push(temp_vec_rows);
    }

    let mut rotated_array = [[false; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            rotated_array[j][i] = temp_vec_board[j][i];
        }
    }

    let mut piece_rotated = piece;
    piece_rotated.tetromino.shape = rotated_array;

    piece_rotated
}
