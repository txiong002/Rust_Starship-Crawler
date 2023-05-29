//! Library file for pickups
//! 5/28/2023
//!
//! A pickup has a name and an effect. It will be placed in a room and the coordinates will be stored elsewhere.
//! The room has a Vector of coordinates keeping track of all pickups.
//!
//! The plan is to have a health pickup that restores 20 HP and an attack pickup that boosts the player's attack damage by 10 points.
//! 

/// A pickup has a name and an effect.
/// 
/// The effect is usually a positive number that affects the player's attack and health.
#[derive(Debug, Clone)]
pub struct Pickup {
    name: String,
    effect: usize,
}

// 
impl Pickup {
    /// Create a new pickup with a name and effect.
    pub fn new_pickup(name: String, effect: usize) -> Self {
        Pickup {
            name,
            effect
        }
    }
}