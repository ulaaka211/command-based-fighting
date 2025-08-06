use std::cmp::{max};
use console::Style;

#[derive(Debug, Clone)]
pub struct Actor {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub stamina: i32,
    pub max_stamina: i32,
    pub attack: i32,
    pub heavy_attack: i32,
    pub heal_power: i32,
    pub special_skill_power: i32,
    // pub dodge_chance: f32,
    pub physical_resistance: f32,
    pub magic_resistance: f32,
    pub accuracy: u8,
    pub is_defending: bool,
    pub is_dodging: bool,
}

impl Actor {
    pub fn new(name: impl Into<String>) -> Self {
        Actor {
            name: name.into(),
            hp: 100,
            max_hp: 100,
            mana: 5,
            max_mana: 5,
            stamina: 5,
            max_stamina: 5,
            attack: 10,
            heavy_attack: 20,
            heal_power: 10,
            special_skill_power: 30,
            // dodge_chance: 0.2,
            physical_resistance: 0.3,
            magic_resistance: 0.1,
            accuracy: 3,
            is_defending: false,
            is_dodging: false,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
}

pub trait Actions {
    fn attack(&mut self, target: &mut Actor);
    fn heavy_attack(&mut self, target: &mut Actor);
    fn defend(&mut self);
    fn heal(&mut self);
    fn dodge(&mut self);
    fn special_skill(&mut self, target: &mut Actor);
}

impl Actions for Actor {
    fn attack(&mut self, target: &mut Actor) {
        if self.stamina == 0 {
            println!("{}", Style::new().red().apply_to(format!("{} is too tired to attack!", self.name)));
            return;
        }
        self.stamina -= 1;

        if target.is_dodging {
            println!("{}", Style::new().cyan().apply_to(format!("{} dodges the attack!", target.name)));
            target.is_dodging = false;
            return;
        }
        if target.is_defending {
            println!("{}", Style::new().yellow().apply_to(format!("{} blocks the attack!", target.name)));
            target.is_defending = false;
            return;
        }

        let frac = (self.accuracy.min(5) as f32) / 5.0;
        if frac == 0.0 {
            println!("{}", Style::new().magenta().apply_to(format!("{}'s attack missed!", self.name)));
            return;
        }

        let raw = (self.attack as f32 * frac).round() as i32;
        let dmg = reduce_by_fraction(raw, target.physical_resistance);
        target.hp = max(target.hp - dmg, 0);

        println!(
            "{}",
            Style::new().green().apply_to(format!("{} → {} takes {} damage!", self.name, target.name, dmg))
        );
    }

    fn heavy_attack(&mut self, target: &mut Actor) {
        if self.stamina < 2 {
            println!("{}", Style::new().red().apply_to(format!("{} is too tired for a heavy attack!", self.name)));
            return;
        }
        self.stamina -= 2;

        if target.is_dodging {
            println!("{}", Style::new().cyan().apply_to(format!("{} dodges the heavy attack!", target.name)));
            target.is_dodging = false;
            return;
        }
        if target.is_defending {
            println!("{}", Style::new().yellow().apply_to(format!("{} blocks the heavy attack!", target.name)));
            target.is_defending = false;
            return;
        }

        let frac = (self.accuracy.min(5) as f32) / 5.0;
        if frac == 0.0 {
            println!("{}", Style::new().magenta().apply_to(format!("{}'s heavy attack missed!", self.name)));
            return;
        }

        let raw = (self.heavy_attack as f32 * frac).round() as i32;
        let dmg = reduce_by_fraction(raw, target.physical_resistance);
        target.hp = max(target.hp - dmg, 0);

        println!(
            "{}",
            Style::new().green().apply_to(format!("{} → {} lands HEAVY for {} damage!", self.name, target.name, dmg))
        );
    }

    fn defend(&mut self) {
        self.is_defending = true;
        self.stamina = (self.stamina + 1).min(self.max_stamina);
        println!("{}", Style::new().cyan().apply_to(format!("{} braces and recovers +1 Stamina.", self.name)));
    }

    fn heal(&mut self) {
        if self.mana < 2 {
            println!(
                "{}",
                Style::new()
                    .red()
                    .apply_to(format!("{} has insufficient Mana!", self.name))
            );
            return;
        }
    
        self.mana -= 2;
        // saturating_add will never overflow past i32::MAX
        let healed = self.hp.saturating_add(self.heal_power);
        // clamp to max_hp so you never go over it
        self.hp = healed.min(self.max_hp);
    
        println!(
            "{}",
            Style::new()
                .blue()
                .apply_to(format!("{} heals for {} HP!", self.name, self.heal_power))
        );
    }

    fn dodge(&mut self) {
        if self.stamina == 0 {
            println!("{}", Style::new().red().apply_to(format!("{} is too tired to dodge!", self.name)));
            return;
        }
        self.stamina -= 1;
        self.is_dodging = true;
        println!("{}", Style::new().cyan().apply_to(format!("{} is ready to dodge next attack!", self.name)));
    }

    fn special_skill(&mut self, target: &mut Actor) {
        let cost_m = 3;
        let cost_s = 2;
        if self.mana < cost_m || self.stamina < cost_s {
            println!("{}", Style::new().red().apply_to(format!("{} can't muster the special skill!", self.name)));
            return;
        }
        self.mana -= cost_m;
        self.stamina -= cost_s;

        if target.is_dodging {
            println!("{}", Style::new().cyan().apply_to(format!("{} dodges the special skill!", target.name)));
            target.is_dodging = false;
            return;
        }
        if target.is_defending {
            println!("{}", Style::new().yellow().apply_to(format!("{} blocks the special skill!", target.name)));
            target.is_defending = false;
            return;
        }

        let raw = self.special_skill_power;
        let dmg = reduce_by_fraction(raw, target.magic_resistance);
        target.hp = max(target.hp - dmg, 0);

        println!(
            "{}",
            Style::new().magenta().apply_to(format!("{} uses SPECIAL on {} for {} damage!", self.name, target.name, dmg))
        );
    }
}

fn reduce_by_fraction(val: i32, res: f32) -> i32 {
    ((val as f32) * (1.0 - res)).round() as i32
}
