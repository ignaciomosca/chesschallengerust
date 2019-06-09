# Chess Challenge

# Problem
The problem is to find all unique configurations of a set of normal chess pieces on a chess board with dimensions MxN where none of the pieces is in a position to take any of the others. Assume the colour of the piece does not matter, and that there are no pawns among the pieces.
Write a program which takes as input:
The dimensions of the board: M, N
The number of pieces of each type (King, Queen, Bishop, Rook and Knight) to try and place on the board.

As output, the program should list all the unique configurations to the console for which all of the pieces can be placed on the board without threatening each other.

When returning your solution, please provide with your answer the total number of unique configurations for a 7x7 board with 2 Kings, 2 Queens, 2 Bishops and 1 Knight. Also provide the time it took to get the final score. Needless to say, the lower the time, the better.

# The Solution

The following solution is based on the [N-Queen Problem](https://developers.google.com/optimization/cp/queens) which is a well known optimization problem taught in Computer Science Classes. The proposed solution makes use of a technique called backtracking to find all the solutions to the problem.
Based on experience with previous problems, I reduced the current problem to an N-Queen problem with an 8x8 Board. I figured that after being able to get the solution for that problem, solving the rest of it was a matter of adding more pieces, determining how they moved, and writing unit tests for them.

# Dependencies

* Rust
* Cargo

# Instructions

* `git clone https://github.com/ignaciomosca/chesschallengerust.git`
* `cd chesschallengerust`
* `cargo build`
* `cargo run` | `cargo test`