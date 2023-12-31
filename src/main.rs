//! Main file
//! 4/28/2023
//! CS 510 Rust
//!
//! This file is used to execute the rust-starship-crawler game.
//!
//! Run it using the command `cargo run`.
//!
//! Have fun!

// Imports ========================================
// For prompting user input
use prompted::input;

// Using rust modules and libraries
use rsc_lib::{
    combat::{display_player_and_enemy_health, display_player_health, face_off, regenerate_health},
    combat::{Attack, Entity},
    logbook::{generate_logbook_coordinates, Logbook},
    logbook::{
        generate_logbook_entries_for_room_1_floor_1, generate_logbook_entries_for_room_1_floor_2,
        generate_logbook_entries_for_room_1_floor_3, generate_logbook_entries_for_room_1_floor_4,
    },
    pickup::Pickup,
    *,
};

// Randomly generate numbers to determine hit chance and determine which enemies spawn
// https://docs.rs/rand/latest/rand/trait.Rng.html#method.gen_range
use rand::{thread_rng, Rng};

// ================================================
// CONSTANTS

// Maximun number of Levels
const MAX_LEVELS: usize = 4;

// ================================================

/// Spawn the enemy based on which room and level we are in.
///
/// Enemy Name - determine which enemy should spawn
///
///    0 = Ceiling Crawler
///
///    1 = Rogue drone
///
///    2 = Radioactive Mutant
///
/// # Arguments
///
/// count_level = the current level number. Starts from 1.
///
/// count_room = the current room number. Starts from 1.
///
/// num_rooms_per_floor = the number of rooms per floor. Minimum value is 1.
///
fn spawn_enemy(count_level: usize, count_room: usize, num_rooms_per_floor: usize) -> Entity {
    let mut rng = thread_rng();

    // Determine which enemy should spawn in each room
    // Enemy Name - determine which enemy should spawn
    // 0 = Ceiling crawler
    // 1 = Rogue drone
    // 2 = Radioactive Mutant
    let enemy_name: &str;
    let enemy_health: usize;
    let mut enemy_attacks: Vec<Attack> = vec![];

    // Final level is the Boss level - generate a boss
    // It has Acid Spit, Bite, and Swipe as its main attacks.
    if count_level == MAX_LEVELS && count_room == num_rooms_per_floor {
        enemy_name = "BOSS: Alpha Ceiling Crawler";
        enemy_health = 200;
        let attack_1 = Attack::new_attack("Acid Spit".to_string(), 35);
        let attack_2 = Attack::new_attack("Swipe".to_string(), 25);
        let enemy_attack_bite: Attack = Attack::new_attack("Bite".to_string(), 30);
        enemy_attacks.push(attack_1);
        enemy_attacks.push(attack_2);
        enemy_attacks.push(enemy_attack_bite);
    // All other levels - generate a random enemy
    } else {
        // Determine the enemy's starting health value based on the level
        // Enemies on later levels have more health than those on the early levels.
        let starting_enemy_health: usize = if count_level == 1 {
            25
        } else if count_level == 2 {
            50
        } else if count_level == 3 {
            75
        } else {
            100
        };

        // Pick an enemy to spawn.
        let enemy_index: usize = rng.gen_range(0..=2);

        if enemy_index == 0 {
            // Set up the enemy name and health.
            enemy_name = "Ceiling Crawler";
            enemy_health = starting_enemy_health;

            // Define the enemy's attack list.
            let enemy_attack_bite: Attack = Attack::new_attack("Bite".to_string(), 5);
            let enemy_attack_swipe: Attack = Attack::new_attack("Swipe".to_string(), 9);
            let enemy_attack_acid_blob: Attack = Attack::new_attack("Acid Blob".to_string(), 12);
            enemy_attacks.push(enemy_attack_bite);
            enemy_attacks.push(enemy_attack_swipe);
            enemy_attacks.push(enemy_attack_acid_blob);
        } else if enemy_index == 1 {
            // Set up the enemy name and health.
            enemy_name = "Rogue Drone";
            enemy_health = starting_enemy_health;

            // Define the enemy's attack list.
            let enemy_attack_laser: Attack = Attack::new_attack("Laser Blast".to_string(), 11);
            let enemy_attack_mini_missile: Attack =
                Attack::new_attack("Mini Missiles".to_string(), 15);
            enemy_attacks.push(enemy_attack_laser);
            enemy_attacks.push(enemy_attack_mini_missile);
        } else {
            // Set up the enemy name and health.
            enemy_name = "Radioactive Mutant";
            enemy_health = starting_enemy_health;

            // Define the enemy's attack list.
            let enemy_attack_gamma_ray: Attack = Attack::new_attack("Gamma Ray".to_string(), 13);
            let enemy_attack_tackle: Attack = Attack::new_attack("Tackle".to_string(), 7);
            enemy_attacks.push(enemy_attack_gamma_ray);
            enemy_attacks.push(enemy_attack_tackle);
        }
    }

    // Create a new enemy based on the above parameters
    let enemy_backpack: Vec<Pickup> = vec![];
    let enemy: Entity = Entity::new_enemy(
        enemy_name.to_string(),
        enemy_health,
        enemy_attacks,
        enemy_backpack,
        0,
    );

    // Return the enemy.
    enemy
}

/// Show an opening message when the player arrives at the start of a floor.
///
/// # Arguments
///
/// floor_index: The index of the current level. Starts from 0.
fn show_floor_message(floor_index: usize) {
    if floor_index == 0 {
        // FIRST FLOOR
        println!("\n===== LEVEL {}: CARGO BAY  =====", floor_index + 1);
        println!("You crack open the starship's charred cargo bay door and make your way inside the first room. There is a low growling sound from across the hall. You draw your weapon and scan the area.");
    } else if floor_index == 1 {
        // SECOND FLOOR
        println!("\n===== LEVEL {}: HANGAR  =====", floor_index + 1);
        println!("You survived the enemies hiding in the cargo bay. You make your way to the hangar on the second floor.\nThere, you see the charred and acid-scarred wrecks of starships big and small.\n");
    } else if floor_index == 2 {
        // SECOND FLOOR
        println!("\n===== LEVEL {}: COMMAND CENTER  =====", floor_index + 1);
        println!("You survived the enemies hiding in the hangar. You make your way to the command center on the third floor.\nIn this room, the ship's captain would manage the crew and keep track of the ship's vital functions.\n");
    } else if floor_index == MAX_LEVELS - 1 {
        // SECOND FLOOR
        println!("\n===== LEVEL {}: RESEARCH LAB  =====", floor_index + 1);
        println!("You survived the enemies hiding in the command center. You make your way to the final floor where the research lab lies.\nThe research lab is the heart of the crew's experiments on genetic engineering. The lead scientists were all aiming for a breakthrough, but alas, a terrifying secret cut their research short.\nYou hear a deafening roar from the adjoining lab.\n");
    }
}

// ================================================

/// Main function
fn main() {
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

    // Loop for restarting or quitting the game
    'state: loop {
        // Create the player entity with their attack list, health, and other values.
        let player_attacks: Vec<Attack> = vec![Attack::new_attack("Scattershot".to_string(), 10)];

        // Create the player's backpack, which starts empty.
        let player_backpack: Vec<Pickup> = vec![];

        // Create new player
        let mut player: Entity = Entity::new_player(
            player_name.clone(),
            80,             // starting health is 80, which is 20 less than the maximum of 100
            player_attacks, // starting attack damage is 10. Set to 100 to defeat enemies instantly (i.e. to debug level progression)
            player_backpack,
            1, // starting movement range is 1 tile
        );

        println!("Your name is: {}", player.name);
        println!("You have {} health.", player.health);

        // Create a vector of two floors.
        // The floors have two rooms each
        let num_rooms_per_floor: usize = 2;
        let num_floors: usize = 4;

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

        // Generate the logbook entries for the first room of Floor 3.
        let r1f3_logbooks: Vec<Logbook> = generate_logbook_entries_for_room_1_floor_3();
        let r1f3_lb_coords: Vec<(usize, usize)> = generate_logbook_coordinates();

        // Generate the logbook entries for the first room of Floor 4.
        let r1f4_logbooks: Vec<Logbook> = generate_logbook_entries_for_room_1_floor_4();
        let r1f4_lb_coords: Vec<(usize, usize)> = generate_logbook_coordinates();

        // Indices to keep track of level index and room index
        // Counters to keep track of room number and level number
        let mut count_level: usize = 1; // Level number, starts from 1.
        let mut count_room: usize = 1; // Room number in a level. Starts from 1.

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
                show_floor_message(f);

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
                    } else if f == 2 && r == 0 {
                        room.logbooks = r1f3_logbooks.clone();
                        room.logbook_coords = r1f3_lb_coords.clone();
                    } else if f == 3 && r == 0 {
                        room.logbooks = r1f4_logbooks.clone();
                        room.logbook_coords = r1f4_lb_coords.clone();
                    }

                    println!("\n===== ROOM {}  =====", r + 1);

                    // Determine which enemy should spawn in each room
                    // Enemy Name - determine which enemy should spawn
                    // 0 = Ceiling crawler
                    // 1 = Rogue drone
                    // 2 = Radioactive Mutant
                    let mut enemy: Entity =
                        spawn_enemy(count_level, count_room, num_rooms_per_floor);

                    // Show the room, player, enemy, and pickup locations.
                    show_room(&room);
                    show_player_location(&room);
                    show_pickup_locations(&room);

                    // Game loop logic - end the game when the player wins or the player dies.
                    // inner loop is used to handle fights
                    'inner: loop {
                        // // Once player beets Room #02, we change Level

                        println!();
                        // Show the player's health at all times.
                        display_player_health(&player);
                        println!("You can move {} space.", player.movement);
                        let mut check = 0;
                        while check == 0 {
                            // Get user move
                            room = match user_move(room.clone(), &player) {
                                Some(room) => {
                                    check = 1;
                                    room
                                }
                                // Continue if move was invalid
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

                            // Player cannot pick up more than one pair of boots
                            if player.backpack.contains(&pickup_found.0)
                                && pickup_found.0.name == "Pair of Boots"
                            {
                                println!("You already have a Pair of Boots!");
                            } else {
                                // Pickup is not a pair of boots or the player doesn't have a pair of boots

                                // Apply pickup effects.
                                player = apply_pickup_effects(player, &pickup_found.0);

                                // Add the pickup to the player's backpack if it isn't a medkit.
                                if pickup_found.0.pickup_type != "health" {
                                    player.backpack.push(pickup_found.0);
                                    show_player_backpack(&player);
                                }

                                // Regardless of the pickup type, delete it by resetting its location
                                room.pickup_coords.remove(pickup_found.2);
                                room.pickups.remove(pickup_found.2);
                            }
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
                                display_player_and_enemy_health(&player, &enemy); // Show the health values
                                let game_over = face_off(&mut player, &mut enemy); // Player and enemy attack each other

                                // If player wins, move to next level
                                if !game_over && player.health > 0 {
                                    count_room += 1;
                                    if count_level == MAX_LEVELS && count_room > num_rooms_per_floor
                                    {
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
                                        regenerate_health(&mut player);
                                        count_room = 1; // reset the counter as we move to the next level.
                                                        // Move to the next room
                                    } else {
                                        println!("\n============= You are now entering Room # {}, Good Luck!  ==============", r+2);

                                        // replace player health back to full for testing purposes
                                        regenerate_health(&mut player);
                                    }
                                    input!("Press ENTER to move to next room");
                                    break 'inner;
                                } else if !game_over {
                                    // If either the enemy or the player has lost all their health, the game ends.
                                    println!("Game Over");

                                    // Prompt the user for input to determine whether they want to restart the game
                                    loop {
                                        let choice: String = input!(
                                            "Would you like to restart the game? (Y/N)\n\nChoice: "
                                        );

                                        if choice == "N" || choice == "n" {
                                            // // End the game
                                            println!("Thank you for playing!");
                                            break 'state;
                                        } else if choice == "Y" || choice == "y" {
                                            // Player answers yes - game restarts with new values
                                            break 'outer;
                                        } else {
                                            println!("Invalid input - please enter Y or N.")
                                        }
                                    }
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

                    // Prompt the user for input to determine whether they want to restart the game
                    loop {
                        let choice: String =
                            input!("Would you like to restart the game? (Y/N)\n\nChoice: ");

                        if choice == "N" || choice == "n" {
                            // // End the game
                            println!("Thank you for playing!");
                            break 'state;
                        } else if choice == "Y" || choice == "y" {
                            // Player answers yes - game restarts with new values
                            break 'outer;
                        } else {
                            println!("Invalid input - please enter Y or N.")
                        }
                    }
                }
            }
            // End floors loop
        }
        // End outer loop
    }
    // End restart/quit loop
}
