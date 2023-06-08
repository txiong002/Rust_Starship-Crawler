//! Library file for combat between enemies and the player'
//! 5/12/2023
//!
//! Included in this file is an entity struct that can be used for a player or an enemy.
//! The entity struct has a name, a health value, a damage value, and a movement value.
//!
//!
//!
//! // For prompting user input
use prompted::input;

// Randomly generate numbers
// https://docs.rs/rand/latest/rand/trait.Rng.html#method.gen_range
use rand::{thread_rng, Rng};

// ========================================================================================

// ========================================================================================
/// A moveable entity. It can be a player or an enemy.
///
/// Each entity has a name, a health bar, a base attack damage, and a base movement value.
#[derive(Debug, Clone)]
pub struct Entity {
    /// The emtity's name.
    pub name: String,
    /// The entity's health.
    pub health: usize,
    /// Attack name.
    pub attack_name: String,
    /// Attack damage - base is 10
    pub attack_damage: usize,
    /// Stat that determines how much the entity can move in one turn. Default is 1 tile.
    pub movement: usize,
}

/// Entity implementation.
///
///
impl Entity {
    /// Create a new player.
    pub fn new_player(
        name: String,
        health: usize,
        attack_name: String,
        attack_damage: usize,
        movement: usize,
    ) -> Self {
        // Return the player.
        Entity {
            name,
            health,
            attack_name,
            attack_damage,
            movement,
        }
    }

    /// Create a new enemy. This is the same as the new_player function.
    pub fn new_enemy(
        name: String,
        health: usize,
        attack_name: String,
        attack_damage: usize,
        movement: usize,
    ) -> Self {
        // Return the enemy.
        Entity {
            name,
            health,
            attack_name,
            attack_damage,
            movement,
        }
    }

    //
}

// ========================================================================================

// ========================================================================================

/// Combat implementation: Player and enemy take turns attacking each other.
///
/// Returns a bool that determines whether the game continues or not.
///
/// `true` means the game continues.
///
/// `false` means the game ends.
pub fn face_off(player: &mut Entity, enemy: &mut Entity) -> bool {
    if enemy.health == 0 && player.health == 0 {
        println!("Double KO, there was no winner!");
        false
    } else if enemy.health == 0 {
        println!("###########  {} has been defeated!  #######", enemy.name);
        false
    } else if player.health == 0 {
        println!("{} has been defeated", player.name);
        false
    } else {
        // Ask for the player's command.
        input!("Press the Enter key to attack!\n ");

        // Flip a coin to determine whether the player's attack lands or misses.
        let mut rng = thread_rng();
        let coin_flip_player: usize = rng.gen_range(0..=1);

        if coin_flip_player == 0 {
            // Player misses
            println!("{} used {} but missed!", player.name, player.attack_name);
        } else {
            // Determine if the player lands a critical hit.
            // A critical hit does twice as much damage as a normal hit.
            // 0 to 6 = no critical hit.
            // 7 to 9 = critical hit.
            let crit_hit_value: usize = rng.gen_range(0..=9);

            // Player attacks enemy.
            if enemy.health >= player.attack_damage {
                if crit_hit_value > 6 {
                    println!("** Player landed a critical hit! **");
                    if enemy.health >= (2 * player.attack_damage) {
                        enemy.health -= 2 * player.attack_damage
                    } else {
                        enemy.health = 0
                    }
                } else {
                    enemy.health -= player.attack_damage;
                }
            } else {
                // avoid overflow
                enemy.health = 0
            }

            // If the player lands a critical hit, show the message and updated attack damage
            if crit_hit_value > 6 {
                println!(
                    "{} used {} and inflicted {} damage on {}!",
                    player.name, player.attack_name, 2 * player.attack_damage, enemy.name
                );
            } else {
                println!(
                    "{} used {} and inflicted {} damage on {}!",
                    player.name, player.attack_name, player.attack_damage, enemy.name
                );
            }
        }

        // Flip a coin to determine whether the enemy's attack lands or misses.
        let coin_flip_enemy: usize = rng.gen_range(0..=1);

        if coin_flip_enemy == 0 {
            // Enemy misses
            println!("{} used {} but missed!", enemy.name, enemy.attack_name);
        } else {
            // Enemy attacks player.
            if player.health >= enemy.attack_damage {
                player.health -= enemy.attack_damage;
            } else {
                player.health = 0;
            }
            println!(
                "{} used {} and inflicted {} damage on {}!",
                enemy.name, enemy.attack_name, enemy.attack_damage, player.name
            );
        }

        true
    }
}

/// Display the health of the player and the enemy.
pub fn display_health(player: &Entity, enemy: &Entity) {
    println!("{}'s current health: {}", player.name, player.health);
    println!("{}'s current heath: {}", enemy.name, enemy.health);
}
