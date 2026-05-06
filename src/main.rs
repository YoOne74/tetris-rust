#![allow(unused)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::collapsible_if)]
use crossterm::cursor;
use crossterm::event::KeyEventKind;
use crossterm::execute;
use crossterm::queue;
use crossterm::style;
use crossterm::style::Colors;
use crossterm::style::SetColors;
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

const BOARDHEIGHT: u16 = 20;
const BOARDWIDTH: u16 = 10;
const BOARDX: u16 = 25;
const BOARDY: u16 = 2;
const PIECESTARTX: u16 = BOARDX + (BOARDWIDTH / 2);

const HELDBOARDHEIGHT: u16 = 4;
const HELDBOARDWIDTH: u16 = 6;
const HELDBOARDX: u16 = 15;
const HELDBOARDY: u16 = 3;

#[derive(Clone, Debug)]
struct Tetromino {
    shape: Vec<Vec<bool>>,
    color: Color,
}

impl Tetromino {
    // Helper function to map 1s to true and 0s to false
    fn from_vector(vec: Vec<Vec<u8>>, color: Color) -> Self {
        let height = vec.len();
        let width = vec[0].len();
        let mut shape = vec![vec![false; height]; height];

        for y in 0..height {
            for x in 0..width {
                shape[y][x] = vec[y][x] == 1;
            }
        }
        Tetromino { shape, color }
    }

    pub fn new_t() -> Self {
        Self::from_vector(
            vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 0, 0]],
            Color::Magenta,
        )
    }
    pub fn new_i() -> Self {
        Self::from_vector(
            vec![
                vec![0, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
            Color::Cyan,
        )
    }
    pub fn new_j() -> Self {
        Self::from_vector(
            vec![vec![1, 0, 0], vec![1, 1, 1], vec![0, 0, 0]],
            Color::Blue,
        )
    }
    pub fn new_o() -> Self {
        Self::from_vector(vec![vec![1, 1], vec![1, 1]], Color::Yellow)
    }
    pub fn new_l() -> Self {
        Self::from_vector(
            vec![vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]],
            Color::DarkYellow,
        ) // Orange
    }
    pub fn new_s() -> Self {
        Self::from_vector(
            vec![vec![1, 1, 0], vec![0, 1, 1], vec![0, 0, 0]],
            Color::Green,
        )
    }
    pub fn new_z() -> Self {
        Self::from_vector(
            vec![vec![0, 1, 1], vec![1, 1, 0], vec![0, 0, 0]],
            Color::Red,
        )
    }
}

#[derive(Clone)]
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

    // let mut points: Vec<i32> = vec![10, 12, 100];
    // start_menu(points);

    let mut board: Vec<Vec<Option<Color>>> =
        vec![vec![None; BOARDWIDTH as usize]; BOARDHEIGHT as usize];

    let tetrominos = [
        Tetromino::new_i(),
        Tetromino::new_j(),
        Tetromino::new_l(),
        Tetromino::new_o(),
        Tetromino::new_s(),
        Tetromino::new_t(),
        Tetromino::new_z(),
    ];

    let mut current_piece = MovingTetromino {
        tetromino: tetrominos[random_u32(6) as usize].clone(),
        coords: (PIECESTARTX, 2),
    };

    let mut ticks = 0;
    let mut ticks_on_ground = 0;
    let mut debug_mode: bool = false;

    let mut has_held_piece: bool = false;

    let mut bag = make_bag();

    let mut held_piece: Option<Tetromino> = None;
    let mut held_piece_board: Vec<Vec<Option<Color>>> =
        vec![vec![None; HELDBOARDWIDTH as usize]; HELDBOARDHEIGHT as usize];

    let mut next_pieces_board: Vec<Vec<Option<Color>>> = vec![vec![None; 6]; 20];

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
        let mut saved_y = current_piece.coords.1;
        let mut ghost_board: Vec<Vec<Option<Color>>> =
            vec![vec![None; BOARDWIDTH as usize]; BOARDHEIGHT as usize];

        while check_valid_board(&board, &current_piece, 0, 1) {
            current_piece.coords.1 += 1;
        }
        ghost_board = place_piece_in_board(&ghost_board, &current_piece, BOARDX, BOARDY);
        current_piece.coords.1 = saved_y;

        // this is to draw the board
        for by in BOARDY..(BOARDHEIGHT + BOARDY) {
            queue!(stdout, cursor::MoveTo(BOARDX - 1, by));
            stdout.queue(Print("|")).unwrap();

            let board_with_piece = place_piece_in_board(
                &vec![vec![None; BOARDWIDTH as usize]; BOARDHEIGHT as usize],
                &current_piece,
                BOARDX,
                BOARDY,
            );

            for bx in BOARDX..(BOARDWIDTH + BOARDX) {
                if board_with_piece[(by - BOARDY) as usize][(bx - BOARDX) as usize].is_some() {
                    stdout.queue(SetColors(Colors::new(
                        current_piece.tetromino.color,
                        Color::Reset,
                    )));
                    stdout.queue(Print("▮"));
                } else if board[(by - BOARDY) as usize][(bx - BOARDX) as usize].is_some() {
                    stdout.queue(SetColors(Colors::new(
                        board[(by - BOARDY) as usize][(bx - BOARDX) as usize]
                            .expect("what the hell"),
                        Color::Reset,
                    )));
                    stdout.queue(Print("▮"));
                } else if ghost_board[(by - BOARDY) as usize][(bx - BOARDX) as usize].is_some() {
                    queue!(
                        stdout,
                        SetColors(Colors::new(Color::Reset, Color::DarkGrey)),
                        Print("▮")
                    );
                } else {
                    stdout.queue(Print(" "));
                }
                stdout.queue(ResetColor);
            }
            stdout.queue(Print("|"));
        }
        for i in BOARDX..(BOARDWIDTH + BOARDX) {
            queue!(stdout, cursor::MoveTo(i, BOARDHEIGHT + BOARDY));
            stdout.queue(Print("▔"));
        }
        stdout.flush();

        // for i in BOARDX..(BOARDWIDTH + BOARDX) {
        //     queue!(stdout, cursor::MoveTo(i, BOARDHEIGHT + BOARDY));
        //     stdout.queue(Print("▔"));
        // }

        // This is for drawing held piece
        if held_piece.is_some() {
            held_piece_board = vec![vec![None; HELDBOARDWIDTH as usize]; HELDBOARDHEIGHT as usize];
            held_piece_board = place_piece_in_board(
                &held_piece_board,
                &MovingTetromino {
                    tetromino: held_piece.clone().expect("wtf"),
                    coords: (HELDBOARDX + 1, HELDBOARDY + 1),
                },
                HELDBOARDX,
                HELDBOARDY,
            );
        }
        draw_board(&held_piece_board, HELDBOARDX, HELDBOARDY);
        stdout.flush();

        bag.reverse();
        for i in 0..5 {
            let mut piece_tmp: MovingTetromino = MovingTetromino {
                tetromino: bag[i].clone(),
                coords: (37, i as u16 * 4 + 3),
            };
            next_pieces_board = place_piece_in_board(&next_pieces_board, &piece_tmp, 36, 2)
        }

        draw_board(&next_pieces_board, 36, 2);
        stdout.flush();
        // to reset the board
        next_pieces_board = vec![vec![None; 6]; 20];
        bag.reverse();

        if poll(Duration::from_millis(50)).unwrap() {
            if let Event::Key(key_event) = read().unwrap() {
                match key_event.kind {
                    KeyEventKind::Press => {
                        match key_event.code {
                            KeyCode::Char('a') | KeyCode::Char('h') | KeyCode::Left => {
                                if check_valid_board(&board, &current_piece, -1, 0) {
                                    current_piece.coords.0 -= 1;
                                }
                            }
                            KeyCode::Char('d') | KeyCode::Char('l') | KeyCode::Right => {
                                if check_valid_board(&board, &current_piece, 1, 0) {
                                    current_piece.coords.0 += 1;
                                }
                            }
                            KeyCode::Char('s') | KeyCode::Char('j') | KeyCode::Down => {
                                if check_valid_board(&board, &current_piece, 0, 1) {
                                    current_piece.coords.1 += 1;
                                }
                            }
                            KeyCode::Char('w') | KeyCode::Char('k') | KeyCode::Up => {
                                if check_valid_board(&board, &rotate_right(&current_piece), 0, 0) {
                                    current_piece = rotate_right(&current_piece);
                                }
                            }
                            KeyCode::Char('r') | KeyCode::Char('R') => {
                                //reset("TODO")
                            }
                            KeyCode::Char('c') => {
                                if !has_held_piece {
                                    has_held_piece = true;
                                    if held_piece.is_some() {
                                        let mut temp: Option<Tetromino>;
                                        temp = held_piece.clone();
                                        held_piece = Some(current_piece.tetromino.clone());
                                        current_piece = MovingTetromino {
                                            tetromino: temp
                                                .expect("something with held piece")
                                                .clone(),
                                            coords: (PIECESTARTX, 2),
                                        }
                                    } else {
                                        held_piece = Some(current_piece.tetromino.clone());

                                        let mut rand_mino: Tetromino;

                                        loop {
                                            let mut random_tetromino = bag.pop();
                                            match random_tetromino {
                                                Some(mino) => {
                                                    rand_mino = mino;

                                                    if bag.len() < 5 {
                                                        bag = append_to_bag(&bag);
                                                    }
                                                    break;
                                                }
                                                None => bag = make_bag(),
                                            }
                                        }
                                        current_piece = MovingTetromino {
                                            tetromino: rand_mino,
                                            coords: (PIECESTARTX, 2),
                                        };
                                    }
                                }
                            }

                            KeyCode::End => {
                                board =
                                    place_piece_in_board(&board, &current_piece, BOARDX, BOARDY);
                                debug_mode = !debug_mode;
                                if !debug_mode {
                                    queue!(stdout, terminal::Clear(terminal::ClearType::All));
                                }
                            }
                            KeyCode::Char(' ') => {
                                ticks_on_ground = 20;
                                while check_valid_board(&board, &current_piece, 0, 1) {
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
            if check_valid_board(&board, &current_piece, 0, 1) {
                current_piece.coords.1 += 1;
            }
        }

        if !check_valid_board(&board, &current_piece, 0, 1) {
            ticks_on_ground += 1;
        }

        if ticks_on_ground >= 20 {
            let mut rand_mino: Tetromino;

            loop {
                let mut random_tetromino = bag.pop();
                match random_tetromino {
                    Some(mino) => {
                        rand_mino = mino;
                        if bag.len() < 5 {
                            bag = append_to_bag(&bag)
                        }
                        break;
                    }
                    None => bag = make_bag(),
                }
            }

            board = place_piece_in_board(&board, &current_piece, BOARDX, BOARDY);
            board = clear_lines(board);

            current_piece = MovingTetromino {
                tetromino: rand_mino,
                coords: (PIECESTARTX, 2),
            };
            // if !check_valid_board(&board,&MovingTetromino {tetromino: current_piece.tetromino.clone(),coords: (PIECESTARTX, 3),},0,0,) {cleanup();}
            ticks_on_ground = 0;
            has_held_piece = false;
        }
    }
}

fn draw_board(board: &Vec<Vec<Option<Color>>>, board_x: u16, board_y: u16) {
    let board_height: u16 = board.len() as u16;
    let board_width: u16 = board[0].len() as u16;
    let mut stdout = stdout();

    for by in board_y..(board_height + board_y) {
        queue!(stdout, cursor::MoveTo(board_x, by));
        stdout.queue(Print("|")).unwrap();

        for bx in board_x..(board_width + board_x) {
            if board[(by - board_y) as usize][(bx - board_x) as usize].is_some() {
                stdout.queue(SetColors(Colors::new(
                    board[(by - board_y) as usize][(bx - board_x) as usize].expect("what the hell"),
                    Color::Reset,
                )));
                stdout.queue(Print("▮"));
            } else {
                stdout.queue(Print(" "));
            }
            stdout.queue(ResetColor);
        }
        stdout.queue(Print("|"));
    }
    stdout.flush();
}

fn make_bag() -> Vec<Tetromino> {
    let mut list_of_tetrominos = vec![
        Tetromino::new_i(),
        Tetromino::new_j(),
        Tetromino::new_t(),
        Tetromino::new_l(),
        Tetromino::new_o(),
        Tetromino::new_s(),
        Tetromino::new_z(),
    ];

    let mut new_vec = Vec::with_capacity(list_of_tetrominos.len());
    let mut random = random_u32(list_of_tetrominos.len() as u32 - 1);

    while list_of_tetrominos.len() > 1 {
        new_vec.push(list_of_tetrominos[random as usize].clone());
        list_of_tetrominos.remove(random as usize);
        random = random_u32(list_of_tetrominos.len() as u32);
    }
    new_vec
}

fn append_to_bag(bag: &Vec<Tetromino>) -> Vec<Tetromino> {
    let mut bag_tmp: Vec<Tetromino> = bag.clone();
    let mut new_bag = make_bag();

    for tetromino in new_bag {
        bag_tmp.insert(0, tetromino)
    }
    bag_tmp
}

fn random_u32(end: u32) -> u32 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    let random: u32 = (time % end);

    random
}
fn place_piece_in_board(
    board: &Vec<Vec<Option<Color>>>,
    piece: &MovingTetromino,
    board_x: u16,
    board_y: u16,
) -> Vec<Vec<Option<Color>>> {
    let mut tmp_board: Vec<Vec<Option<Color>>> = board.clone();
    let piece_x: u16 = piece.coords.0;
    let piece_y: u16 = piece.coords.1;
    let piece_shape = piece.tetromino.shape.clone();

    for x in piece_x..(piece_x + piece_shape[0].len() as u16) {
        for y in piece_y..(piece_y + piece_shape.len() as u16) {
            if piece_shape[(y - piece_y) as usize][(x - piece_x) as usize] {
                tmp_board[(y - board_y) as usize][(x - board_x) as usize] =
                    Some(piece.tetromino.color);
            }
        }
    }
    tmp_board
}

fn clear_lines(board: Vec<Vec<Option<Color>>>) -> Vec<Vec<Option<Color>>> {
    let mut board_tmp = board.to_vec();
    if board[board.len() - 1].iter().all(|&item| item.is_some()) {
        let mut board_tmp_tmp = vec![vec![None; board[0].len()]; board.len()];
        while board_tmp[board.len() - 1]
            .iter()
            .all(|&item| item.is_some())
        {
            board_tmp[board.len() - 1] = vec![None; 10];

            for i in 1..(board.len()) {
                board_tmp_tmp[i] = board_tmp[(i - 1)].clone();
            }

            board_tmp = board_tmp_tmp.clone()
        }
        return board_tmp_tmp;
    }
    board
}

fn shift_down_board(board: Vec<Vec<Option<Color>>>, amount: u16) -> Vec<Vec<Option<Color>>> {
    // TODO
    vec![vec![None; 10]; 10]
}

fn check_valid_board(
    board: &Vec<Vec<Option<Color>>>,
    piece: &MovingTetromino,
    dx: i16,
    dy: i16,
) -> bool {
    let mut tmp_board: &Vec<Vec<Option<Color>>> = board;
    let piece_x = piece.coords.0 as i16 + dx;
    let piece_y = piece.coords.1 as i16 + dy;

    let piece_shape = piece.tetromino.shape.clone();

    if piece_x < 0 || piece_y < 0 {
        return false;
    } else {
        let upiece_x = piece_x as u16;
        let upiece_y = piece_y as u16;

        for x in upiece_x..(upiece_x + piece_shape[0].len() as u16) {
            for y in upiece_y..(upiece_y + piece_shape.len() as u16) {
                if piece_shape[(y - upiece_y) as usize][(x - upiece_x) as usize] {
                    if x < BOARDX || x >= BOARDX + BOARDWIDTH {
                        return false;
                    } else if y <= BOARDY || y >= BOARDY + BOARDHEIGHT {
                        return false;
                    }
                }
            }
        }
        let mut tmp_piece = piece.clone();
        tmp_piece.coords = (upiece_x, upiece_y);
        let board_with_placed_piece = place_piece_in_board(
            &vec![vec![None; BOARDWIDTH as usize]; BOARDHEIGHT as usize],
            &tmp_piece,
            BOARDX,
            BOARDY,
        );
        for x in 0..(BOARDWIDTH as usize) {
            for y in 0..(BOARDHEIGHT as usize) {
                if board_with_placed_piece[y][x].is_some() && board[y][x].is_some() {
                    return false;
                }
            }
        }
        return true;
    }
    true
}

fn rotate_right(piece: &MovingTetromino) -> MovingTetromino {
    let mut temp_vec_board = Vec::new();
    let shape = piece.tetromino.shape.clone();

    let with_and_height: usize = shape.len();
    for i in 0..with_and_height {
        let mut temp_vec_rows = Vec::new();
        for j in (0..with_and_height).rev() {
            temp_vec_rows.push(piece.tetromino.shape[j][i]);
        }
        temp_vec_board.push(temp_vec_rows);
    }

    let mut piece_rotated = piece.clone();
    piece_rotated.tetromino.shape = temp_vec_board;

    piece_rotated.clone()
}
