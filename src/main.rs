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
    combat::{display_health, face_off, regenerate_health},
    logbook::{generate_logbook_coordinates, Logbook},
    logbook::{
        generate_logbook_entries_for_room_1_floor_1, generate_logbook_entries_for_room_1_floor_2,
    },
    pickup::Pickup,
    *,
};

// Randomly generate numbers to determine hit chance and determine which enemies spawn
// https://docs.rs/rand/latest/rand/trait.Rng.html#method.gen_range
use rand::{thread_rng, Rng};

// ================================================

/// The player's health
const _MAX_PLAYER_HEALTH: usize = 100;
/// The enemy's health - it varies by floor
// let ENEMY_HEALTH_LVL1: usize = 25;
// let ENEMY_HEALTH_LVL2: usize = 50;
// let ENEMY_HEALTH_LVL3: usize = 75;
// let ENEMY_HEALTH_LVL4: usize = 100;
/// The player's base attack damage.
// const BASE_ATTACK_DAMAGE: usize = 10;
/// The enemy's base attack damage. Default is 9.
// const ENEMY_ATTACK_DAMAGE: usize = 9;
/// The number of tiles the player is allowed to traverse.
// const BASE_MOVEMENT: usize = 1;

// Maximun number of Levels
const MAX_LEVELS: usize = 2;

/// Main function
fn main() {
    // Set up the RNG
    let mut rng = thread_rng();

    // Welcome message
    println!("==== Welcome to Starship Crawler! ====");
    println!();
    println!("BACKSTORY: You are an explorer assigned to investigate an abandoned cargo ship that was lost in space. The cargo ship was last seen departing a tropical planet with some treasure on board. There are rumors of strange and terrifying monstrosities lurking within the cargo bays and engineering rooms, but that won't stop you from finding out the starship's secrets, won't it?");
    println!();
    println!("Are you ready to discover the secrets and treasures within?");
    println!();

    // Ask for the player's name.

    let mut player_name: String;
    loop {
        player_name = input!("Enter your name: ");

        // End the loop if the player name isn't blank - otherwise, keep asking for the player name.
        if !player_name.is_empty() {
            break;
        } else {
            println!("ERROR - Player name cannot be blank! Please enter a player name!");
        }
    }

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

    // Generate the logbook entries for the first room of Floor 1.
    let r1f1_logbooks: Vec<Logbook> = generate_logbook_entries_for_room_1_floor_1();
    let r1f1_lb_coords: Vec<(usize, usize)> = generate_logbook_coordinates();

    // Generate the logbook entries for the first room of Floor 2.
    let r1f2_logbooks: Vec<Logbook> = generate_logbook_entries_for_room_1_floor_2();
    let r1f2_lb_coords: Vec<(usize, usize)> = generate_logbook_coordinates();

    // Indices to keep track of level index and room index
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

            // Each floor has a backstory to tell.
            // The text is hardcoded for now
            if f == 0 {
                // FIRST FLOOR
                println!("\n===== LEVEL {}: CARGO BAY  =====", f + 1);
                println!("You crack open the starship's charred cargo bay door and make your way inside the first room. There is a low growling sound from across the hall. You draw your weapon and scan the area.");
            } else if f == 1 {
                // SECOND FLOOR
                println!("\n===== LEVEL {}: COMMAND CENTER  =====", f + 1);
                println!("You survived the enemies hiding in the cargo bay. You make your way to the second floor.\nThis is the command center floor where all the ship's crew would fly it and keep track of the ship's vital functions.\n");
            }

            // ROOM LOOP
            // Loop through that floor's set of rooms
            // Room loop ends whenever the player clears a room
            // Room loop breaks if the player dies.
            for r in 0..current_floor.rooms.len() {
                // Get the current room
                let mut room: Room = current_floor.rooms[r].clone();

                // For the first room of each floor, set the logbooks
                if f == 0 && r == 0 {
                    room.logbooks = r1f1_logbooks.clone();
                    room.logbook_coords = r1f1_lb_coords.clone();
                } else if f == 1 && r == 0 {
                    room.logbooks = r1f2_logbooks.clone();
                    room.logbook_coords = r1f2_lb_coords.clone();
                }

                println!("\n===== ROOM {}  =====", r + 1);

                // Determine which enemy should spawn in each room
                // Enemy Name - determine which enemy should spawn
                // 0 = Ceiling crawler
                // 1 = Rogue drone
                // 2 = Radioactive Mutant
                let enemy_name: &str;
                let enemy_attack_damage: usize;
                let enemy_health: usize;
                let enemy_attack_name: &str;

                // Final level is the Boss level - generate a boss
                if count_level == MAX_LEVELS && count_room == num_rooms_per_floor {
                    enemy_name = "BOSS: Alpha Ceiling Crawler";
                    enemy_attack_damage = 25;
                    enemy_health = 200;
                    enemy_attack_name = "Acid Spit";
                // All other levels - generate a random enemy
                } else {
                    // Determine the enemy's starting health value based on the level
                    // Enemies on later levels have more health than those on the early levels.
                    let starting_enemy_health: usize = if count_level == 1 {
                        25
                    } else if count_level == 2 {
                        50
                    } else {
                        100
                    };

                    // Pick an enemy to spawn.
                    let enemy_index: usize = rng.gen_range(0..=2);

                    if enemy_index == 0 {
                        enemy_name = "Ceiling Crawler";
                        enemy_attack_damage = 9;
                        enemy_health = starting_enemy_health;
                        enemy_attack_name = "Swipe";
                    } else if enemy_index == 1 {
                        enemy_name = "Rogue Drone";
                        enemy_attack_damage = 11;
                        enemy_health = starting_enemy_health;
                        enemy_attack_name = "Laser Blast";
                    } else {
                        enemy_name = "Radioactive Mutant";
                        enemy_attack_damage = 13;
                        enemy_health = starting_enemy_health;
                        enemy_attack_name = "Gamma Ray";
                    }
                }

                // Create a new enemy based on the above parameters
                let mut enemy: Entity = Entity::new_enemy(
                    enemy_name.to_string(),
                    enemy_health,
                    enemy_attack_name.to_string(),
                    enemy_attack_damage,
                    0,
                );

                // // Create a new health pickup
                // let pickup: Pickup = Pickup::generate_pickup();

                // Show the room, player, enemy, and pickup locations.
                show_room(&room);
                show_player_location(&room);
                show_enemy_location(&enemy, &room);
                show_pickup_locations(&room);

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
                    // Reprint the room with the player location.
                    show_room(&room);
                    show_player_location(&room);

                    // Check if the player has found a pickup.
                    // If so, apply the pickup's effects to the player.
                    let pickup_found: (Pickup, (usize, usize), usize, bool) =
                        found_pickup(room.clone());

                    if pickup_found.3 {
                        println!("You found a {}!", pickup_found.0.name);
                        player = apply_pickup_effects(player, &pickup_found.0);

                        // Regardless of the pickup type, delete it by resetting its location
                        room.pickup_coords.remove(pickup_found.2);
                        room.pickups.remove(pickup_found.2);
                        // room.pickup_location = (100, 100);
                    }

                    // Check if the player found a logbook entry.
                    // If so, show the respective entry.
                    // Obtain the logbook, the coordinates of the logbook, the logbook index, and a boolean that states whether the logbook was found or not.
                    // logbook_found.0 = the Logbook object
                    // logbook_found.1 = the coordinates of logbook_found.0
                    // logbook_found.2 = the index of the coordinates.
                    // logbook_found.3 = was the logbook found at the player's position?
                    let logbook_found: (Logbook, (usize, usize), usize, bool) =
                        found_logbook(room.clone());

                    if logbook_found.3 {
                        // Show the logbook.
                        println!("You found a logbook!");
                        logbook_found.0.show_logbook();

                        // Remove the logbook from the map.
                        room.logbook_coords.remove(logbook_found.2);
                        room.logbooks.remove(logbook_found.2);
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
                                if count_level == MAX_LEVELS && count_room > num_rooms_per_floor {
                                    println!("{}, you have completed all levels!", player.name);
                                    break 'inner;
                                } else if count_level <= MAX_LEVELS
                                    && count_room > num_rooms_per_floor
                                {
                                    println!(
                                        "\n*****  Good job on passing Level {}, moving on to next level...  *****",
                                        f+1
                                    );
                                    // increase player health back to full for testing purposes
                                    //player.health = MAX_PLAYER_HEALTH;
                                    regenerate_health(&mut player);
                                    count_room = 1; // reset the counter as we move to the next level.
                                                    // Move to the next room
                                } else {
                                    println!("\n============= You are now entering Room # {}, Good Luck!  ==============", r+2);

                                    // replace player health back to full for testing purposes
                                    //player.health = MAX_PLAYER_HEALTH;
                                    regenerate_health(&mut player);
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
            if count_level > MAX_LEVELS {
                println!(" ====== ALL LEVELS CLEARED ====== ");
                println!("Mission complete, Thanks for playing!");
                break 'outer;
            }
        }
        // End floors loop
    }
    // End outer loop
}
