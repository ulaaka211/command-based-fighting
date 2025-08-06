use rand::{thread_rng, Rng};
use console::Style;
use crate::actor::{Actor, Actions};

/// Decide and perform the dragon's turn with simple heuristics.
pub fn take_turn(dragon: &mut Actor, player: &mut Actor, action_menu: &[&str]) {
    let mut rng = thread_rng();

    // Compute health ratios
    let hp_ratio = dragon.hp as f32 / dragon.max_hp as f32;
    let player_hp_ratio = player.hp as f32 / player.max_hp as f32;

    // Heuristic priorities:
    // 1. If dragon's HP is low and can heal, cast heal.
    // 2. If player's HP is low and dragon has stamina, perform heavy attack.
    // 3. If out of stamina, defend to recover.
    // 4. If enough resources for special and chance, use special skill.
    // 5. Otherwise, perform normal attack.
    let choice = if hp_ratio < 0.3 && dragon.mana >= 2 {
        3 // Heal
    } else if player_hp_ratio < 0.25 && dragon.stamina >= 2 {
        1 // Heavy attack
    } else if dragon.stamina == 0 {
        2 // Defend
    } else if dragon.mana >= 3 && dragon.stamina >= 2 && rng.gen_bool(0.5) {
        5 // Special Skill
    } else {
        0 // Attack
    };

    // Display choice in color
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

    println!("Dragon chooses: {}", colored);

    // Execute the chosen action
    match choice {
        0 => dragon.attack(player),
        1 => dragon.heavy_attack(player),
        2 => dragon.defend(),
        3 => dragon.heal(),
        4 => dragon.dodge(),
        5 => dragon.special_skill(player),
        _ => dragon.attack(player),
    }
}
