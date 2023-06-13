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
/// An attack usable by any entity.
/// An attack can have a name and a damage value.
#[derive(Debug, Clone)]
pub struct Attack {
    /// The attack's name.
    pub name: String,
    /// The attack's damage value.
    pub damage_value: usize,
}

impl Attack {
    pub fn new_attack(name: String, damage_value: usize) -> Self {
        Attack { name, damage_value }
    }
}

// ========================================================================================
/// A moveable entity. It can be a player or an enemy.
///
/// Each entity has a name, a health bar, a base attack damage, and a base movement value.
#[derive(Debug, Clone)]
pub struct Entity {
    /// The entity's name.
    pub name: String,
    /// The entity's health.
    pub health: usize,
    /// Attacks the entity can use. Each attack has a name and a damage value. For now the player has only one attack.
    pub attacks: Vec<Attack>,
    // /// Attack name.
    // pub attack_name: String,
    // /// Attack damage - base is 10
    // pub attack_damage: usize,
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
        attacks: Vec<Attack>,
        // attack_name: String,
        // attack_damage: usize,
        movement: usize,
    ) -> Self {
        // Return the player.
        Entity {
            name,
            health,
            attacks,
            movement,
        }
    }

    /// Create a new enemy. This is the same as the new_player function.
    pub fn new_enemy(
        name: String,
        health: usize,
        attacks: Vec<Attack>,
        // attack_name: String,
        // attack_damage: usize,
        movement: usize,
    ) -> Self {
        // Return the enemy.
        Entity {
            name,
            health,
            attacks,
            movement,
        }
    }

    //
}

// ========================================================================================

// ========================================================================================

/// Combat implementation: Player and enemy take turns attacking each other.
///
/// The enemy will randomly select an attack.
///
/// Returns a bool that determines whether the game continues or not.
///
/// `true` means the game continues.
///
/// `false` means the game ends.
pub fn face_off(player: &mut Entity, enemy: &mut Entity) -> bool {
    // Both player and enemy defeat each other.
    if enemy.health == 0 && player.health == 0 {
        println!("Double KO, there was no winner!");
        false
    // Enemy was defeated. Player has at least 1 HP.
    } else if enemy.health == 0 {
        println!("###########  {} has been defeated!  #######", enemy.name);
        false
    // Player was defeated and enemy has at least 1 HP.
    } else if player.health == 0 {
        println!("{} has been defeated", player.name);
        false
    // Player and enemy have at least 1 HP and take turns attacking.
    } else {
        // ====================================================================================================
        // PLAYER ATTACKS ENEMY

        // Ask for the player's command.
        input!("Press the Enter key to attack!\n ");

        // Flip a coin to determine whether the player's attack lands or misses.
        let mut rng = thread_rng();
        let coin_flip_player: usize = rng.gen_range(0..=1);

        if coin_flip_player == 0 {
            // Player misses
            println!(
                "{} used {} but missed!",
                player.name, player.attacks[0].name
            );
        } else {
            // Determine if the player lands a critical hit.
            // A critical hit does twice as much damage as a normal hit.
            // 0 to 6 = no critical hit.
            // 7 to 9 = critical hit.
            let crit_hit_value: usize = rng.gen_range(0..=9);

            // Player attacks enemy.
            if enemy.health >= player.attacks[0].damage_value {
                if crit_hit_value > 6 {
                    println!("** Player landed a critical hit! **");
                    if enemy.health >= (2 * player.attacks[0].damage_value) {
                        enemy.health -= 2 * player.attacks[0].damage_value
                    } else {
                        enemy.health = 0
                    }
                } else {
                    enemy.health -= player.attacks[0].damage_value;
                }
            } else {
                // avoid overflow
                enemy.health = 0
            }

            // If the player lands a critical hit, show the message and updated attack damage
            if crit_hit_value > 6 {
                println!(
                    "{} used {} and inflicted {} damage on {}!",
                    player.name,
                    player.attacks[0].name,
                    2 * player.attacks[0].damage_value,
                    enemy.name
                );
            } else {
                println!(
                    "{} used {} and inflicted {} damage on {}!",
                    player.name, player.attacks[0].name, player.attacks[0].damage_value, enemy.name
                );
            }
        }

        // ====================================================================================================
        // ENEMY ATTACKS PLAYER

        // Determine which attack the enemy should use if it has multiple attacks.
        let enemy_attack_name: String;
        let enemy_attack_damage_value: usize;
        if enemy.attacks.len() > 1 {
            // Randomly select an attack.
            let attack_index: usize = rng.gen_range(0..enemy.attacks.len());

            enemy_attack_name = enemy.attacks[attack_index].name.clone();
            enemy_attack_damage_value = enemy.attacks[attack_index].damage_value;
        } else {
            // Use the attack given.
            enemy_attack_name = enemy.attacks[0].name.clone();
            enemy_attack_damage_value = enemy.attacks[0].damage_value;
        }

        // Flip a coin to determine whether the enemy's attack lands or misses.
        let coin_flip_enemy: usize = rng.gen_range(0..=1);

        if coin_flip_enemy == 0 {
            // Enemy misses
            println!("{} used {} but missed!", enemy.name, enemy_attack_name);
        } else {
            // Enemy attacks player.
            if player.health >= enemy_attack_damage_value {
                player.health -= enemy_attack_damage_value;
            } else {
                player.health = 0;
            }
            println!(
                "{} used {} and inflicted {} damage on {}!",
                enemy.name, enemy_attack_name, enemy_attack_damage_value, player.name
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

// function to restore player health. Generate number between 0 and 4, then applies health based on number generated
pub fn regenerate_health(player: &mut Entity) {
    let restore: u32 = rand::thread_rng().gen_range(0..4);
    match restore {
        0 => player.health += 20,
        1 => player.health += 25,
        2 => player.health += 30,
        3 => player.health = 75,
        4 => player.health = 100,
        5_u32..=u32::MAX => todo!(),
    }
}

// ==============================================================================
/// Test that the player's health is regenerated correctly.
#[test]
fn test_regenerate_health() {
    // Test that the player, starting with 30 health, has at least 50 health
    let mut player: Entity = Entity::new_player(
        "Test Player".to_string(),
        30, // Base health is 30
        vec![Attack::new_attack("Scattershot".to_string(), 10)],
        1, // starting movement range is 1 tile
    );

    // Regenerate the player's health based on a random number.
    regenerate_health(&mut player);

    // Assert that the player's health is now greater than the previous value.
    println!("Player health is now {}", player.health);
    assert!(player.health > 30);
}
