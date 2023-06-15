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
//! When the player accesses a logbook entry, the entry will disappear.

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
    let title1: String = "Entry 6103: The Drones".to_string();
    let descr1: String = "To guard our ship and its crew, we couldn't just rely on flesh and blood. We also had to employ steel and circuitry, hence our defense force of drones.\nFor years, these drones have kept the peace whenever our crew got into fights or the ceiling crawlers and mutants tried to break out. What could go wrong?".to_string();

    let title2: String = "Entry 6104: Research issues".to_string();
    let descr2: String =
        "The ceiling crawlers are notoriously difficult to keep under control while we tried to get samples from them. Each time we shine a light on them, they keep trying to bite our researchers. One of them chewed a drone into pieces and wrecked our equipment before we tranquilized it. This will set our research back at least a year.".to_string();

    let lb3: Logbook = Logbook::new_logbook(title1, descr1);
    let lb4: Logbook = Logbook::new_logbook(title2, descr2);

    vec![lb3, lb4]
}

/// Define the logbook entries for the first room of Floor 3.
///
/// Return a Vector of logbook entries.
pub fn generate_logbook_entries_for_room_1_floor_3() -> Vec<Logbook> {
    let title1: String = "Entry 6105: The Mutants".to_string();
    let descr1: String = "Alongside the DNA for our ceiling crawlers, we also found radioactive DNA for what appear to be...mutants. These mutants use radioactive powers to survive, and their properties make them immune to solar radiation. They can fire deadly gamma rays to weaken our strength and resolve. I've advised my assistant to be wary around our latest mutant subject, whose powers sent another researcher to the infirmary for months.".to_string();

    let title2: String = "Entry 6106: Funding".to_string();
    let descr2: String =
        "For 2 years we have been doing quite well. However, no research can be done without funding, and we are afraid that funding will run out. Our superiors back at home have given us five more days to get a result out or else they will cut off our funding. We are under immense pressure to deliver, but how can we deliver when our subjects are uncooperative?".to_string();

    let lb5: Logbook = Logbook::new_logbook(title1, descr1);
    let lb6: Logbook = Logbook::new_logbook(title2, descr2);

    vec![lb5, lb6]
}

/// Define the logbook entries for the first room of Floor 4.
///
/// Return a Vector of logbook entries.
pub fn generate_logbook_entries_for_room_1_floor_4() -> Vec<Logbook> {
    let title1: String = "Entry 6107: The Breakout".to_string();
    let descr1: String = "Oh no! Just as our day couldn't get any worse, one of our drones started disobeying orders. We ordered it to defend us from a ceiling crawler that broke loose, but instead, it turned on us and sent us running for cover. Let's just hope my assistant keeps our prize under wraps.\n\n...\n\nOh for star's sake...It appears he forgot to lock the alpha ceiling crawler's cage last night!\n\nWe are so screwed. *GROWLING NOISES*\n\n".to_string();

    let title2: String = "Entry 6108: <CORRUPTED>".to_string();
    let descr2: String =
        "<ENTRY CORRUPTED - ACIDIC LIQUID DETECTED - PROCEED WITH CAUTION>".to_string();

    let lb7: Logbook = Logbook::new_logbook(title1, descr1);
    let lb8: Logbook = Logbook::new_logbook(title2, descr2);

    vec![lb7, lb8]
}

/// Generate the logbook coordinates for two logbooks in one room.
///
/// This is hardcoded for now.
pub fn generate_logbook_coordinates() -> Vec<(usize, usize)> {
    vec![(2, 1), (2, 2)]
}
