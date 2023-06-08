//! Rust Starship Crawler library
//! 4/29/2023
//! CS 510 Rust
//!
//! Library code goes here
//!
//! Code based on HW3 - Chomp
//!
//!
//!

// Import the combat module into the library
pub mod combat;
pub mod pickup;

// Get access to Player and Enemy
use combat::Entity;
use pickup::Pickup;
use prompted::input;

// Randomly generate numbers
// https://docs.rs/rand/latest/rand/trait.Rng.html#method.gen_range
use rand::{thread_rng, Rng};

/// Minimum width of the room.
const MIN_WIDTH: usize = 5;
/// Minimum height of the room.
const MIN_HEIGHT: usize = 5;

/// Maximum width of the room.
const MAX_WIDTH: usize = 10;
/// Maximum height of the room.
const MAX_HEIGHT: usize = 10;

/// A room in a floor.
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

    /// TBD: Pickups list - default value is an empty vector
    /// pub pickups: Vec<Pickup>,
    ///
    /// List of pickup coordinates - default value is an empty vector
    /// pub pickup_coords: Vec<(usize, usize)>,
    ///
    ///

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

    /// The pickup's location.
    ///
    /// All coordinates are 0-indexed. Since a wall is at index 0, the minimum x and y coordinate is 1.
    ///
    /// Example: (1, 1)
    pub pickup_location: (usize, usize),
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
            pickup_location: (1, 5),
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

        // Return a room
        let mut my_room: Room = Room {
            width: room_width,
            height: room_height,
            room_area: vec![vec![true; room_height]; room_width], // Every square is set to false except those within the bounds of nrows and ncols

            // Start with having the player be centered at the top of the room
            player_location: (1, room_height / 2),

            // The enemy is at the bottom of the room
            enemy_location: (room_width - 2, room_height / 2),

            // The pickup location is next to the player
            pickup_location: (1, 2),
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
        Floor {
            rooms: all_rooms, //vec![Room::new_proc_room(); num_rooms],
        }
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

/// Display the room
pub fn show_room(room: &Room) {
    // Show the room
    for i in 0..room.width {
        for j in 0..room.height {
            // Print player location
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
            // Print pickup location
            else if i == room.pickup_location.0 && j == room.pickup_location.1 {
                // check if pickup location and player location are the same
                print!("+");
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

pub fn show_pickup_location(pickup: &Pickup, room: &Room) {
    println!(
        "A {} is in square ({}, {}).",
        pickup.name, room.pickup_location.0, room.pickup_location.1
    );
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
///
pub fn found_pickup(room: Room) -> bool {
    // check to see if the player is on the square with the pickup
    let row: usize = room.player_location.0;
    let col: usize = room.player_location.1; // check to see if the player is on the square with the pickup

    // If we are on the same square as the pickup, consume it
    // otherwise, do nothing
    row == room.pickup_location.0 && col == room.pickup_location.1
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
        player.attack_damage += pickup.effect;
        println!("Attack damage increased by {}.", pickup.effect);
        println!("Your current attack damage is now {}", player.attack_damage);
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
