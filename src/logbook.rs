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
//! When the player accesses a logbook entry, the entry will not disappear, so the player can read it as many times as they want.

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

    // Display the logbook.
    pub fn show_logbook(self) {
        println!("Title: {}", self.title);
        println!("Description: {}", self.descr);
    }
}

/// Define the logbook entries for the first room of Floor 1.
///
/// Return a Vector of logbook entries.
pub fn generate_logbook_entries_for_room_1_floor_1() -> Vec<Logbook> {
    let title1: String = "Entry 6101: The Ceiling Crawler".to_string();
    let descr1: String = "We found some long lost creature DNA while exploring the nearby ice moon. Our researchers and scientists examined it and it turns out this DNA belongs to fierce, ruthless predators.\nThe four-legged creatures, which we dubbed \"Ceiling Crawlers\", are adept at climbing walls and walking on places no human could ever walk. They lack sight, yet their sense of smell is second to none. Their four jaws open up to reveal layers of serrated teeth, capable of munching through the hardest steel. Who knows what ceiling crawlers could do in the wrong hands?".to_string();

    let title2: String = "Entry 6102: Our Plans".to_string();
    let descr2: String = "Imagine all the things we could do if we could harness the power of ceiling crawlers. We could see into caves without risking our lives. We could sniff out anything we couldn't. All we have to do is keep the ceiling crawlers under control and lock the cages every night.".to_string();

    let lb1: Logbook = Logbook::new_logbook(title1, descr1);
    let lb2: Logbook = Logbook::new_logbook(title2, descr2);

    vec![lb1, lb2]
}

/// Define the logbook entries for the first room of Floor 2.
///
/// Return a Vector of logbook entries.
pub fn generate_logbook_entries_for_room_1_floor_2() -> Vec<Logbook> {
    let title1: String = "Entry 6103: The Breakout".to_string();
    let descr1: String = "Oh no! It appears someone forgot to lock the ceiling crawler's cage last night! What do we do?! AAH!!!!\n\n".to_string();

    let title2: String = "Entry 6104: <STATIC>".to_string();
    let descr2: String = "<ENTRY CORRUPTED - ACIDIC LIQUID DETECTED - HANDLE WITH CAUTION>".to_string();

    let lb3: Logbook = Logbook::new_logbook(title1, descr1);
    let lb4: Logbook = Logbook::new_logbook(title2, descr2);

    vec![lb3, lb4]
}

/// Hardcode the logbook coordinates.
pub fn generate_logbook_coordinates() -> Vec<(usize, usize)> {
    vec![(2, 1), (2, 2)]
}
