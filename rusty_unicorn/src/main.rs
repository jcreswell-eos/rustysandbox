use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::io;

#[derive(Debug, Clone)]
enum DamageType {
    FIRE,
    ICE,
    ELEC,
    SLASHING,
    PIERCING,
    BLUDGEONING,
    NONE,
}

struct Character {
    name: String,
    powers: Vec<Power>,
    stats: Stats,
}

impl Character {
    fn pretty_powers(&self) -> String {
        let mut powers_combo = String::new();
        for power in self.powers.iter() {
            powers_combo.push_str(&format!("{}", power));
            powers_combo.push_str("\n");
        }
        powers_combo
    }
}

impl fmt::Debug for Character {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the name into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "{}\n{}\n{:#?}",
            self.name,
            self.pretty_powers(),
            self.stats
        )
    }
}

impl fmt::Display for Character {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the name into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}\nHP: {}", self.name, self.stats.hp)
    }
}

#[derive(Debug)]
struct Stats {
    level: u8,
    max_hp: f64,
    hp: f64,
    max_mp: f64,
    mp: f64,
    strength: f64,
    defense: f64,
    will: f64,
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

#[derive(Debug, Clone)]
struct Power {
    name: String,
    damage: f64,
    cost: f64,
    damage_type: DamageType,
    effect: fn(power: &mut Power, source: &mut Character, destination: &mut Character),
}

impl Power {
    /**
     * Simulates rolling dice_count die_sides sided dice.
     * @param dice_count: the number of dice to roll
     * @param die_sides: the number of sides each die has
     * @return the sum of all rolled dice
     */
    fn roll_dice(dice_count: u8, die_sides: u8) -> f64 {
        let mut sum: f64 = 0.0;
        for _ in 1..dice_count {
            sum += rand::thread_rng().gen_range(1..=die_sides) as f64;
        }
        sum
    }
}

impl Default for Power {
    fn default() -> Self {
        Power {
            name: String::from(""),
            cost: 0.0,
            damage: 0.0,
            damage_type: DamageType::NONE,
            effect: |power: &mut Power, source: &mut Character, destination: &mut Character| {},
        }
    }
}

impl fmt::Display for Power {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the name into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.name)
    }
}

fn main() {
    println!("Let's get ready to ruuuuuuumble!");
    let mut player = Character {
        name: String::from("Our Fearless Hero"),
        stats: Stats {
            level: 5,
            max_hp: 100.0,
            hp: 100.0,
            max_mp: 100.0,
            mp: 100.0,
            strength: 15.0,
            defense: 10.0,
            will: 25.0,
        },
        powers: vec![
            Power {
                name: String::from("fireball"),
                damage: 0.0,
                cost: 25.0,
                damage_type: DamageType::FIRE,
                effect: |power: &mut Power, source: &mut Character, destination: &mut Character| {
                    power.damage = Power::roll_dice(source.stats.level, 6) + source.stats.will;
                    destination.stats.hp -= power.damage - 0.1 * destination.stats.will;
                    source.stats.mp -= power.cost;
                },
            },
            Power {
                name: String::from("sword"),
                damage: 0.0,
                cost: 0.0,
                damage_type: DamageType::SLASHING,
                effect: |power: &mut Power, source: &mut Character, destination: &mut Character| {
                    power.damage = Power::roll_dice(2, 6) + source.stats.strength;
                    destination.stats.hp -= power.damage - 0.5 * destination.stats.defense;
                },
            },
            Power {
                name: String::from("shield"),
                damage: 0.0,
                cost: 0.0,
                damage_type: DamageType::NONE,
                effect: |power: &mut Power, source: &mut Character, destination: &mut Character| {
                    // todo: apply a shielded status effect to source character, which doubles defense and recovers a little HP and MP.
                },
            },
        ],
    };
    let mut monster = Character {
        name: String::from("Snort, The Unicron"),
        stats: Stats {
            level: 7,
            max_hp: 250.0,
            hp: 250.0,
            max_mp: 100.0,
            mp: 100.0,
            strength: 50.0,
            defense: 10.0,
            will: 5.0,
        },
        powers: vec![
            Power {
                name: String::from("hoof stomp"),
                damage: 0.0,
                cost: 0.0,
                damage_type: DamageType::BLUDGEONING,
                effect: |power: &mut Power, source: &mut Character, destination: &mut Character| {
                    print!("a terrible hoof strikes you, then the earth, then you again!");
                    power.damage = Power::roll_dice(2, 20) + 0.5 * source.stats.strength;
                    destination.stats.hp -= power.damage - 0.1 * destination.stats.defense;
                },
            },
            Power {
                name: String::from("horn stab"),
                damage: 0.0,
                cost: 0.0,
                damage_type: DamageType::PIERCING,
                effect: |power: &mut Power, source: &mut Character, destination: &mut Character| {
                    power.damage = Power::roll_dice(3, 6) + 0.25 * source.stats.strength;
                    destination.stats.hp -= power.damage;
                    // todo: inflict bleeding status effect
                },
            },
            Power {
                name: String::from("stone breath"),
                damage: 0.0,
                cost: 25.0,
                damage_type: DamageType::NONE,
                effect: |power: &mut Power, source: &mut Character, destination: &mut Character| {
                    power.damage = Power::roll_dice(source.stats.level, 8)
                        - (source.stats.max_hp - source.stats.hp);
                    destination.stats.hp -= power.damage - 0.2 * destination.stats.will;
                    source.stats.mp -= power.cost;
                    // todo: roll chance to inflict stone status effect
                },
            },
        ],
    };
    // todo: does passing in the player and monster via move instead of borrowing result in a reallocation or is it just a transfer of ownership?
    let mut combatants = HashMap::from([("player", &mut player), ("monster", &mut monster)]);
    // todo: what happens if we pass in the hashmap via move instead of borrowing? Does a reallocation occur or is it just that we could no longer use the combatants variable in this scope?
    battle_loop(&mut combatants); // apparently the hashmap needs o be mutable even to mod the values stored in it? That's dumb af.
}

fn battle_loop(combatants: &mut HashMap<&str, &mut Character>) {
    'main_battle_loop: loop {
        println!(
            "\nChoose your weapon, one of: \n{}",
            combatants["player"].pretty_powers()
        );
        let mut player_attack = String::new();
        match io::stdin().read_line(&mut player_attack) {
            Ok(byte_count) => byte_count,
            Err(e) => {
                println!("Error reading stdin: {e}.");
                continue;
            }
        };
        player_attack = player_attack.trim().to_string().to_lowercase();

        let mut picked_power = Power::default();
        for power in combatants["player"].powers.iter() {
            if power.name == player_attack {
                picked_power = power.clone();
                break;
            }
        }
        if picked_power.name.is_empty() {
            match player_attack.to_lowercase().as_str() {
                "quit" | "exit" | "q" => {
                    println!("ok thx bye ily!");
                    break 'main_battle_loop;
                }
                _ => println!("{player_attack} is not within the player's power!"),
            }
        } else {
            println!("\nThe player has chosen the mighty power of {picked_power}!");
            let [Some(player), Some(monster)] = combatants.get_disjoint_mut(["player", "monster"])
            else {
                panic!("expected combatants not found in map!")
            };
            (picked_power.effect)(&mut picked_power, player, monster);
            println!("\nThe noxious uncihorn narrows its beady glowing red eyes...");
            let mut monster_power = ai_pick_power(player, monster);
            (monster_power.effect)(&mut monster_power, monster, player);
        }
        println!("\nCombatants, report!");
        for combatant in combatants.keys() {
            println!("{}", combatants[combatant]);
        }

        // endgame state
        if combatants["player"].stats.hp <= 0.0 && combatants["monster"].stats.hp <= 0.0 {
            println!(
                "The planet groans with the shifting fabric of the spacetime continuum as ye titans two fall simultaneously to each other's apocalyptic fury!"
            );
            println!("You have died! Also your foe, but also you!");
            break;
        } else if combatants["player"].stats.hp <= 0.0 {
            println!(
                "The unicorn smiles horseyly as it prepares to gnaw the remaining gristle from your splintery-charred bones."
            );
            println!("You have died!");
            break;
        } else if combatants["monster"].stats.hp <= 0.0 {
            println!(
                "The unicorn lets out a terrible neighing howl as it dissolved into the hellfire whence it came!"
            );
            println!("You stand triumphant!");
            break;
        }
    }
}

/*
fn ai_pick_power<'a>(player: &'a mut Character, ai: &'a mut Character) -> &'a mut Power {
    &mut ai.powers[0]
}
*/
fn ai_pick_power(player: &mut Character, ai: &mut Character) -> Power {
    ai.powers[0].clone()
}
