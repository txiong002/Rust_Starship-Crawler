//! Library file for combat between enemies and the player'
//! 5/12/2023
//!
//! Included in this file are the player struct and the enemy struct.
//! The player has a name and it can move around the map. It can also attack enemies.  
//!
//! The enemy only has health and the ability to attack.
//!
//!
//! // For prompting user input
use prompted::input;

/// A player character.
#[derive(Debug, Clone)]
pub struct Player {
    /// The player's name.
    pub name: String,
    /// The player's health.
    pub health: usize,
    /// Attack damage - base is 10
    pub attack_damage: usize,
    /// Stat that determines how much the player can move in one turn
    pub movement: usize,
}

/// A simple enemy.
#[derive(Debug, Clone)]
pub struct Enemy {
    /// The enemy's health.
    pub health: usize,
    pub attack_damage: usize,
}

/// Player implementation
/// The player can move and attack.
impl Player {
    pub fn new_player(name: String, health: usize, attack_damage: usize, movement: usize) -> Self {
        // Return the player.
        Player {
            name,
            health,
            attack_damage,
            movement,
        }
    }

    //
}

/// Enemy implementation
impl Enemy {
    /// Create a new enemy with a certain health and attack damage value
    pub fn new_enemy(health: usize, attack_damage: usize) -> Self {
        // Return the enemy.
        Enemy {
            health,
            attack_damage,
        }
    }
}

/// Combat implementation: Player and enemy take turns attacking each other.
///
/// Returns a bool that determines whether the game continues or not.
///
/// `true` means the game continues.
///
/// `false` means the game ends.
pub fn face_off(player: &mut Player, enemy: &mut Enemy) -> bool {
    if enemy.health == 0 {
        println!("###########  Enemy has been defeated!  #######");
        false
    } else if player.health == 0 {
        println!("{} has been defeated", player.name);
        false
    } else {
        // Ask for the player's name.
        input!("Press any key to attack!\n ");
        enemy.health -= player.attack_damage;
        println!(
            "{} afflicted {} damage points to enemy",
            player.name, player.attack_damage
        );
        player.health -= enemy.attack_damage;
        println!(
            "Enemy has afflicted {} damage points to {}",
            enemy.attack_damage, player.name
        );
        true
    }
}

/// Display the health of the player and the enemy.
pub fn display_health(player: &Player, enemy: &Enemy) {
    println!("{}'s current health: {}", player.name, player.health);
    println!("Enemy's current heath: {}", enemy.health);
}
