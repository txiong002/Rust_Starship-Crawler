//! Library file for combat between enemies and the player'
//! 5/12/2023
//!
//!
//!
//!
//!
//!

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
    pub fn new_enemy(health: usize, attack_damage: usize) -> Self {
        // Return the enemy.
        Enemy {
            health,
            attack_damage,
        }
    }
}

// Combat implementation
pub fn face_off(player: &mut Player, enemy: &mut Enemy) -> bool {
    if enemy.health == 0 {
        println!("Enemy has been defeated!");
        false
    } else if player.health == 0 {
        println!("{} has been defeated", player.name);
        false
    } else {
        enemy.health -= player.attack_damage;
        player.health -= enemy.attack_damage;
        true
    }
}

pub fn display_health(player: &Player, enemy: &Enemy) {
    println!("{} current health: {}", player.name, player.health);
    println!("Enemies current heath: {}", enemy.health);
}
