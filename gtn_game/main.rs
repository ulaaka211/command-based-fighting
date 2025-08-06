use dialoguer::{Input, Select};
use console::Style;
use rand::{Rng};
use std::cmp::Ordering;
use std::collections::HashSet;

enum GameAction {
    RandomNumber,
    GuessNumber,
}

impl GameAction {
    fn create_random_number() -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=100)
    }
}

fn main() {
    println!(
        "{}",
        Style::new()
            .bold()
            .blue()
            .apply_to("=== Guess the Number ===")
    );

    let choices = [
        &Style::new().yellow().apply_to("Generate a random number").to_string(),
        &Style::new().yellow().apply_to("Play guess-the-number").to_string(),
    ];
    let selection = Select::new()
        .with_prompt(Style::new().cyan().apply_to("Choose an action").to_string())
        .items(&choices)
        .default(0)
        .interact()
        .expect("Failed to read your choice");

    let action: GameAction = if selection == 0 {
        GameAction::RandomNumber
    } else {
        GameAction::GuessNumber
    };

    match action {
        GameAction::RandomNumber => {
            let n = GameAction::create_random_number();
            if n == 3 {
                println!(
                    "{}",
                    Style::new()
                        .magenta()
                        .bold()
                        .apply_to(format!("🎲 You've got luckiest number between 1 and 100: {}", n))
                );
            } else {
                println!(
                    "{}",
                    Style::new()
                        .magenta()
                        .apply_to(format!("🎲 Your random number is: {}", n))
                );
            }
        }

        GameAction::GuessNumber => {
            let taunts = [
                Style::new().red().apply_to("Nice try… kiddo! 😉").to_string(),
                Style::new().red().apply_to("Is that all you’ve got? 🤔").to_string(),
                Style::new().red().apply_to("Not even close! 😑").to_string(),
                Style::new().red().apply_to("Epic fail! 💥").to_string(),
            ];
            
            let secret = GameAction::create_random_number();
            let mut attempts = 5;
            let mut taunt_index = 0;
            let mut seen = HashSet::new();

            println!(
                "{}",
                Style::new()
                    .cyan()
                    .apply_to(format!(
                        "I have chosen a number between 1 and 100. You have {} attempts.",
                        attempts
                    ))
            );

            while attempts > 0 {
                 let prompt = Style::new()
                    .green()
                    .apply_to(format!("Your guess ({} left)", attempts))
                    .to_string();

                let raw_input: String = Input::new()
                    .with_prompt(prompt)
                    .interact_text()
                    .expect("Failed to read the guess");
                
                let guess: i32 = match raw_input.trim().parse::<i32>() {

                   
                    Ok(n) if (1..=100).contains(&n) => n,
                
                    
                    Ok(n) if n < 1 => {
                        attempts -= 1;

                        if attempts == 0 {
                        let fail_msg = format!(
                        "What a joke. The number was {}. Try harder next time… if you can.",
                        secret
                        );

                        println!("{}", Style::new().red().bold().apply_to(fail_msg));
                        break;
                    
                    }
                        println!("{} is below 1, you can do better than that!", n);
                        continue;
                    }
                    
                    Ok(n) if n > 100 => {
                        attempts -= 1;

                        if attempts == 0 {
                        let fail_msg = format!(
                        "What a joke. The number was {}. Try harder next time… if you can.",
                        secret
                        );

                        println!("{}", Style::new().red().bold().apply_to(fail_msg));
                        break;
                    
                    }

                        println!("{}? Are you even reading the rules? 1–100, buddy!", n);
                        continue;
                    }

                
                
                    Err(_) => {
                        attempts -= 1;

                        if attempts == 0 {
                        let fail_msg = format!(
                        "What a joke. The number was {}. Try harder next time… if you can.",
                        secret
                        );

                        println!("{}", Style::new().red().bold().apply_to(fail_msg));
                        break;
                    
                    }
                        println!("`{}` isn’t even a number! Try again.", raw_input);
                        continue;
                    }                   
                
                    _ => unreachable!(),
                };                


                 if seen.contains(&guess) {
                    attempts -= 1;
                    println!(
                        "{}",
                        Style::new()
                            .red()
                            .bold()
                            .apply_to("LOL, You've already tried that.")
                    );

                    if attempts == 0 {
                        let fail_msg = format!(
                        "What a joke. The number was {}. Try harder next time… if you can.",
                        secret
                        );

                        println!("{}", Style::new().red().bold().apply_to(fail_msg));
                    
                    }

                    continue;
                }

                match guess.cmp(&secret) {
                    Ordering::Less => {
                        if attempts != 1 {
                            seen.insert(guess);
                            attempts -= 1;
                            let idx = taunt_index.min(taunts.len() - 1);
                            let taunt = &taunts[idx];
                            taunt_index += 1;
                            println!(
                                "{}\n{}",
                                Style::new().bold().red().apply_to("📉 Too small!"),
                                taunt
                            );
                        } else  {
                            attempts -= 1
                        }
                    }
                    Ordering::Greater => {
                        if attempts != 1 {
                            seen.insert(guess);
                            attempts -= 1;
                            let idx = taunt_index.min(taunts.len() - 1);
                            let taunt = &taunts[idx];
                            taunt_index += 1;
                            println!(
                                "{}\n{}",
                                Style::new().bold().red().apply_to("📈 Too big!"),
                                taunt
                            );
                        } else  {
                            attempts -= 1
                        }
                    }
                    Ordering::Equal => {
                        let msg = if attempts == 5 {
                            "🎉🎉🎉 You got it on the very first try! What a genius🎉🎉🎉"
                        } else {
                            "🎉 You got it! 🎉"
                        };
                        println!("{}", Style::new().green().bold().apply_to(msg));
                        return;
                    }
                }

                if attempts == 0 {
                    let fail_msg = format!(
                        "The number was {}. Try harder next time… if you can.",
                        secret
                    );
                    println!("{}", Style::new().red().bold().apply_to(fail_msg));
                }
            }
        }
    }
}


