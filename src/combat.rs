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
    if enemy.health == 0 {
        println!("###########  {} has been defeated!  #######", enemy.name);
        false
    } else if player.health == 0 {
        println!("{} has been defeated", player.name);
        false
    } else {
        // Ask for the player's command.
        input!("Press the Enter key to attack!\n ");

        // Player attacks enemy.
        enemy.health -= player.attack_damage;
        println!(
            "{} used {} and inflicted {} damage on {}!",
            player.name, player.attack_name, player.attack_damage, enemy.name
        );
        // Enemy attacks player.
        player.health -= enemy.attack_damage;
        println!(
            "{} used {} and inflicted {} damage on {}!",
            enemy.name, enemy.attack_name, enemy.attack_damage, player.name
        );
        true
    }
}

/// Display the health of the player and the enemy.
pub fn display_health(player: &Entity, enemy: &Entity) {
    println!("{}'s current health: {}", player.name, player.health);
    println!("{}'s current heath: {}", enemy.name, enemy.health);
}
