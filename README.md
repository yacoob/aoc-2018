## I have no idea what I'm doing!

_I know nothing of Rust. This will be fun. :D_

Obvious corollary: this code will be horrible, non-idiomatic, and unsafe. I'm
learning as I go. This means I'll concentrate on exploring the bits that I find
fun, and not necessarily to make the code most efficient or safe.

Hopefully I'll have enough time to tackle most/all of the problems.

Idea for later: if I end up liking Rust, come back and try to rewrite the
solutions to be more idiomatic. Or faster. Or both.


## Puzzle #2

* `&` usage from yesterday bit explained: complex types that require
  deallocation at the end of their lifecycle aren't copied during assignments
  and function calls.
  [More here](https://doc.rust-lang.org/book/2018-edition/ch04-01-what-is-ownership.html#ownership-and-functions).
* In part A, at few moments I felt that there *has* to be a more elegant
  solution than looping through a collection. Now I need to read about lambdas
  :D
* The difference between `str` and `String` isn't all that clear to me yet.
  I mean, sure, the difference in memory allocation and the way they can behave
  is clear, but the way they are interchangeable thanks to `String`'s traits is
  bit of a mystery.
* Need to carve out the file ingestion function to separate module.
* Used `Option<char>`! It was supper effe... well, it was new at least. On that
  note, I wonder how many things a more experienced Rustacean would change in my
  code. I feel like I tend to fall back to conditional and loops as they're what
  I'm most familiar with. Bit like writing Python without using list
  comprehension: possible, bit verbose, not as much fun.
* interesting approach to part B that I've seen: take a box id, replace one
  character by marker (`_`), save in a set if it's not already there
  [(link)](https://www.reddit.com/r/adventofcode/comments/a2damm/2018_day2_part_2_a_linear_time_solution/eaxco3u/).


## Puzzle #1

In retrospect, tackling this with reading only 5 chapters of ["Rust by
example"](https://doc.rust-lang.org/rust-by-example) might have been bit
counter-productive. There were sufficient number of differences (traits?
ownership?) that could have been better explained in
["The Book"](https://doc.rust-lang.org/book/2018-edition/).

### Observations and questions

* I was pleasantly surprised that at no point have I found an ugly rusty (heh)
  rebar sticking out from underneath. Most of the abstractions mapped nicely
  from other languages that I'm used to.
* Looks like runtime type identification is experimental and will [always
  be](https://doc.rust-lang.org/std/intrinsics/index.html).
  It'd be nice to have it on for debug builds, but oh well.
* Iterating over `changes` actually ate my data. Looks like it's related to the
  concept of memory ownership in Rust.
* At few points while I was playing with it, compiler complained about the fact
  that I need to `use` some traits in order to avail of them. Why some, but not
  the other? Along similar lines, `BufRead` is `use`d but not referenced
  directly.
* Why `.contains_key` wouldn't work with just `current_frequency`, without `&`?
* Those loop labels are weird. Single unpaired quote? :D

### Things to do

* `cargo run` was fun, but I need to make it work with multiple problems
* How do I set up the hierarchy of files for multiple days?

### Things not to do
* Overengineer the exercise using stuff from outside of `std` library,
  especially stuff like [this](https://github.com/Bogdanp/awesome-advent-of-code).
