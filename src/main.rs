use macroquad::prelude::*;

// 012
// 345
// 678

const EMPTY_ROW: [BoxState; 3] = [E, E, E];
const WIN_MOVES: [[i32; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [6, 4, 2],
];
const X: BoxState = BoxState::X;
const O: BoxState = BoxState::O;
const E: BoxState = BoxState::Empty;

// bot = o | player = x
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoxState {
    X,
    O,
    Empty,
}

struct BigGrid {
    grid: [[MiniSquare; 3]; 3],
}

#[derive(Debug, Clone, Copy)]
struct MiniSquare {
    boxes: [[BoxState; 3]; 3],
}

impl MiniSquare {
    fn blanks() -> MiniSquare {
        MiniSquare {
            boxes: [EMPTY_ROW, EMPTY_ROW, EMPTY_ROW],
        }
    }

    fn flatten(&self) -> [BoxState; 9] {
        [
            self.boxes[0][0],
            self.boxes[0][1],
            self.boxes[0][2],
            self.boxes[1][0],
            self.boxes[1][1],
            self.boxes[1][2],
            self.boxes[2][0],
            self.boxes[2][1],
            self.boxes[2][2],
        ]
    }

    fn print(&self) {
        for row in 0..3 {
            for column in 0..3 {
                print!(
                    "{}",
                    match self.boxes[column][row] {
                        BoxState::X => "X",
                        BoxState::O => "O",
                        BoxState::Empty => "•",
                    }
                )
            }
            println!();
        }
    }

    fn bot_move(&mut self) {
        let mut move_options: Vec<(i32, i32)> = (0..9).map(|a| (a, 0)).collect();
        for opponent_move_flat_index in 0..9 {
            let mut hypothetical_future = self.clone();
            let opponent_move = (
                ((opponent_move_flat_index) as f32 / 3.0).floor() as usize,
                opponent_move_flat_index as usize
                    - ((opponent_move_flat_index) as f32 / 3.0).floor() as usize * 3,
            );

            if hypothetical_future.boxes[opponent_move.0][opponent_move.1] == BoxState::Empty {
                hypothetical_future.boxes[opponent_move.0][opponent_move.1] = O
            } else {
                continue;
            };
            move_options[opponent_move_flat_index] = (
                opponent_move_flat_index as i32,
                hypothetical_future.score(6),
            );
        }

        move_options.sort_by(|a, b| a.1.cmp(&b.1));
        for option in move_options {
            if self.boxes[((option.0) as f32 / 3.0).floor() as usize][option.0 as usize - ((option.0) as f32 / 3.0).floor() as usize * 3] == E {
                self.boxes[((option.0) as f32 / 3.0).floor() as usize][option.0 as usize - ((option.0) as f32 / 3.0).floor() as usize * 3] = O;
                break
            }
        }
    }

    /// the depth number should be odd if it is the bots turn and evan if it is the humans turn
    fn score(&self, depth: i32) -> i32 {
        let mut score = 0;
        if depth == 0 {
            return 0;
        }
        for box_indices in WIN_MOVES {
            match [
                self.flatten()[box_indices[0] as usize],
                self.flatten()[box_indices[1] as usize],
                self.flatten()[box_indices[2] as usize],
            ] {
                [BoxState::X, BoxState::X, BoxState::X] => return (-10 as i32).pow(depth as u32),
                [BoxState::O, BoxState::O, BoxState::O] => return (10 as i32).pow(depth as u32),
                _ => {}
            }
        }

        for opponent_move_flat_index in 0..9 {
            let mut hypothetical_future = self.clone();
            let opponent_move = (
                ((opponent_move_flat_index) as f32 / 3.0).floor() as usize,
                opponent_move_flat_index as usize
                    - ((opponent_move_flat_index) as f32 / 3.0).floor() as usize * 3,
            );

            if hypothetical_future.boxes[opponent_move.0][opponent_move.1] == BoxState::Empty {
                hypothetical_future.boxes[opponent_move.0][opponent_move.1] =
                    match ((depth as f32 % 2.0) * 10.0) as i32 {
                        0 => X,
                        _ => O,
                    };
                    score += hypothetical_future.score(depth - 1);
            } else {
                continue;
            };

        }

        return score;
    }
}

fn emf(f: f32) -> f32 {
    screen_width() / 100.0 * f
}
fn emi(i: i32) -> f32 {
    emf(i as f32)
}
fn remf(f: f32) -> f32 {
    f / screen_width() / 100.0
}
fn remi(i: i32) -> f32 {
    remf(i as f32)
}

#[macroquad::main("naughts and crosses")]
async fn main() {
    let mut mini_square = MiniSquare::blanks();
    mini_square.bot_move();
    mini_square.boxes[0][0] = X;
    mini_square.bot_move();

    // mini_square.bot_move();
    // mini_square.bot_move();


    let example_squares = vec![
        [[E, X, E], [E, X, E], [E, E, E]],
        [[E, E, O], [E, O, E], [E, X, E]],
        [[O, O, O], [E, E, E], [E, E, E]],
        [[E, E, E], [X, X, X], [E, E, E]],
        [[E, X, E], [E, X, O], [O, E, E]],
        [[E, E, X], [E, O, E], [E, X, E]],
        [[O, O, O], [E, X, E], [E, E, E]],
        [[E, E, E], [X, X, O], [E, O, E]],
    ];
    for example_square in example_squares {
        let mini = MiniSquare {
            boxes: example_square,
        };
        mini.print();
        println!("{}\n", mini.score(5));
    }

    fn draw_game_grid(x: f32, y: f32, game: MiniSquare) {
        draw_line(x, emi(3) + y, emi(9) + x, emi(3) + y, emf(0.2), BLACK);
        draw_line(x, emi(6) + y, emi(9) + x, emi(6) + y, emf(0.2), BLACK);
        draw_line(emi(3) + x, y, emi(3) + x, emi(9) + y, emf(0.2), BLACK);
        draw_line(emi(6) + x, y, emi(6) + x, emi(9) + y, emf(0.2), BLACK);
        if is_mouse_button_pressed(MouseButton::Left) {
            for x in 0..3 {
                for y in 0..3 {

                }
            } 
        }

        for index in 0..9 {
            match game.boxes[((index) as f32 / 3.0).floor() as usize]
                [index as usize - ((index) as f32 / 3.0).floor() as usize * 3]
            {
                X => {
                    draw_text("X", x + emf(0.55) + emi(index* 3), y +emf(2.5)+ emi(index* 3), emi(4), BLACK);
                }
                O => {
                    draw_text("O", x + emf(0.55) + emi(index* 3), y +emf(2.5)+ emi(index* 3), emi(4), BLACK);
                }
                _ => {}
            }
        }
    }

    loop {
        clear_background(WHITE);
        draw_game_grid(emi(60), emi(5), mini_square);
        let mini_score = mini_square.score(5);
        draw_text(
            &format!("score {mini_score}"),
            emi(60),
            emi(18),
            emf(2.0),
            BLACK,
        );

        next_frame().await
    }
}
