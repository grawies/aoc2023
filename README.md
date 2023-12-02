# aoc2023
Advent of Code 2023 solutions while learning Rust.

The program expects data in subdirectories like this:
```
data/
     day_1/
           input.txt
           test1
     day_2/
           input.txt
     ... etc
```
...relative to where the program is run.

How to build:
```
cargo build
cargo build --release
```

How to run:
```
# Run all (hitherto solved) days with default input.txt.
$ target/release/aoc2023

# Run only the part 2 solutions for days 1 and 2, part 2
# on respective input files "data/day_1/test1" and "data/day_2/test1".
$ target/release/aoc2023 --days=1,2 --part2 --input_file=test1
```
