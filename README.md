Dice Roller
===========

A handy `roll` command for rolling dice on the command line.

Usage
-----

```
roll [--verbose] [DEFINITION]
roll --help
roll --version
```

### Options

* `--help`: display available commands and options.
* `--verbose`: include the individual dice rolls in the output; by default displays only the total.
* `--version`: display version.

### Definition

Uses standard [dice notation][wiki-dice-notation] to define which dice and modifiers to use.

* `roll`: an easy-peasy six-sided dice roll.
* `roll d20`: specify to roll a 20-sided die.
* `roll 2d8`: roll two 20-sided dice.
* `roll 2d10+4`: roll 2 10-sided dice and add 4 to the result.
* `roll 3d4-2`: roll 3 4-sided dice and subtract 2 from the result.

Installation
------------

### Via Cargo

If you already have the Rust toolchain installed:

```
cargo install dice-roller
```

### Via Homebrew

```
# add this repo as a tap
brew tap pbyrne/roller https://github.com/pbyrne/roller
# then you can install like any other package
brew install dice-roller
```

### Download the binaries

OS-specific binaries are attached to [each release][releases] automatically. Download the one that's right for you and place it somewhere you can call it.

TODOs
-----

* `2d5+4d20`
* Graceful error handling on invalid input

[wiki-dice-notation]:https://en.wikipedia.org/wiki/Dice_notation
[releases]:https://github.com/pbyrne/roller/releases
