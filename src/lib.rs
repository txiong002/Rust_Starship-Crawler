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

    /// The board. `true` is the walkable area, `false is
    /// a wall.
    pub room_area: [[bool; MAX_HEIGHT]; MAX_WIDTH], // A list of up to 4 arrays, each one containing up to 5 booleans
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
