# Rust Starship Crawler
## 4/23/2023

## Description

This is a roguelike game where the player is an explorer investigating an abandoned starship for its secrets. Along the way, the player will find hidden treasures, useful powerups, and terrifying monsters. 

## Installation

Open a terminal and clone the repository with the command below:

`git clone https://gitlab.cecs.pdx.edu/cs510-rust-lth/rust-starship-crawler.git`

## How to play

1) Open a terminal and type the command 

    `cargo run`

2) When prompted, enter your name.

3) You will be shown a map and your current position. To move on the map, enter the x-coord and y-coord of the square you want to move to.

    Example: If you are on square (1, 4) and you want to move to square (2, 4), enter `2 4`.

    Note that you can only move one square at a time.

4) If your destination square contains a pickup, the pickup will either restore 20 HP, boost your attack power by 10, or increase your movement range by 1.

5) When you get close to an enemy, it will appear and you will enter a battle. To attack, press the enter key.

6) When you defeat an enemy, you will move to the next room (or the next floor if you are in the last room of the current floor).

7) If your HP reaches 0, the game will end and you will need to restart from the beginning. 

# References

Lee: I reused code from HW3 to help set up the dungeon room.

The code we reused was based on the code to define (`struct`), implement (`impl`), and display (`show_posn`) the Chomp board.

Definition of a Roguelike game:
https://en.wikipedia.org/wiki/Roguelike

Rand Crate reference:
https://docs.rs/rand/latest/rand/trait.Rng.html#method.gen_range