use macroquad::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

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
const I: BoxState = BoxState::Invalid;

const BOT_DEPTH: i32 = 7;
const PLAYER_MOVE_DEPTH: i32 = 6;

// bot = o | player = x
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BoxState {
    X,
    O,
    Empty,
    Invalid,
}

struct BigGrid {
    grid: [[MiniSquare; 3]; 3],
}

impl BigGrid {
    fn blank() -> BigGrid {
        BigGrid {
            grid: [
                [
                    MiniSquare::blanks(),
                    MiniSquare::blanks(),
                    MiniSquare::blanks(),
                ],
                [
                    MiniSquare::blanks(),
                    MiniSquare::blanks(),
                    MiniSquare::blanks(),
                ],
                [
                    MiniSquare::blanks(),
                    MiniSquare::blanks(),
                    MiniSquare::blanks(),
                ],
            ],
        }
    }

    fn mini_clone(&self, saved_scores: &mut HashMap<[[BoxState; 3]; 3], i32>) -> MiniSquare {
        let mut mini = MiniSquare::blanks();
        for x in 0..3 {
            for y in 0..3 {
                mini.boxes[y][x] = self.grid[y][x].winner(saved_scores);
            }
        }
        return mini;
    }

    fn flat_index(&mut self, index: usize) -> &mut MiniSquare {
        let row = ((index) as f32 / 3.0).floor() as usize;
        let column = index as usize - ((index) as f32 / 3.0).floor() as usize * 3;
        &mut self.grid[row][column]
    }

    fn bot_move(&mut self, saved_scores: &mut HashMap<[[BoxState; 3]; 3], i32>) {
        let mut mini_clone = self.mini_clone(saved_scores);
        mini_clone.bot_move(saved_scores);
        let mini_clone2 = self.mini_clone(saved_scores);

        for mini_grid_index in 0..9 {
            if mini_clone2.flatten()[mini_grid_index] != mini_clone.flatten()[mini_grid_index] {
                self.flat_index(mini_grid_index).bot_move(saved_scores);
                break;
            }
        }
    }
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

    fn winner(&self, saved_scores: &mut HashMap<[[BoxState; 3]; 3], i32>) -> BoxState {
        let empty_spaces: Vec<BoxState> = self.flatten().to_vec();
        let empty_spaces: Vec<BoxState> = empty_spaces
            .into_iter()
            .filter(|&a| a == BoxState::Empty)
            .collect();
        match self.score(BOT_DEPTH, saved_scores) {
            _ if self.score(1, saved_scores) == 10 => O,
            _ if self.score(1, saved_scores) == -10 => X,
            _ if empty_spaces.len() <= 2
                && self.score(2, saved_scores) == 0
                && self.score(1, saved_scores) == 0 =>
            {
                I
            }
            _ => E,
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
                        BoxState::Invalid => " ",
                    }
                )
            }
            println!();
        }
    }

    fn bot_move(&mut self, saved_scores: &mut HashMap<[[BoxState; 3]; 3], i32>) {
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
                hypothetical_future.score(PLAYER_MOVE_DEPTH, saved_scores),
            );
        }

        move_options.sort_by(|a, b| b.1.cmp(&a.1));
        for option in move_options {
            if self.boxes[((option.0) as f32 / 3.0).floor() as usize]
                [option.0 as usize - ((option.0) as f32 / 3.0).floor() as usize * 3]
                == E
            {
                self.boxes[((option.0) as f32 / 3.0).floor() as usize]
                    [option.0 as usize - ((option.0) as f32 / 3.0).floor() as usize * 3] = O;
                break;
            }
        }
    }

    /// the depth number should be odd if it is the bots turn and evan if it is the humans turn
    fn score(&self, depth: i32, saved_scores: &mut HashMap<[[BoxState; 3]; 3], i32>) -> i32 {
        match self.boxes {
            [[O, E, E], [E, E, E], [E, E, E]] => {
                return 3600;
            }
            [[E, O, E], [E, E, E], [E, E, E]] => {
                return 2400;
            }
            [[E, E, O], [E, E, E], [E, E, E]] => {
                return 3600;
            }

            [[E, E, E], [O, E, E], [E, E, E]] => {
                return 3600;
            }
            [[E, E, E], [E, O, E], [E, E, E]] => {
                return 4800;
            }
            [[E, E, E], [E, E, O], [E, E, E]] => {
                return 3600;
            }

            [[E, E, E], [E, E, E], [O, E, E]] => {
                return 3600;
            }
            [[E, E, E], [E, E, E], [E, O, E]] => {
                return 2400;
            }
            [[E, E, E], [E, E, E], [E, E, O]] => {
                return 3600;
            }
            _ => {}
        };

        if saved_scores.contains_key(&self.boxes) {
            return *saved_scores.get(&self.boxes).unwrap();
        }

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
                score += hypothetical_future.score(depth - 1, saved_scores);
            } else {
                continue;
            };
        }

        saved_scores.insert(self.boxes, score);

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
const DEBUG: bool = true;

#[macroquad::main("naughts and crosses")]
async fn main() {
    let mut saved_scores: HashMap<[[BoxState; 3]; 3], i32> = HashMap::new();

    let mut mini_square = MiniSquare::blanks();
    let mut big_grid = BigGrid::blank();

    let example_squares = vec![
        [[E, X, E], [E, X, E], [E, E, E]],
        [[E, E, O], [E, O, E], [E, X, E]],
        [[O, O, O], [E, E, E], [E, E, E]],
        [[E, E, E], [X, X, X], [E, E, E]],
        [[E, X, E], [E, X, O], [O, E, E]],
        [[E, E, X], [E, O, E], [E, X, E]],
        [[O, O, O], [E, X, E], [E, E, E]],
        [[E, E, E], [X, X, O], [E, O, E]],
        [[E, E, E], [E, X, O], [O, E, E]],
    ];
    let mut mini = vec![];
    for example_square in example_squares {
        mini.push(MiniSquare {
            boxes: example_square,
        });
    }

    let mut fps_timer = Instant::now();

    fn draw_game_grid(
        x: f32,
        y: f32,
        game: MiniSquare,
        fps_timer: Instant,
        saved_scores: &mut HashMap<[[BoxState; 3]; 3], i32>,
    ) -> Option<(i32, i32)> {
        draw_line(x, emi(3) + y, emi(9) + x, emi(3) + y, emf(0.2), BLACK);
        draw_line(x, emi(6) + y, emi(9) + x, emi(6) + y, emf(0.2), BLACK);
        draw_line(emi(3) + x, y, emi(3) + x, emi(9) + y, emf(0.2), BLACK);
        draw_line(emi(6) + x, y, emi(6) + x, emi(9) + y, emf(0.2), BLACK);

        match game.winner(saved_scores) {
            X => {
                draw_line(
                    x + emf(1.0),
                    y + emf(1.0),
                    x + emf(8.0),
                    y + emf(8.0),
                    emf(0.6),
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.5,
                    },
                );
                draw_circle(
                    x + emf(1.0),
                    y + emf(1.0),
                    emf(0.3),
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.5,
                    },
                );
                draw_circle(
                    x + emf(8.0),
                    y + emf(8.0),
                    emf(0.3),
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.5,
                    },
                );

                draw_line(
                    x + emf(8.0),
                    y + emf(1.0),
                    x + emf(1.0),
                    y + emf(8.0),
                    emf(0.6),
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.5,
                    },
                );
                draw_circle(
                    x + emf(8.0),
                    y + emf(1.0),
                    emf(0.3),
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.5,
                    },
                );
                draw_circle(
                    x + emf(1.0),
                    y + emf(8.0),
                    emf(0.3),
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.5,
                    },
                );
            }
            O => draw_circle_lines(
                x + emf(4.5),
                y + emf(4.5),
                emi(4),
                emf(0.6),
                Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.5,
                },
            ),
            _ => {}
        }

        for index in 0..9 {
            match game.boxes[((index) as f32 / 3.0).floor() as usize]
                [index as usize - ((index) as f32 / 3.0).floor() as usize * 3]
            {
                X => {
                    draw_text(
                        "X",
                        x + emf(0.55 + 3.0 * (index as f32 - (index as f32 / 3.0).floor() * 3.0)),
                        y + emf(2.5 + 3.0 * (index as f32 / 3.0).floor()),
                        emi(4),
                        BLACK,
                    );
                }
                O => {
                    draw_text(
                        "O",
                        x + emf(0.55 + 3.0 * (index as f32 - (index as f32 / 3.0).floor() * 3.0)),
                        y + emf(2.5 + 3.0 * (index as f32 / 3.0).floor()),
                        emi(4),
                        BLACK,
                    );
                }
                _ => {
                    if DEBUG {
                        let mut copy_of_game = game.clone();

                        if copy_of_game.boxes[(index as f32 / 3.0).floor() as usize]
                            [index - ((index as f32 / 3.0).floor() as usize * 3)]
                            == E
                        {
                            copy_of_game.boxes[(index as f32 / 3.0).floor() as usize]
                                [index - ((index as f32 / 3.0).floor() as usize * 3)] = O;
                            let mini_box_score = copy_of_game.score(BOT_DEPTH, saved_scores);
                            draw_text(
                                &format!("{}", mini_box_score),
                                x + emf(3.1 * (index as f32 - (index as f32 / 3.0).floor() * 3.0)),
                                y + emf(2.8 + 3.0 * (index as f32 / 3.0).floor()),
                                emf(1.1),
                                match mini_box_score > 0 {
                                    true if mini_box_score == 0 => GRAY,
                                    true => GREEN,
                                    false => RED,
                                },
                            );
                        }
                    }
                }
            }
        }

        if DEBUG {
            draw_text(
                &format!(
                    "{}:{}",
                    match game.winner(saved_scores) {
                        X => "X",
                        O => "O",
                        E => "E",
                        I => "I",
                    },
                    game.score(BOT_DEPTH, saved_scores)
                ),
                x + emf(0.0),
                y + emf(0.0),
                emf(1.0),
                BLACK,
            );
            draw_rectangle(
                x,
                y,
                emf(fps_timer.elapsed().as_secs_f32() * 15.0),
                emf(0.5),
                match fps_timer.elapsed().as_secs_f32() {
                    a if a < 0.05 => GREEN,
                    a if a < 0.1 => YELLOW,
                    _ => RED,
                },
            );
        }
        if is_mouse_button_pressed(MouseButton::Left)
            && (game.winner(saved_scores) == E || game.winner(saved_scores) == I)
        {
            let mouse = mouse_position();
            for x2 in 0..3 {
                for y2 in 0..3 {
                    if mouse.0 > emi(x2 * 3) + x
                        && mouse.0 < emi(x2 * 3 + 3) + x
                        && mouse.1 > emi(y2 * 3) + y
                        && mouse.1 < emi(y2 * 3 + 3) + y
                    {
                        println!("{} {}", x2, y2);
                        return Some((y2, x2));
                    }
                }
            }
        }

        return None;
    }

    loop {
        clear_background(WHITE);
        match draw_game_grid(emi(60), emi(5), mini_square, fps_timer, &mut saved_scores) {
            Some(a) => {
                if mini_square.boxes[a.0 as usize][a.1 as usize] == E {
                    mini_square.boxes[a.0 as usize][a.1 as usize] = X;
                    mini_square.bot_move(&mut saved_scores);
                    if mini_square.score(BOT_DEPTH, &mut saved_scores).abs() == 100000
                        || !mini_square.flatten().contains(&E)
                    {
                        mini_square = MiniSquare::blanks();
                    }
                }
            }
            _ => {}
        };

        let mut old_timer = Instant::now();
        for sub_grid_index in 0..9 {
            let game = big_grid.flat_index(sub_grid_index);

            match draw_game_grid(
                emf(10.0 * (sub_grid_index as f32 - (sub_grid_index as f32 / 3.0).floor() * 3.0)),
                emf(1.0 + 10.0 * (sub_grid_index as f32 / 3.0).floor()),
                game.clone(),
                old_timer,
                &mut saved_scores,
            ) {
                Some(a) => {
                    if big_grid.flat_index(sub_grid_index).boxes[a.0 as usize][a.1 as usize] == E {
                        big_grid.flat_index(sub_grid_index).boxes[a.0 as usize][a.1 as usize] = X;
                        big_grid.bot_move(&mut saved_scores);
                    }
                }
                _ => {}
            };
            if big_grid
                .mini_clone(&mut saved_scores)
                .winner(&mut saved_scores)
                != E
            {
                big_grid = BigGrid::blank();
            }

            old_timer = Instant::now();
        }

        let mini_score = mini_square.score(5, &mut saved_scores);
        draw_text("score", emi(60), emi(18), emf(2.0), BLACK);
        draw_text(
            &format!("{mini_score}"),
            emi(65),
            emi(18),
            emf(2.0),
            match mini_score > 0 {
                true if mini_score == 0 => GRAY,
                true => GREEN,
                false => RED,
            },
        );

        // fps timer
        draw_text(
            format!("fps: {}", 1.0 / fps_timer.elapsed().as_secs_f32()).as_str(),
            emi(3),
            emi(32),
            emf(1.0),
            BLACK,
        );
        draw_text(
            format!(
                "Hashmap: {}/{}",
                saved_scores.len(),
                saved_scores.capacity()
            )
            .as_str(),
            emi(3),
            emi(33),
            emf(1.0),
            BLACK,
        );

        fps_timer = Instant::now();

        next_frame().await
    }
}
