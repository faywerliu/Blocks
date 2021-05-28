# Blocks Puzzle Solver

This is a program that attempts to solve sliding block puzzles such as Klotski where similarly shaped blocks are not differentiated. Other puzzles such as 15 Puzzle are not supported without adjustments. Klotski can be solved in less than a second on my device.


![link](https://i.ibb.co/mS0Srs8/Screen-Shot-2021-05-27-at-8-17-58-PM.png)

## Inputs
The inputs to the program are located in the folders "start" and "finish". Please name them the same. The first line of "start" and "finish" contains two integers that represent the width and height of the board. This line can be omitted in "finish" as it is assumed the board does not change size. Each subsequent line contains four integers representing the width, height, column, and width of a block. In "finish" if blocks are omitted, it is assumed they can be located in any legal position on the board.

## How to run
In terminal:
>cargo run [name of input]

## Algorithm
This program attempts a breadth first search. This means that the solution found will be the optimal in terms of the minimum moves.

## Interesting Questions
**Why does this program solve Klotski in 116 moves? Wikipedia and other sources claim the optimal solution is 81 moves.** 

This program treats movement from one tile to another as a single move. For instance, if a block slides two tiles at once, this is treated as 2 moves. These other sources assume this slide to be 1 move.

**What is Zobrist Hashing?**

Zobrist Hashing is an efficient way to hash different game states that involve a "board" with positions and different "pieces". To compute the hash, instead of looping through each element of a game state O(|E|), a set of XOR's does this for us in O(1).

**Why Rust?** 

I wanted to try learning it. This project would probably be easier in a language such as C++ or Java since I'm not quite used to Rust yet.

## To (Maybe) Do:
- Add support for puzzles such as "15 Puzzle"
- Record the moves taken.
- GUI
- Optimization
- Adding support for moves that move more than once.
