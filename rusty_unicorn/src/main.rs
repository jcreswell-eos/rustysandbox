use rand::Rng;
use std::io;

enum DamageType {
    FIRE,
    ICE,
    ELEC,
    SLASHING,
    PIERCING,
    BLUDGEONING,
}

struct Character {
    name: String,
    level: u8,
    powers: Vec<Power>,
    hp: f64,
    mp: f64,
}

struct StatusEffect {
    duration: u8,
    tags: Vec<String>,
    effect: fn(&mut Character),
}

impl StatusEffect {
    const TAG_STONED: &str = "TURNED_TO_STONE";
}

struct Power {
    name: String,
    damage: f64,
    damage_type: DamageType,
    effect: fn(&Power, &mut Character),
}

impl Power {
    /**
     * Simulates rolling dice_count die_sides sided dice.
     * @param dice_count: the number of dice to roll
     * @param die_sides: the number of sides each die has
     * @return the sum of all rolled dice
     */
    fn rollNdM(dice_count: u8, die_sides: u8) -> f64 {
        let mut sum: f64 = 0.0;
        for x in 1..dice_count {
            sum += rand::thread_rng().gen_range(1..=die_sides);
        }
        sum
    }
}

fn main() {
    println!("Let's get ready to ruuuuuuumble!");
    let mut player = Character {
        level: 5,
        name: String::from("Our Fearless Hero"),
        powers: vec![
            Power {
                name: String::from("fireball"),
                damage: 0.0,
                damage_type: DamageType::FIRE,
                effect: |power: &Power, character: &mut Character| {
                    power.damage = Power::rollNdM(character.level, 6);
                    character.hp -= power.damage;
                },
            },
            //String::from("sword"),
            //String::from("shield"),
        ],
        hp: 100.0,
        mp: 100.0,
    };
    let mut monster = Character {
        level: 5,
        name: String::from("Snort, The Unicron"),
        powers: vec![
            //String::from("hoof stomp"),
            //String::from("horn stab"),
            //String::from("stone breath"),
        ],
        hp: 1000.0,
        mp: 100.0,
    };
    let combatants = vec![player, monster];
    battle_loop(&combatants);
}

fn battle_loop(combatants: &Vec<Character>) {
    loop {
        println!("Choose your weapon, one of {:#?}", combatants[0].powers);
        let mut player_attack = String::new();
        let byte_count = match io::stdin().read_line(&mut player_attack) {
            Ok(byte_count) => byte_count,
            Err(e) => {
                println!("Error reading stdin: {e}.");
                continue;
            }
        };
        player_attack = player_attack.trim().to_string();
        match player_attack.as_str() {
            "fireball" | "sword" | "shield" => println!("The player chooses {player_attack}"),
            _ => println!("{player_attack} is not within the player's power!"),
        }
    }
}
