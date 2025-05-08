use rand::Rng;
use std::io;

#[derive(Debug)]
enum DamageType {
    FIRE,
    ICE,
    ELEC,
    SLASHING,
    PIERCING,
    BLUDGEONING,
}

#[derive(Debug)]
struct Character {
    name: String,
    level: u8,
    powers: Vec<Power>,
    hp: f64,
    mp: f64,
}

#[derive(Debug)]
struct StatusEffect {
    duration: u8,
    tags: Vec<String>,
    effect: fn(&mut Character),
}

impl StatusEffect {
    const TAG_STONED: &str = "TURNED_TO_STONE";
}

#[derive(Debug)]
struct Power {
    name: String,
    damage: f64,
    damage_type: DamageType,
    effect: fn(&mut Power, &mut Character),
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
            sum += rand::thread_rng().gen_range(1..=die_sides) as f64;
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
                effect: |power: &mut Power, character: &mut Character| {
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
    'main_battle_loop: loop {
        println!("Choose your weapon, one of {:#?}", combatants[0].powers);
        let mut player_attack = String::new();
        let byte_count = match io::stdin().read_line(&mut player_attack) {
            Ok(byte_count) => byte_count,
            Err(e) => {
                println!("Error reading stdin: {e}.");
                continue;
            }
        };
        player_attack = player_attack.trim().to_string().to_lowercase();
        // todo: run through the vec of powers and match against their names; how can I do that nicely?
        /* I'd like to do some kinda combo of match and iter like this, but I don't think that's legal.
        let picked_power: &String = match player_attack {
            combatants[0].powers.find_if([](elem){elem.name == player_attack}) => &relevant_power.name,
        }
        */

        // so this approach works, but seems suboptimal since I allocate a new string instead of just getting a reference to the extant power name I wanted to find. Not sure how to tease that ref out if null refs aren't a thing and I later need to read the ref outside the iter scope. I could just find the right index in the vector and use that to look up a ref to its name later, but that's both unclear and hacky.
        let mut picked_power = String::new();
        for power in combatants[0].powers.iter() {
            if power.name == player_attack {
                picked_power = power.name.clone();
                break;
            }
        }
        if picked_power.is_empty() {
            match player_attack.to_lowercase().as_str() {
                "quit" | "exit" | "q" => {
                    println!("ok thx bye ily!");
                    break 'main_battle_loop;
                }
                _ => println!("{player_attack} is not within the player's power!"),
            }
        } else {
            println!("The player has chosen the mighty power of {picked_power}!");
        }
    }
}
