## I have no idea what I'm doing!

_I know nothing of Rust. This will be fun. :D_

Obvious corollary: this code will be horrible, non-idiomatic, and unsafe. I'm
learning as I go. This means I'll concentrate on exploring the bits that I find
fun, and not necessarily to make the code most efficient or safe.

Hopefully I'll have enough time to tackle most/all of the problems.

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
