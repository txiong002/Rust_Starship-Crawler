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
// Get access to Player and Enemy
//use combat::Player;
use combat::Entity;
use prompted::input;

/// Maximum number of rows the AI can handle.
const MAX_WIDTH: usize = 10;
/// Maximum number of columns the AI can handle.
const MAX_HEIGHT: usize = 10;

/// A room in a floor.
#[derive(Debug, Clone)]
pub struct Room {
    /// The width of the room.
    pub width: usize,

    /// The height of the room
    pub height: usize,

    /// The room. Each value is a boolean where `true` is a walkable tile or entity. `false` is a wall.
    pub room_area: [[bool; MAX_HEIGHT]; MAX_WIDTH], // A list of up to 4 arrays, each one containing up to 5 booleans

    /// A randomly generated room. It has the same values as `room_area`.
    /// Since the room will be generated at runtime, it needs to be a Vector.
    // pub proc_room_area: Vec<bool>,

    // Player location (set of coordinates) inside current room (maybe set this up a different way later?)
    pub player_location: (usize, usize),
    pub enemy_location: (usize, usize),
}

impl Room {
    /// Initialize the room with the given width and height.
    /// Set up the walls.
    pub fn new(width: usize, height: usize) -> Self {
        // Return a room
        let mut my_room = Room {
            width,
            height,
            room_area: [[true; MAX_HEIGHT]; MAX_WIDTH], // Every square is set to false except those within the bounds of nrows and ncols

            //Start with having the player be centered at the bottom of the room
            player_location: (1, 4),
            enemy_location: (8, 4),
        };

        my_room = set_up_walls(my_room);
        my_room
    }
}

/// Set up the room's walls
pub fn set_up_walls(mut room: Room) -> Room {
    for i in 0..room.room_area.len() {
        // First and last row are walls
        room.room_area[i][0] = false;
        room.room_area[i][room.room_area.len() - 1] = false;

        // Top and bottom rows are all walls
        if i == 0 || i == room.room_area.len() - 1 {
            for j in 0..room.room_area[0].len() {
                room.room_area[i][j] = false;
            }
        }
    }

    // Return the room
    room
}

/// Function to move player location (basic idea is that once a player touches an "enemy" tile, battle initiates?)
/// Get arguments from user (similar to input from chomp)
pub fn user_move(mut room: Room, player: &Entity) -> Option<Room> {
    let user_move: (usize, usize);
    //Get user input
    let user_input: String = input!("Where do you want to move? (Input Format: 1 4): ")
        .parse()
        .unwrap();
    let user_move_str = user_input.split_whitespace().collect::<Vec<_>>();
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
        //Check to see if the input was out of bounds
        if user_move.0 >= room.height || user_move.1 >= room.width {
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

        // If the move is confirmed to be a valid movement for the player to make
        //Change player position
        room.player_location = user_move;
        return Some(room);
    }
    println!("Invalid input! Please make sure that your input is two positive integers!");
    None
}

/// Check if player has found the enemy
/// The enemy is found if the enemy is one tile away from the player.
pub fn found_enemy(room: Room) -> bool {
    let row = room.player_location.0 + 1; // check to see of player is 1 row above enemy
    let col = room.player_location.1.wrapping_sub(2); // check to see if player is 2 columns to the right of enemy
    let col_2 = room.player_location.1.wrapping_add(2); // check to see if player is 2 columns to the left of enemy

    if row == room.enemy_location.0 {
        // if col <= room.enemy_location.1 || col_2 >= room.enemy_location.1 {
        //     true
        // } else {
        //     false
        // }
        // If we are in range of the enemy, initiate the battle
        // otherwise, do nothing
        col <= room.enemy_location.1 || col_2 >= room.enemy_location.1
    } else {
        false
    }
}
