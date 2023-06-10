//! Logbook module file
//!
//! 6/10/2023
//!
//! # About
//!
//! This module contains the struct and implementation for a logbook entry object.
//!
//! A logbook entry is a piece of lore that describes a story event.
//!
//! It has a title and a description, both of which are of type String.
//!
//! Logbook entries are hardcoded since the story of the game is the same.
//!
//! A room can have multiple logbook entries. Logbook entries are only found on the first room of each floor.
//!
//! When the player accesses a logbook entry, the entry disappears after it is read.

/// A logbook entry has a title and a description.
///
#[derive(Debug, Clone)]
pub struct Logbook {
    /// The title of the logbook entry.
    pub title: String,
    /// The content of the logbook entry.
    ///
    /// This is a larger string that expands upon the title.
    pub descr: String,
}

/// Implement the Logbook struct.
impl Logbook {
    /// Create a new logbook with the given title and description.
    pub fn new_logbook(title: String, descr: String) -> Self {
        Logbook { title, descr }
    }
}

/// Define the logbook entries for the first room of Floor 1.
///
/// Return a Vector of logbook entries.
pub fn generate_logbook_entries_for_room_1_floor_1() -> Vec<Logbook> {
    let title1: String = "Entry 6101: The Experiments".to_string();
    let descr1: String = "TBD 1".to_string();

    let title2: String = "Entry 6102: Where to?".to_string();
    let descr2: String = "TBD 2".to_string();

    let lb1: Logbook = Logbook::new_logbook(title1, descr1);
    let lb2: Logbook = Logbook::new_logbook(title2, descr2);

    vec![lb1, lb2]
}

/// Define the logbook entries for the first room of Floor 2.
///
/// Return a Vector of logbook entries.
pub fn generate_logbook_entries_for_room_1_floor_2() -> Vec<Logbook> {
    let title1: String = "Entry 6103: The Breakout".to_string();
    let descr1: String = "TBD 3".to_string();

    let title2: String = "Entry 6104: <STATIC>".to_string();
    let descr2: String = "TBD 4".to_string();

    let lb3: Logbook = Logbook::new_logbook(title1, descr1);
    let lb4: Logbook = Logbook::new_logbook(title2, descr2);

    vec![lb3, lb4]
}
