use std::{
    fmt::{Display, Write},
    iter::once,
};

use chumsky::prelude::*;
use comfy_table::Table;

type Cord = (usize, usize);

type Cells = [[Player; 3]; 3];

#[derive(PartialEq, Clone, Copy)]
enum Player {
    X,
    O,
    Empty,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => f.write_char('X'),
            Player::O => f.write_char('O'),
            Player::Empty => f.write_char(' '),
        }
    }
}

impl Player {
    fn alternate(&self) -> Self {
        match &self {
            Player::X => Player::O,
            Player::O => Player::X,
            Player::Empty => unreachable!(),
        }
    }
}

const WIN_CONDITIONS: [[Cord; 3]; 8] = [
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    [(0, 0), (1, 1), (2, 2)],
    [(0, 2), (1, 1), (2, 0)],
];

enum GameOver {
    Victory(Player),
    Tie,
}

fn main() {
    let mut cells = [[Player::Empty; 3]; 3];
    let mut player = Player::X;

    let mut game_over = None;

    display_cells(&cells);

    while let None = game_over {
        input_mark(&mut cells, &player);

        player = player.alternate();

        display_cells(&cells);

        game_over = check_game_over(&cells, player.alternate());
    }

    match game_over.unwrap() {
        GameOver::Victory(p) => println!("{} Wins!", p),
        GameOver::Tie => println!("Tie Game!"),
    }
}

fn check_game_over(cells: &Cells, player: Player) -> Option<GameOver> {
    let won = WIN_CONDITIONS
        .iter()
        .map(|[a, b, c]| {
            cells[a.0][a.1] == player && cells[b.0][b.1] == player && cells[c.0][c.1] == player
        })
        .reduce(|a, b| a | b)
        .unwrap();

    if won {
        Some(GameOver::Victory(player))
    } else {
        for row in cells.clone() {
            if row.contains(&Player::Empty) {
                return None;
            }
        }

        Some(GameOver::Tie)
    }
}

fn input_mark(cells: &mut Cells, player: &Player) {
    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        if let Ok(c) = coord_parser().parse(input) {
            if cells[c.0][c.1] != Player::Empty {
                println!("This Position Cannot Be Overwriten!");
                continue;
            } else {
                cells[c.0][c.1] = player.clone();
                break;
            }
        } else {
            println!("Invalid Position");
            continue;
        }
    }
}

fn display_cells(cells: &Cells) {
    let mut table = Table::new();

    table.set_header(vec!["#", "1", "2", "3"]);

    for (row, c) in ["A", "B", "C"].iter().enumerate() {
        table.add_row(once(c.to_string()).chain(cells[row].iter().map(|p| format!("{}", p))));
    }

    println!("{table}");
}

fn coord_parser() -> impl Parser<char, Cord, Error = Simple<char>> {
    let letter = filter(|c: &char| match c {
        'a' | 'b' | 'c' | 'A' | 'B' | 'C' => true,
        _ => false,
    })
    .map(|c: char| match c {
        'a' | 'A' => 0,
        'b' | 'B' => 1,
        'c' | 'C' => 2,
        _ => unreachable!(),
    });

    let number = filter(|c: &char| match c {
        '1' | '2' | '3' => true,
        _ => false,
    })
    .map(|c: char| match c {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        _ => unreachable!(),
    });

    number.then(letter).map(|(b, a)| (a, b))
}
