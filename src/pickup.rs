//! Library file for pickups
//! 5/28/2023
//!
//! A pickup has a name and an effect. It will be placed in a room and the coordinates will be stored elsewhere.
//! The room has a Vector of coordinates keeping track of all pickups.
//!
//! The plan is to have a health pickup that restores 20 HP and an attack pickup that boosts the player's attack damage by 10 points.

use rand::Rng;

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
    ///Randomly build a pick-up
    pub fn generate_pickup() -> Pickup {
        //Randomly generate item type
        let item_type = rand::thread_rng().gen_range(0..3);

        //Based on which item type was generated, create the item
        if item_type == 0 {
            Pickup {
                name: String::from("Medkit"),
                pickup_type: String::from("health"),
                effect: 20,
            }
            //return item;
        } else if item_type == 1 {
            Pickup {
                name: String::from("Pair of Boots"),
                pickup_type: String::from("movement"),
                effect: 1,
            }
            //return item;
        } else {
            Pickup {
                name: String::from("Knife"),
                pickup_type: String::from("attack"),
                effect: 10,
            }
            //return item;
        }
    }
}
