# 🎅🏻🌟🎄 Advent of Code 2022

This repository contains my solutions to [Advent of Code 2023](https://adventofcode.com/2023) in Rust 🦀.

From [AoC's](https://adventofcode.com/2023) about page:

> Advent of Code is an Advent calendar of small programming puzzles for a variety of skill sets and skill levels that can be solved in any programming language you like. People use them as interview prep, company training, university coursework, practice problems, a speed contest, or to challenge each other.

## How to run

To run the solution for a given day, run the following command:

```bash
cargo run --bin day[N]
# example: cargo run --bin day1
```

where `N` is the day number.

To run tests for a given day, run the following command:

```bash
cargo test --bin day[N]
```

where `N` is the day number.

## Project structure

The project is structured as follows:

```bash
src
 | - lib                 # exports utility functions shared between days
 | - bin
      | - day1
          | - example.in # example input
          | - input.in   # my unique input for the day
          | - main.rs
      | - day2           # same for day 2, etc
      | - ...

```
