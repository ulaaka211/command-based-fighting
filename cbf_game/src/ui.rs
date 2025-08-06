use console::Style;
use dialoguer::Select;
use crate::actor::Actor;

fn bar(current: i32, max: i32, width: usize, style: Style, fill_char: char) -> String {
    let ratio = (current as f32 / max as f32).clamp(0.0, 1.0);
    let filled = (ratio * width as f32).round().min(width as f32) as usize;
    let bar_str = fill_char.to_string().repeat(filled) + &" ".repeat(width - filled);
    style.apply_to(&bar_str).to_string()
}

pub fn display_stats(actor: &Actor) {
    println!("{}", Style::new().bold().underlined().apply_to(&actor.name));

    println!(
        "  HP      [{}] {}/{}",
        bar(actor.hp, actor.max_hp, 20, Style::new().red(), '█'),
        actor.hp,
        actor.max_hp
    );
    println!(
        "  Mana    [{}] {}/{}",
        bar(actor.mana, actor.max_mana, 10, Style::new().blue(), '▄'),
        actor.mana,
        actor.max_mana
    );
    let defend_flag = if actor.is_defending {
        Style::new().cyan().apply_to(" [DEF]").to_string()
    } else {
        String::new()
    };
    println!(
        "  Stamina [{}] {}/{}{}",
        bar(actor.stamina, actor.max_stamina, 10, Style::new().yellow(), '▄'),
        actor.stamina,
        actor.max_stamina,
        defend_flag
    );

    println!();
}

pub fn prompt_action(menu: &[&str]) -> usize {
    let prompt = Style::new().bold().apply_to("Choose your action:").to_string();
    Select::new()
        .with_prompt(prompt)
        .items(menu)
        .default(0)
        .interact()
        .unwrap()
}
