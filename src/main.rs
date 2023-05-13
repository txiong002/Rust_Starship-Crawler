//! Main file
//! 4/28/2023
//! CS 510 Rust
//!
//! This file is used to execute the game.
//!
//! Run it using the command `cargo run`.
//!
//!
use rsc_lib::*;

/// Display the room
fn show_room(room: &Room) {
    // Show the room
    for i in 0..room.width {
        for j in 0..room.height {
            // Print a room tile depending on whether it is a wall or a floor.
            if room.room_area[i][j] {
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
    println!("TBD");

    let room = Room::new(10, 10);

    println!("{:?}", room);

    show_room(&room);
}
