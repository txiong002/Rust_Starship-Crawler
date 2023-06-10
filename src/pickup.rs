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
    /// The name of the pickup. This is the pickup's in-game name.
    pub name: String,
    /// The type of the pickup. Can be `health`, `attack`, or `movement`.
    ///
    /// `health` = affects the player's HP. Max HP is 100.
    ///
    /// `attack` = affects the player's attack damage.
    ///
    /// `movement` = affects the player's movement range.
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

// Implementation of a Pickup
impl Pickup {
    ///Randomly build a pick-up
    pub fn generate_pickup() -> Pickup {
        //Randomly generate item type
        let item_type = rand::thread_rng().gen_range(0..3);

        //Based on which item type was generated, create the item.
        if item_type == 0 {
            Pickup {
                name: String::from("Medkit"),
                pickup_type: String::from("health"),
                effect: 20, // Increase current health by 20
            }
        } else if item_type == 1 {
            Pickup {
                name: String::from("Pair of Boots"),
                pickup_type: String::from("movement"),
                effect: 1, // Increase movement range by 1
            }
        } else {
            Pickup {
                name: String::from("Knife"),
                pickup_type: String::from("attack"),
                effect: 10, // Increase attack by 10
            }
        }
    }
}

/// Test that a pickup is randomly generated with the correct type and effects.
#[test]
fn test_generate_pickup() {
    // For
    for i in 0..100 {
        // Generate the pickup.
        let pickup: Pickup = Pickup::generate_pickup();

        println!("Test {}: Generated a {}", i, pickup.name);

        // Assert that the pickup_type and effect are correct based on the pickup's name.
        if pickup.name == "Medkit" {
            assert_eq!(pickup.pickup_type, "health");
            assert_eq!(pickup.effect, 20);
        } else if pickup.name == "Pair of Boots" {
            assert_eq!(pickup.pickup_type, "movement");
            assert_eq!(pickup.effect, 1);
        } else if pickup.name == "Knife" {
            assert_eq!(pickup.pickup_type, "attack");
            assert_eq!(pickup.effect, 10);
        }
    }
}
