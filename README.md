# Minesweeper - Command Line Interface
This is a classic Minesweeper game implemented in Rust language, playable in the command line interface.

## Gameplay
The objective of the game is to clear a rectangular board containing hidden "mines" without detonating any of them, with the help of clues about the number of neighboring mines in each field.

The game begins with a board of covered squares. You can uncover a square by pressing `enter` key or flag it as a suspected mine by pressing `f` key.

If you uncover a square that contains a mine, you lose the game. If you uncover all squares that don't contain a mine, you win.

## Installation
To install and run the game, please ensure that you have Rust language installed on your system. Then, clone this repository and build the executable by running the following commands:
```shell
git clone https://github.com/Diwakar-Gupta/minesweeper.git
cd minesweeper
cargo build --release
```

## How to Play
To start the game, run the following command:
```shell
./target/release/minesweeper
```

This will generate a randomized board for you to play with.

## Controls
The game is played entirely using the keyboard. Use the arrow keys to move the cursor and the enter to uncover a square. f to flag a square as a suspected mine.

## Acknowledgments
This game was inspired by the classic Minesweeper game that shipped with Microsoft Windows.
