// src/combat.rs

use rand::{thread_rng, Rng};
use console::Style;
use crate::actor::{Actor, Actions};
use crate::ui::{display_stats, prompt_action};
use crate::bot_dragon::take_turn;

#[derive(Debug, Clone, Copy)]
pub enum Turn {
    Player,
    BotDragon,
}

pub fn run_combat() {
    let action_menu = &[
        "Attack", "Heavy attack", "Defend", "Heal", "Dodge", "Special Skill",
    ];

    println!("{}", Style::new().bold().blue().apply_to("=== Turn-Based Combat ==="));

    let mut player = Actor::new("Hero");
    let mut dragon = Actor::new("Dragon");
    let mut turn = Turn::Player;

    while player.is_alive() && dragon.is_alive() {
        println!("\n— {:?}’s turn —", turn);

        match turn {
            Turn::Player => {
                display_stats(&player);
                display_stats(&dragon);

                let choice = prompt_action(action_menu);
                let colored = match choice {
                    0 => Style::new().green(),
                    1 => Style::new().red(),
                    2 => Style::new().cyan(),
                    3 => Style::new().blue(),
                    4 => Style::new().yellow(),
                    5 => Style::new().magenta(),
                    _ => Style::new(),
                }
                .apply_to(action_menu[choice]);
                println!("You chose: {}", colored);

                match choice {
                    0 => player.attack(&mut dragon),
                    1 => player.heavy_attack(&mut dragon),
                    2 => player.defend(),
                    3 => player.heal(),
                    4 => player.dodge(),
                    5 => player.special_skill(&mut dragon),
                    _ => unreachable!(),
                }

                turn = Turn::BotDragon;
            }

            Turn::BotDragon => {
                display_stats(&dragon);
                display_stats(&player);

                take_turn(&mut dragon, &mut player, action_menu);

                // regen + clear flags
                player.mana    = (player.mana    + 1).min(player.max_mana);
                player.stamina = (player.stamina + 1).min(player.max_stamina);
                dragon.mana    = (dragon.mana    + 1).min(dragon.max_mana);
                dragon.stamina = (dragon.stamina + 1).min(dragon.max_stamina);
                player.is_defending = false;
                dragon.is_defending = false;

                turn = Turn::Player;
            }
        }
    }

    // Show final health bars
    println!("\n{}", Style::new().bold().yellow().apply_to("============= FINAL STATS ============="));
    display_stats(&player);
    display_stats(&dragon);

    // Outcome handling
    if player.is_alive() {
        print_victory(&player.name);
    } else {
        print_defeat();
    }
}

/// Prints a trophy ASCII art + a styled victory message.
fn print_victory(victor: &str) {
    println!("{}", Style::new().bold().green().underlined().apply_to("============= VICTORY ============="));

    let trophy = r#"
       ___________
      '._==_==_=_.'
      .-\\:      /-.
     | (|:.     |) |
      '-|:.     |-' 
        \\::.    /
         '::. .' 
           ) (   
         _.' '._ 
        `"""""""`"#;

    println!("{}", Style::new().yellow().bold().apply_to(trophy));
    println!("{}", Style::new().magenta().bold().apply_to(format!("🏆 {} reigns supreme! 🏆\n", victor)));
}

/// Prints a clear defeat header, a taunt, and then the victory banner for the winner.
fn print_defeat() {
    // Defeat header
    println!("{}", Style::new().bold().red().underlined().apply_to("============= DEFEAT ============="));

    // Random taunt in bold red
    let taunt = vec![
        "You noob! Too easy!",
        "Is that all you've got? Ha!",
        "Learn to fight, weakling!",
        "Come back when you're stronger!",
    ];
    
    let mut rng = thread_rng();
    let message = taunt[rng.gen_range(0..taunt.len())];

    let border = "─".repeat(message.len() + 4);
    let border_style = Style::new().red().bold();
    let text_style   = Style::new().red().italic().bold();

    println!("{}", border_style.apply_to(&border));
    println!("{}", text_style.apply_to(format!("│ {} │", message)));
    println!("{}", border_style.apply_to(&border));

}
