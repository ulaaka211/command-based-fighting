use rand::Rng;
use std::io::{self, Write};

struct Character {
    name: String,
    hp: i32,
    max_hp: i32,
}

impl Character {
    fn new(name: &str, max_hp: i32) -> Self {
        Self { name: name.into(), hp: max_hp, max_hp }
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn take_damage(&mut self, dmg: i32) {
        self.hp = (self.hp - dmg).max(0);
        println!("{} takes {} damage (HP: {}/{})", self.name, dmg, self.hp, self.max_hp);
    }

    fn heal(&mut self, amount: i32) {
        let healed = amount.min(self.max_hp - self.hp);
        self.hp += healed;
        println!("{} heals {} HP (HP: {}/{})", self.name, healed, self.hp, self.max_hp);
    }
}

enum Action {
    Attack,
    Heal,
    Defend,
    Dodge,
    Special,
}

fn read_player_action() -> Action {
    loop {
        print!("\nChoose your action:\n\
                1) Attack   2) Heal   3) Defend\n\
                4) Dodge    5) Special\n> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => return Action::Attack,
            "2" => return Action::Heal,
            "3" => return Action::Defend,
            "4" => return Action::Dodge,
            "5" => return Action::Special,
            _ => println!("Invalid choice, try again."),
        }
    }
}

fn enemy_action() -> Action {
    match rand::thread_rng().gen_range(1..=5) {
        1 => Action::Attack,
        2 => Action::Heal,
        3 => Action::Defend,
        4 => Action::Dodge,
        _ => Action::Special,
    }
}

fn main() {
    let mut player = Character::new("You", 100);
    let mut enemy  = Character::new("CPU",  100);
    let mut player_defending = false;
    let mut enemy_defending  = false;

    println!("=== Turn-Based Combat ===");

    while player.is_alive() && enemy.is_alive() {
        // Player turn
        let action = read_player_action();
        player_defending = false; // reset flag each turn

        match action {
            Action::Attack => {
                let dmg = 15;
                let actual = if enemy_defending {
                    (dmg / 2).max(1)
                } else { dmg };
                enemy.take_damage(actual);
            }
            Action::Heal => {
                player.heal(20);
            }
            Action::Defend => {
                println!("You brace for incoming attacks.");
                player_defending = true;
            }
            Action::Dodge => {
                println!("You prepare to dodge the next attack.");
                // dodge logic applied in enemy turn
            }
            Action::Special => {
                if rand::thread_rng().gen_bool(0.5) {
                    let dmg = 30;
                    println!("Special skill hits for {} damage!", dmg);
                    enemy.take_damage(dmg);
                } else {
                    println!("Special skill missed!");
                }
            }
        }

        if !enemy.is_alive() { break; }

        // Enemy turn
        let e_action = enemy_action();
        enemy_defending = false;
        println!("\n-- CPU's turn --");

        match e_action {
            Action::Attack => {
                let dmg = 12;
                // Check player dodge
                if let Action::Dodge = action {
                    if rand::thread_rng().gen_bool(0.5) {
                        println!("You dodged the CPU's attack!");
                        continue;
                    }
                }
                let actual = if player_defending { (dmg / 2).max(1) } else { dmg };
                player.take_damage(actual);
            }
            Action::Heal => {
                enemy.heal(15);
            }
            Action::Defend => {
                println!("CPU is defending.");
                enemy_defending = true;
            }
            Action::Dodge => {
                println!("CPU prepares to dodge.");
            }
            Action::Special => {
                if rand::thread_rng().gen_bool(0.4) {
                    let dmg = 25;
                    println!("CPU uses special for {} damage!", dmg);
                    player.take_damage(dmg);
                } else {
                    println!("CPU's special missed!");
                }
            }
        }
    }

    // Game over
    if player.is_alive() {
        println!("\n🎉 You win!");
    } else {
        println!("\n💀 You were defeated...");
    }
}
