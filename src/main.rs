//! Main file
//! 4/28/2023
//! CS 510 Rust
//!
//! This file is used to execute the game.
//!
//! Run it using the command `cargo run`.
//!
//!

// Imports ========================================
// For prompting user input
use prompted::input;

// Using rust modules and libraries
use rsc_lib::{
    combat::Player,
    combat::{display_health, face_off, Enemy},
    *,
};

/// The player's health
const MAX_PLAYER_HEALTH: usize = 100;
/// The player's base attack damage.
const BASE_ATTACK_DAMAGE: usize = 10;
/// The enemy's base attack damage.
const ENEMY_ATTACK_DAMAGE: usize = 9;
/// The number of tiles the player is allowed to traverse.
const BASE_MOVEMENT: usize = 1;

/// Display the room
fn show_room(room: &Room) {
    // Show the room
    for i in 0..room.width {
        for j in 0..room.height {
            //Print player location
            if i == room.player_location.0 && j == room.player_location.1 {
                print!("^");
            }
            //  print enemy location
            else if i == room.enemy_location.0 && j == room.enemy_location.1 {
                let is_found = found_enemy(room.clone());
                if is_found {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            // Print a room tile depending on whether it is a wall or a floor.
            else if room.room_area[i][j] {
                // Floor
                print!(".");
            } else {
                // Wall
                print!("#");
            }
        }
        // Go to the next row
        println!();
    }
}

fn main() {
    println!("==== Welcome to Starship Crawler! ====");
    println!();
    println!("BACKSTORY: You are an explorer assigned to investigate an abandoned cargo ship that was lost in space. The cargo ship was last seen departing a tropical planet with some treasure on board. There are rumors of strange and terrifying monstrosities lurking within the cargo bays and engineering rooms, but that won't stop you from finding out the starship's secrets, won't it?");
    println!();
    println!("You crack open the starship's charred cargo bay door and make your way inside the first room. There is a low growling sound from across the hall. You draw your weapon and scan the area.");
    println!();
    println!("Are you ready to discover the secrets and treasures within?");
    println!();

    // Ask for the player's name.
    let player_name: String = input!("Enter your name: ");

    println!();

    // Create new player
    let mut player: Player = Player::new_player(
        player_name,
        MAX_PLAYER_HEALTH,
        BASE_ATTACK_DAMAGE,
        BASE_MOVEMENT,
    );

    // Create new enemy
    let mut enemy: Enemy = Enemy::new_enemy(MAX_PLAYER_HEALTH, ENEMY_ATTACK_DAMAGE);

    let mut room = Room::new(10, 10);

    // println!("{:?}", room);

    show_room(&room);
    println!(
        "You are in square ({}, {}).",
        room.player_location.0, room.player_location.1
    );

    println!("Your name is: {}", player.name);
    println!("Your have {} health.", player.health);

    // Game loop logic - end the game when the player wins or the player dies.
    // Loop until game is over
    'outer: loop {
        println!("You can move {} space.", player.movement);
        let mut check = 0;
        while check == 0 {
            //Get user move
            room = match user_move(room.clone(), &player) {
                Some(room) => {
                    check = 1;
                    room
                }
                //Continue if move was invalid
                None => continue,
            };
        }
        //Reprint room
        show_room(&room);
        println!(
            "You are in square ({}, {}).",
            room.player_location.0, room.player_location.1
        );

        // Check if the player has found an enemy. If so, the player and the enemy will fight.
        // Currently the player has no control over when they get to make a move.
        let is_found = found_enemy(room.clone());
        if is_found {
            println!("\nGET READY TO BATTLE!!!!");
            loop {
                display_health(&player, &enemy); // Show the health values
                let game_over = face_off(&mut player, &mut enemy); // Player and enemy attack each other
                if !game_over {
                    // If either the enemy or the player has lost all their health, the game ends.
                    println!("Game over, Thanks for playing");
                    break 'outer;
                }
            }
        } else {
            continue;
        }
    }
}
