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
    combat::Entity,
    combat::{display_health, face_off},
    pickup::Pickup,
    *,
};

/// The player's health
const MAX_PLAYER_HEALTH: usize = 100;
/// The player's base attack damage.
// const BASE_ATTACK_DAMAGE: usize = 10;
/// The enemy's base attack damage. Default is 9.
const ENEMY_ATTACK_DAMAGE: usize = 9;
/// The number of tiles the player is allowed to traverse.
// const BASE_MOVEMENT: usize = 1;

/// Main function
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
    let mut player: Entity = Entity::new_player(
        player_name,
        80, // starting health is 80, which is 20 less than the maximum of 100
        "Scattershot".to_string(),
        10, // starting attack damage is 10. Set to 100 to defeat enemies instantly (i.e. to debug level progression)
        1,  // starting movement range is 1 tile
    );

    println!("Your name is: {}", player.name);
    println!("You have {} health.", player.health);

    // Create a vector of two floors.
    // The floors have two rooms each
    let num_rooms_per_floor: usize = 2;
    let num_floors: usize = 2;

    // This line will initialize a collection of Floor structs.
    // Each Floor has a set number of room.s
    // levels[0].rooms[0] will access Level 1, Room 1
    // levels[1].rooms[0] will access Level 2, Room 2
    let mut levels: Vec<Floor> = vec![];

    // Counter for initializing
    let mut n: usize = 0;

    // Initialize the rooms and add them to the floor.
    loop {
        levels.push(Floor::new_floor(num_rooms_per_floor));
        n += 1;
        // Stop adding rooms once we add enough of them.
        if n >= num_floors {
            break;
        }
    }

    // Indices to keep track of level index and room index
    //let mut level_index = 0;
    //let mut room_index = 0;
    // Counters to keep track of room number and level number
    let mut count_level: usize = 1; // Level number
    let mut count_room: usize = 1; // Room number in a level

    // outer loop is used for the main game logic
    // Outer loop ends when the game is over (i.e. the player dies or the player finishes the last room in the last floor)
    'outer: loop {
        // FLOOR LOOP: Loop through each floor
        // Floor loop proceeds after all rooms in that floor are cleared
        // Floor loop ends if the player clears the last floor or the player dies.
        // for (f, <item>) in levels.iter().enumerate()
        for (f, floor) in levels.iter().enumerate() {
            // Get the current floor
            let current_floor: Floor = floor.clone();

            println!("\n===== LEVEL {}  =====", f + 1);

            // ROOM LOOP
            // Loop through that floor's set of rooms
            // Room loop ends whenever the player clears a room
            // Room loop breaks if the player dies.
            for r in 0..current_floor.rooms.len() {
                // Get the current room
                let mut room: Room = current_floor.rooms[r].clone();

                println!("\n===== ROOM {}  =====", r + 1);

                // Create a new enemy
                let mut enemy: Entity = Entity::new_enemy(
                    "Ceiling Crawler".to_string(),
                    MAX_PLAYER_HEALTH,
                    "Swipe".to_string(),
                    ENEMY_ATTACK_DAMAGE,
                    0,
                );

                // Create a new health pickup
                let pickup: Pickup = Pickup::generate_pickup();

                // Show the room, player, enemy, and pickup locations.
                show_room(&room);
                show_player_location(&room);
                show_enemy_location(&enemy, &room);
                show_pickup_location(&pickup, &room);

                // Game loop logic - end the game when the player wins or the player dies.
                // inner loop is used to handle fights
                'inner: loop {
                    // // Once player beets Room #02, we change Level

                    println!("===== LEVEL #{} =====", f + 1);

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
                    show_player_location(&room);

                    // Check if the player has found a pickup.
                    // If so, apply the pickup's effects to the player.
                    let pickup_found = found_pickup(room.clone());

                    if pickup_found {
                        println!("You found a {}!", pickup.name);
                        player = apply_pickup_effects(player, &pickup);

                        // Regardless of the pickup type, delete it by resetting its location
                        room.pickup_location = (100, 100);
                    }

                    // Check if the player has found an enemy. If so, the player and the enemy will fight.
                    // Currently the player has no control over when they get to make a move.
                    let is_found = found_enemy(room.clone());

                    if is_found {
                        println!("\nALERT: You encountered a {}!!!", enemy.name);
                        println!("\nGET READY TO BATTLE!!!!");
                        loop {
                            display_health(&player, &enemy); // Show the health values
                            let game_over = face_off(&mut player, &mut enemy); // Player and enemy attack each other

                            //If player wins, move to next level
                            if !game_over && player.health > 0 {
                                count_room += 1;
                                if count_room > num_rooms_per_floor {
                                    println!(
                                        "\n*****  Good job on passing Level {}, moving on to next level...  *****",
                                        f+1
                                    );
                                    // increase player health back to full for testing purposes
                                    player.health = MAX_PLAYER_HEALTH;
                                    count_room = 1; // reset the counter as we move to the next level.
                                                    // Move to the next room
                                } else {
                                    println!("\n============= You are now entering Room # {}, Good Luck!  ==============", r+2);

                                    // replace player health back to full for testing purposes
                                    player.health = MAX_PLAYER_HEALTH;
                                }
                                input!("Press ENTER to move to next room");
                                // Add health to player for next level
                                // player.health += 20;
                                break 'inner;
                            } else if !game_over {
                                // If either the enemy or the player has lost all their health, the game ends.
                                println!("Game over, Thanks for playing");
                                break 'outer;
                            }
                        }
                    } else {
                        continue;
                    }
                    // END is_found
                }
                // End inner loop
            }
            // End room loop

            // Increment the level counter
            count_level += 1;

            // End the game when the user finishes the last level
            if count_level > num_floors {
                println!(" ====== ALL LEVELS CLEARED ====== ");
                println!("Mission complete, Thanks for playing!");
                break 'outer;
            }
        }
        // End floors loop
    }
    // End outer loop
}
