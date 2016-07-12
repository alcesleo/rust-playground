# 8 queens puzzle

Finds all possible solutions to the [8 queens puzzle][] (actually the N queens
puzzle, it works for other sizes as well) with a recursive backtracking
algorithm.

```bash
$ cargo run

...

. . . . . . . Q
. . Q . . . . .
Q . . . . . . .
. . . . . Q . .
. Q . . . . . .
. . . . Q . . .
. . . . . . Q .
. . . Q . . . .

. . . . . . . Q
. . . Q . . . .
Q . . . . . . .
. . Q . . . . .
. . . . . Q . .
. Q . . . . . .
. . . . . . Q .
. . . . Q . . .

Found 92 solutions
```

[8 queens puzzle]: https://en.wikipedia.org/wiki/Eight_queens_puzzle
