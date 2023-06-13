//! Rust Starship Crawler library
//!
//! 4/29/2023
//!
//! CS 510 Rust
//!
//! Library code goes here
//!
//! Initial room generation ode based on HW3 - Chomp
//!
//!
//!

// Import the combat module into the library
pub mod combat;
// Import the pickup module
pub mod pickup;
// Import the logbook module
pub mod logbook;

// Get access to the player/enemy, pickup, and logbook objects.
use combat::Entity;
use logbook::Logbook;
use pickup::Pickup;
use prompted::input;

// Randomly generate numbers
// https://docs.rs/rand/latest/rand/trait.Rng.html#method.gen_range
use rand::{thread_rng, Rng};

/// Minimum width of the room.
const MIN_WIDTH: usize = 7;
/// Minimum height of the room.
const MIN_HEIGHT: usize = 5;

/// Maximum width of the room.
const MAX_WIDTH: usize = 10;
/// Maximum height of the room.
const MAX_HEIGHT: usize = 10;

/// A room in a floor.
///
/// The room is the playable area where all the entities (player and enemies), pickups, and logbooks are spawned.
///
/// The minimum and maximum size of the room can be set via the constants MIN_WIDTH, MIN_HEIGHT, MAX_WIDTH, and MAX_HEIGHT.
#[derive(Debug, Clone)]
pub struct Room {
    /// The width of the room.
    ///
    /// The max index is `width - 2` since `width - 1` will have a wall.
    pub width: usize,

    /// The height of the room.
    ///
    /// The max index is `height - 2` since `height - 2` will have a wall.
    pub height: usize,

    /// The room. It can be hard-coded or procedurally generated.
    ///
    /// Each value is a boolean where `true` is a walkable tile or entity. `false` is a wall.
    ///
    /// Since the room will be generated at runtime, it needs to be a Vector.
    pub room_area: Vec<Vec<bool>>,

    /// Player location (set of coordinates) inside current room (maybe set this up a different way later?)
    ///
    /// All coordinates are 0-indexed. Since a wall is at index 0, the minimum x and y coordinate is 1.
    ///
    /// Example: (1, 1)
    pub player_location: (usize, usize),

    /// The enemy's location.
    ///
    /// All coordinates are 0-indexed. Since a wall is at index 0, the minimum x and y coordinate is 1.
    ///
    /// Example: (1, 1)
    pub enemy_location: (usize, usize),

    /// Pickups list - default value is an empty vector
    pub pickups: Vec<Pickup>,
    /// List of pickup coordinates - default value is an empty vector
    ///
    /// All coordinates are 0-indexed. Since a wall is at index 0, the minimum x and y coordinate is 1.
    ///
    /// Example: (1, 1)
    pub pickup_coords: Vec<(usize, usize)>,

    /// Logbooks list - default value is an empty vector
    pub logbooks: Vec<Logbook>,

    /// List of logbook coordinates - default value is an empty vector
    pub logbook_coords: Vec<(usize, usize)>,
}

impl Room {
    /// Initialize the room with the given width and height.
    ///
    /// Set up the walls.
    pub fn new_static_room(width: usize, height: usize) -> Self {
        // Return a room
        let mut my_room = Room {
            width,
            height,
            room_area: vec![vec![true; height]; width], // Every square is set to false except those within the bounds of nrows and ncols

            // Start with having the player be centered at the bottom of the room
            player_location: (1, 5),
            enemy_location: (8, 4),
            pickups: vec![],
            pickup_coords: vec![(1, 5)],
            logbooks: vec![],
            logbook_coords: vec![],
        };

        my_room = set_up_walls(my_room);
        my_room
    }

    /// Procedurally generate a room using random numbers.
    /// The width and height of the room will be different each time this function
    /// is run.
    ///
    /// `Returns`
    /// A Room object
    pub fn new_proc_room() -> Self {
        // Randomly determine the width and height of the room.
        let mut rng = thread_rng();
        let room_width: usize = rng.gen_range(MIN_WIDTH..=MAX_WIDTH);
        let room_height: usize = rng.gen_range(MIN_HEIGHT..=MAX_HEIGHT);

        // Randomly determine how many pickups will be present.
        // Number is between 1 and 3.
        let num_pickups: usize = rng.gen_range(1..=3);

        // Generate the pickups and their coordinates.
        let mut all_pickups: Vec<Pickup> = vec![];
        let mut all_pickup_coords: Vec<(usize, usize)> = vec![];
        loop {
            // Randomly generate a pickup coordinate that stays within the bounds of the room
            let coord: (usize, usize) = (
                rng.gen_range(2..=room_width - 2),
                rng.gen_range(1..=room_height - 2),
            );

            // Add the pickup coordinate
            all_pickup_coords.push(coord);

            // Generate the initial pickup
            let new_pickup: Pickup = Pickup::generate_pickup();

            // If the pickup is an attack or health pickup, add it. 
            // But only add a pair of boots if there isn't one already.
            
            // New pickup is a pair of boots and there isn't one already.
            if new_pickup.pickup_type == "movement" && !(all_pickups.contains(&new_pickup)) {
                all_pickups.push(new_pickup);
            // There is a pair of boots already in the room. Generate a medkit or an attack pickup.
            } else {
                // Keep generating the pickup until it isn't a pair of boots.
                'get_non_movement: loop {
                    let new_pickup: Pickup = Pickup::generate_pickup();
                    if new_pickup.pickup_type != "movement" {
                        all_pickups.push(new_pickup);
                        break 'get_non_movement;
                    }
                }
            }

            // When all pickups are added, stop the loop.
            if all_pickup_coords.len() == num_pickups && all_pickups.len() == num_pickups {
                break;
            }
        }

        // Return the procedurally generated room
        let mut my_room: Room = Room {
            width: room_width,
            height: room_height,
            room_area: vec![vec![true; room_height]; room_width], // Every square is set to false except those within the bounds of nrows and ncols

            // Start with having the player be centered at the top of the room
            player_location: (1, room_height / 2),

            // The enemy is at the bottom of the room
            enemy_location: (room_width - 2, room_height / 2),

            // The pickup location is next to the player
            pickups: all_pickups,
            pickup_coords: all_pickup_coords,

            // Logbooks and their coordinates
            logbooks: vec![],
            logbook_coords: vec![],
        };

        my_room = set_up_walls(my_room);
        my_room
    }
}

/// A Floor is a level in the game.
///
/// Each Floor has one or more Rooms.
///
#[derive(Debug, Clone)]
pub struct Floor {
    pub rooms: Vec<Room>,
}

/// Functions for floor
impl Floor {
    /// Create a new floor with a given number of procedurally generated rooms.
    pub fn new_floor(num_rooms: usize) -> Self {
        // Create an empty Room vector.
        let mut all_rooms: Vec<Room> = vec![];

        // Counter for initializing
        let mut i: usize = 0;

        // Initialize the rooms and add them to the floor.
        loop {
            all_rooms.push(Room::new_proc_room());
            i += 1;
            // Stop adding rooms once we add enough of them.
            if i >= num_rooms {
                break;
            }
        }

        // Return the finished floor.
        Floor { rooms: all_rooms }
    }
}

/// Set up the room's walls
pub fn set_up_walls(mut room: Room) -> Room {
    let num_rows: usize = room.room_area.len();
    let num_cols: usize = room.room_area[0].len();
    for i in 0..num_rows {
        // First and last row are walls
        room.room_area[i][0] = false;
        room.room_area[i][num_cols - 1] = false;

        // Top and bottom rows are all walls
        if i == 0 || i == num_rows - 1 {
            for j in 0..num_cols {
                room.room_area[i][j] = false;
            }
        }
    }

    // Return the completed room with walls
    room
}

/// Display the room with walls, tiles, the player, enemies, pickups and logbook entries.
pub fn show_room(room: &Room) {
    // The string representing the room.
    let mut room_string: String = "".to_string();

    // Build the room to display, adding the right char to the room_string.
    // room_string.extend(['L'].iter()); will concatenate a single character to the String object.
    for i in 0..room.width {
        for j in 0..room.height {
            // Tile is a floor
            // It may or may not be occupied by an object.
            if room.room_area[i][j] {
                // Show the player as ^
                if i == room.player_location.0 && j == room.player_location.1 {
                    room_string.extend(['^'].iter());

                // Show the enemy as X
                } else if i == room.enemy_location.0 && j == room.enemy_location.1 {
                    let is_found = found_enemy(room.clone());
                    if is_found {
                        room_string.extend(['X'].iter());
                    } else {
                        room_string.extend(['.'].iter());
                    }

                // Show the pickup locations as +
                } else if room.pickup_coords.contains(&(i, j)) {
                    // check if pickup location and player location are the same
                    room_string.extend(['+'].iter());
                // Show all logbook locations if there are any.
                // This is hardcoded for now since there are only two logbook entries per room
                } else if room.logbook_coords.contains(&(i, j)) {
                    // Is the current coordinate a logbook coordinate? If so, display it as 'L'
                    room_string.extend(['L'].iter());

                // Show a normal tile - general case
                } else {
                    // Unoccupied Tile, display with a dot.
                    room_string.extend(['.'].iter());
                }
            // Show a wall
            } else {
                // Wall
                room_string.extend(['#'].iter());
            }
        }
        // Go to the next row
        room_string.extend(['\n'].iter());
    }

    // Show the room
    println!("{}", room_string);
}

/// Function to move player location (basic idea is that once a player touches an "enemy" tile, battle initiates?)
/// Get arguments from user (similar to input from chomp)
pub fn user_move(mut room: Room, player: &Entity) -> Option<Room> {
    let mut user_move: (usize, usize);
    //Get user input
    let user_input: String = input!("Where do you want to move? (Input Format: 1 4): ")
        .parse()
        .unwrap();

    let user_move_str: Vec<&str> = user_input.split_whitespace().collect::<Vec<_>>();

    //Make sure the user didn't input too much
    if user_move_str.len() == 2 {
        //Turn the string vector into a usize vector (return None if one of them fails to map to the usize vector)
        let v: Vec<usize> = match user_move_str
            .iter()
            .map(|s| s.parse())
            .collect::<Result<Vec<usize>, _>>()
        {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid input! Please use positive numbers only!");
                return None;
            }
        };
        //Set the usize tuple
        user_move = (v[0], v[1]);

        //Check to see if the input was out of bounds or goes into a wall
        if user_move.0 >= room.width - 1 || user_move.1 >= room.height - 1 {
            println!("You can't move here!");
            return None;
        }

        // User tries to go upwards or leftwards into a wall
        if user_move.0 == 0 || user_move.1 == 0 {
            println!("You can't move here!");
            return None;
        }

        // Edge case: User tries to move diagonally upwards to the right
        // user_move.0 < room.player_location.0
        if (user_move.0 < room.player_location.0) && (user_move.1 > room.player_location.1) {
            if ((room.player_location.0 - user_move.0) > player.movement)
                || ((user_move.1 - room.player_location.1) > player.movement)
            {
                println!("You cannot move that far!");
                return None;
            }
        }
        // Edge case: User tries to move diagonally downwards to the left
        // user_move.1 < room.player_location.1
        else if (user_move.0 > room.player_location.0) && (user_move.1 < room.player_location.1) {
            if ((user_move.0 - room.player_location.0) > player.movement)
                || ((room.player_location.1 - user_move.1) > player.movement)
            {
                println!("You cannot move that far!");
                return None;
            }
        }
        // General purpose code: Ensure that player move is within their current movement bounds
        // The code below will crash if the player is moving diagonally upwards to the right or diagonally downward to the left.
        else if (user_move.0 > room.player_location.0) || (user_move.1 > room.player_location.1) {
            if ((user_move.0 - room.player_location.0) > player.movement)
                || ((user_move.1 - room.player_location.1) > player.movement)
            {
                println!("You cannot move that far!");
                return None;
            }
        }
        // Same, but if the move is meant to be going "backwards"
        else if ((room.player_location.0 - user_move.0) > player.movement)
            || ((room.player_location.1 - user_move.1) > player.movement)
        {
            println!("You cannot move that far!");
            return None;
        }

        let mut r = room.player_location.0;
        let mut c = room.player_location.1;

        //Check to make sure that the user didn't pass the enemy, and if the user's move will move past the enemy, move the user to the next
        //closest spot
        loop {
            loop {
                if (r + 1) == room.enemy_location.0 && (c) == room.enemy_location.1 {
                    user_move = (r, c);
                    room.player_location = user_move;
                    return Some(room);
                } else if r == room.enemy_location.0
                    && ((c + 1) == room.enemy_location.1 || (c - 1) == room.enemy_location.1)
                {
                    user_move = (r - 1, c);
                    room.player_location = user_move;
                    return Some(room);
                } else if c == user_move.1 {
                    break;
                }
                if user_move.1 < room.player_location.1 {
                    c -= 1;
                } else {
                    c += 1;
                }
            }
            if r == user_move.0 {
                break;
            }
            if user_move.0 < room.player_location.0 {
                r -= 1;
            } else {
                r += 1;
            }
        }

        //If the move was entirely valid, set the user's move and return
        room.player_location = user_move;
        return Some(room);
    }
    println!("Invalid input! Please make sure that your input is two positive integers!");
    None
}

/// Show the player coordinates.
pub fn show_player_location(room: &Room) {
    println!(
        "You are in square ({}, {}).",
        room.player_location.0, room.player_location.1
    );
}

/// Show the enemy coordinates.
pub fn show_enemy_location(enemy: &Entity, room: &Room) {
    println!(
        "An enemy {} is in square ({}, {}).",
        enemy.name, room.enemy_location.0, room.enemy_location.1
    );
}

/// Show the pickups and their locations.
pub fn show_pickup_locations(room: &Room) {
    for (p, coords) in room.pickup_coords.iter().enumerate() {
        println!(
            "A {} is in square ({}, {}).",
            room.pickups[p].name, coords.0, coords.1
        );
    }
}

/// Check if player has found the enemy.
///
/// The enemy is found if the enemy is one tile away from the player.
pub fn found_enemy(room: Room) -> bool {
    let row: usize = room.player_location.0 + 1; // check to see if player is 1 row above enemy
    let col: usize = room.player_location.1.wrapping_sub(2); // check to see if player is 2 columns to the right of enemy
    let col_2: usize = room.player_location.1.wrapping_add(2); // check to see if player is 2 columns to the left of enemy

    if row == room.enemy_location.0 {
        // If we are in range of the enemy, initiate the battle
        // otherwise, do nothing
        col <= room.enemy_location.1 || col_2 >= room.enemy_location.1
    } else {
        false
    }
}

/// Check if the player has found a pickup.
///
/// This happens when the player and the pickup are on the same square
pub fn found_pickup(room: Room) -> (Pickup, (usize, usize), usize, bool) {
    // check to see if the player is on the square with the pickup
    let row: usize = room.player_location.0;
    let col: usize = room.player_location.1; // check to see if the player is on the square with the pickup

    // Locate and return the appropriate pickup
    // for (f, <item>) in levels.iter().enumerate()
    for (c, coord) in room.pickup_coords.iter().enumerate() {
        if row == coord.0 && col == coord.1 {
            // Get that pickup
            let pickup: Pickup = room.pickups[c].clone();

            // // Get the index of the logbook coordinates and use that to get the index of the logbook entry.
            let pickup_coords: (usize, usize) = room.pickup_coords[c];

            // Return the logbook entry with its coordinates, its index, and a true value.
            return (pickup, pickup_coords, c, true);
        }
    }

    // No pickup was found, return a 4-tuple with a null pickup, placeholder coordinates, a placeholder index, and a false value.
    (Pickup::generate_pickup(), (100, 100), 100, false)
}

/// Apply pickup effects to the player depending on the pickup type.
pub fn apply_pickup_effects(mut player: Entity, pickup: &Pickup) -> Entity {
    // Health pickup
    if pickup.pickup_type == "health" {
        if player.health <= 80 {
            // Health is 80 or lower, add the health back
            player.health += pickup.effect;
            println!("Health restored by {}.", pickup.effect);
            println!("Your current health is now {}", player.health);
        } else {
            // Health is 81 or higher, set health to 100
            player.health = 100;
        }
    } else if pickup.pickup_type == "attack" {
        player.attacks[0].damage_value += pickup.effect;
        println!("Attack damage increased by {}.", pickup.effect);
        println!(
            "Your current attack damage is now {}",
            player.attacks[0].damage_value
        );
    } else if pickup.pickup_type == "movement" {
        player.movement += pickup.effect;
        println!("Movement range increased by {} tile.", pickup.effect);
        println!(
            "Your current movement range is now {} tiles",
            player.movement
        );
    }
    player
}

/// Check if the player has found a logbook entry.
///
/// This happens when the player and the logbook entry are on the same square
pub fn found_logbook(room: Room) -> (Logbook, (usize, usize), usize, bool) {
    // check to see if the player is on the square with the pickup
    let row: usize = room.player_location.0;
    let col: usize = room.player_location.1; // check to see if the player is on the square with the pickup

    // Locate and return the appropriate logbook
    // for (f, <item>) in levels.iter().enumerate()
    for (c, coord) in room.logbook_coords.iter().enumerate() {
        if row == coord.0 && col == coord.1 {
            // Get that logbook entry
            let logbook_entry: Logbook = room.logbooks[c].clone();

            // Get the index of the logbook coordinates and use that to get the index of the logbook entry.
            let logbook_coords = room.logbook_coords[c];

            // Return the logbook entry with its coordinates, its index, and a true value.
            return (logbook_entry, logbook_coords, c, true);
        }
    }

    // No logbook was found, return a 4-tuple with a null logbook, placeholder coordinates, a placeholder index, and a false value.
    (
        Logbook::new_logbook("NULL".to_string(), "NULL".to_string()),
        (100, 100),
        100,
        false,
    )
}

/// Test that a new procedurally generated room has a width and height <= 10
/// Test that the room's size is correctly defined.
#[test]
fn test_new_proc_room() {
    for i in 0..100 {
        println!("Test #{}", i);

        // Create a room.
        let room: Room = Room::new_proc_room();

        // Assert that the width and height of the procedurally generated room is less than 10.
        assert_eq!(room.width <= 10, true);
        assert_eq!(room.height <= 10, true);

        assert_eq!(room.room_area.len(), room.width);
        assert_eq!(room.room_area[0].len(), room.height);
    }
}
