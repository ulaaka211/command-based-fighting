use dialoguer::Select;
use console::Style;

#[derive(Debug, PartialEq, Eq)]
enum GameAction { Rock, Paper, Scissor }

impl GameAction {
    fn from_index(idx: usize) -> Self {
        match idx {
            0 => Self::Rock,
            1 => Self::Paper,
            2 => Self::Scissor,
            _ => unreachable!(),
        }
    }
}

fn check_winner(a1: &GameAction, a2: &GameAction) -> &'static str {
    if a1 == a2 {
        "Draw"
    } else if (a1 == &GameAction::Rock    && a2 == &GameAction::Scissor)
           || (a1 == &GameAction::Paper   && a2 == &GameAction::Rock)
           || (a1 == &GameAction::Scissor && a2 == &GameAction::Paper)
    {
        "Player 1"
    } else {
        "Player 2"
    }
}

fn main() {
    let items = &["Rock", "Paper", "Scissor"];

    println!("=== Rock, Paper, Scissors ===");

    let p1 = GameAction::from_index(
        Select::new()
            .with_prompt("Player 1: Choose your move")
            .items(items)
            .default(0)
            .interact()
            .unwrap()
    );

    let p2 = GameAction::from_index(
        Select::new()
            .with_prompt("Player 2: Choose your move")
            .items(items)
            .default(1)
            .interact()
            .unwrap()
    );

    let result = check_winner(&p1, &p2);

    match result {
        "Draw"      => println!("{}", Style::new().yellow().bold().apply_to("It's a draw!")),
        "Player 1"  => println!("{}", Style::new().green().bold().apply_to("Player 1 wins!")),
        "Player 2"  => println!("{}", Style::new().green().bold().apply_to("Player 2 wins!")),
        _           => unreachable!(),
    }
}
