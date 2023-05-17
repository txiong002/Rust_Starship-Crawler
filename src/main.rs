//! Main file
//! 4/28/2023
//! CS 510 Rust
//!
//! This file is used to execute the game.
//!
//! Run it using the command `cargo run`.
//!
//!

use prompted::input;
use rsc_lib::{combat::Player, *};

const MAX_PLAYER_HEALTH: usize = 100;
const BASE_ATTACK_DAMAGE: usize = 10;
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

    // Ask for the player's name.
    let player_name: String = input!("Enter your name: ");

    println!();

    let player: Player = Player::new_player(
        player_name,
        MAX_PLAYER_HEALTH,
        BASE_ATTACK_DAMAGE,
        BASE_MOVEMENT,
    );

    let mut room = Room::new(10, 10);

    // println!("{:?}", room);

    show_room(&room);

    println!("Your name is: {}", player.name);
    println!("Your have {} health.", player.health);
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
            None => continue
        };
    }
    //Reprint room
    show_room(&room);
    println!("You inflict {} damage on enemies.", player.attack_damage);

    // Game loop logic - end the game when the player wins or the player dies.
    // let mut is_game_over: bool = false;
}
