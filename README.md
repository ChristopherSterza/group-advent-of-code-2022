# Advent of Code 2022

Using AoC to learn a new language; this year is rust.

### Running the code

To run the code you will need the standard rust-lang setup, learn how
[here](https://www.rust-lang.org/learn/get-started).

Executing `cargo {run|build|test}` at the root level will execute that command
for all of the packages.

> e.g. `cargo test` at the root level will execute **all** tests (day1, day2,
> ... )

Executing the command in the `dayN/` directory will target _only_ that package.
Alternatively, you can execute `cargo <cmd> -p dayN ...` to target a specific
package.

Then, to execute day 1's exercises we need to:

`cargo run -p day1 --bin part1`

for part 1 and

`cargo run -p day1 --bin part2`

for part 2.

> replace **run** with **test** in the above snippets to target a specific
> package to test
