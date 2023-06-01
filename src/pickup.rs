//! Library file for pickups
//! 5/28/2023
//!
//! A pickup has a name and an effect. It will be placed in a room and the coordinates will be stored elsewhere.
//! The room has a Vector of coordinates keeping track of all pickups.
//!
//! The plan is to have a health pickup that restores 20 HP and an attack pickup that boosts the player's attack damage by 10 points.
//! 

/// A pickup has a name, a type, and an effect.
/// 
/// The effect is usually a positive number that affects the player's attack and health.
#[derive(Debug, Clone)]
pub struct Pickup {
    /// The name of the pickup.
    pub name: String,
    /// The type of the pickup. Can be `health`, `attack`, or `movement`.
    pub pickup_type: String,
    /// The effect the pickup has on the player.
    /// 
    /// If pickup_type is `health`, restore 20 health.
    /// 
    /// If pickup_type is `attack`, increase attack by 10.
    /// 
    /// If pickup_type is `movement`, increase movement range by 1.
    pub effect: usize,
}

// 
impl Pickup {
    /// Create a new pickup with a name and effect.
    pub fn new_pickup(name: String, pickup_type: String, effect: usize) -> Self {
        Pickup {
            name,
            pickup_type,
            effect
        }
    }
}