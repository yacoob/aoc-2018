## I have no idea what I'm doing!

_I know nothing of Rust. This will be fun. :D_

Obvious corollary: this code will be horrible, non-idiomatic, and unsafe. I'm
learning as I go. This means I'll concentrate on exploring the bits that I find
fun, and not necessarily to make the code most efficient or safe.

Hopefully I'll have enough time to tackle most/all of the problems.

Idea for later: if I end up liking Rust, come back and try to rewrite the
solutions to be more idiomatic. Or faster. Or both.


## 11th of December, puzzle #11
* Tapioca brain day. First I've found a semi-sensible `O(n^3)` algorithm, then
  reddit suggested a
  [summed-area table](https://en.wikipedia.org/wiki/Summed-area_table). Great
  idea, but I've spent my evening hunting for off by one errors. Not a good day.
* Both parts finish <150ms; before using summed-area table I've been tempted to
  try `rayon` with this one. Maybe in another puzzle.


## 10th of December, puzzle #10
* **Puzzle:** Star-writing: collapsing starfield versus image recognition.
* **Solution:** Iterative + modest application of human eye 8)

* The puzzle itself was really simple. Used `image` crate instead of producing
  an ascii art. Added some appropriate quotes to the functions comments.
* `clippy` suggested nice work around for a immutable/mutable borrow conflict
  during image making. I was first borrowing `old_stars` to calculate
  `x_margin`, and then modifying `old_stars.x[i]`. Initially I've `.clone()`d the
  values out, but clippy noted that the whole thing has `Copy` trait, so I can
  just dereference the `old_stars`. Much cleaner code.
* Went on with the smushing of A and B parts for day 2; I got into some
  lifetimes scuttle, read about
  [lifetimes](https://doc.rust-lang.org/nightly/book/ch10-03-lifetime-syntax.html),
  fallen asleep while reading. Finished the code afterwards, but it's probably
  not as good as it could be.


## 9th of December, puzzle #9
* **Puzzle:** Elves playing horribly worded fully deterministic solitare game.
* **Solution:** Iterative; `Vec` based too slow, `VecDeque` acceptable.

* Nothing like spending half an hour debugging your solution, only to discover
  that you left a function unimplemented, always returning `0`. :D
* Also, wording in that puzzle was fuzzy, to say the least. Part 2 had *"if
  the number of the last marble were 100 times larger"*. Last marble? Or last
  marbles? Confusion ensues. Maybe it was part of the difficulty. 🙀
* But hey, I wrote another `fmt::Display` trait!
* Optimisation brought `VecDeque`, and <2.5s running time, instead of +1h. And
  a `--release` binary finishes part B in <100ms. This puzzle is the first
  puzzle where I had to worry about running time.
* Not **that much** new things today; need to read more of The Book to keep my
  appetite.


## 8th of December, puzzle #8
* **Puzzle:** Licence Check: integer stream to tree parsing and mangling.
* **Solution:** Stack-based parser plus some tree traversal. No recursion!

* **"Recursion is bad"** I thought, then spent some time juggling indices of
  a vector emulating a tree. 🤷
* Number of times I've written `nods` instead of `nods`: `ETOOMANY`. *nods*
* Syntax for a `for` loop going in reverse is slightly silly. `(a..b).rev()`,
  really? :D Worse, I don't think I've seen a warning that `(i..j)` with `i>j`
  is going to give you an empty range.
* Default iterator is `into_iter`; I thought it's `iter`. It's still bit
  confusing, need to read
  [up](https://blog.codeship.com/understanding-rust-loops/)
  on
  [that](https://doc.rust-lang.org/std/iter/index.html).
* Way too many off-by-one errors in part B. I'm probably tired today.


## 7th of December, puzzle #7
* **Puzzle:** Building a sleigh: instructions dependency hell.
* **Solution:** Lexicographic topological sort. The algorithm I ended up with
  looks like
  [Kahn's](https://www.geeksforgeeks.org/topological-sorting-indegree-based-solution/).

* This code turned out to be beefier than I expected. This is usually a sign
  that same thing could have been done in a more concise way :D
* Finally used `clippy`; it only checks things that are being compiled, which
  means that if you run it twice there'll be no output on the second run.
* Added some tests in the same file; this forced me to split code into
  functions; this in turn made me add `asserts` to `main`, to make sure I don't
  break the code while I refactor it.
* `<small>`I got lost while looking at another solution, due to the fact that it
  was using proper English math terms for bits of graph, and I don't know this
  terminology well :o`</small>`
  UPDATE: Looks like this puzzle maps to
  [lexicographical topological sorting](https://en.wikipedia.org/wiki/Topological_sorting),
  and the thing I've thrown together looks a bit like Kahn's algorithm :D Now,
  the question is whether I'd have more fun implementing that algorithm based on
  its description...
* `clippy` suggested to use `.or_insert_with(|| Step::new(foo))` to avoid
  calling `Step::new()` regardless whether `HashMap`'s key exist or not. Took me
  some time to understand that it's a lambda that takes no parameters, instead
  of logical operator with no left argument. Now it makes sense - make a lambda
  that captures `foo`, pass it to `or_insert_with`, save one function call.
* Couldn't get `clippy` to play nicely with Vim.
* I got an impression that sometimes syntax errors were not caught by Syntastic
  in the test section.
* What should I use for things like default value of `last_worked_on`? Given
  that I *need* to have a value, I end up using a sentinel value, which isn't
  not ideal.
* Given that part B produces more output than usual, I ended up using
  `eprintln!` for debug output.
* At one point I've used `BinaryHeap<Step>` to keep `steps`. Alas, there's no free
  launch, and nothing will rearrange the heap for me, if I go behind heap's
  back and change step's `prerequisites`.


## 6th of December, puzzle #6
* **Puzzle:** Teleportation destination selection a'la Voronoi partition.
* **Solution:** Brute-force; it was fast enough and existing algorithms were
  rather complex.

* I've found a reviewer for the style of my Rust code. Thanks, @Lucretiel!
* I'll most likely take a day off the schedule; I need to get some other
  entertainment, and read up more of the Rust book.
* I got to write a method, a class method and a trait for my struct!
* Entertained a thought of going after something more sophisticated in terms of
  Voronoi partition for a very short moment. Then I got better.
* First I sorted a vector full of tuples, only then I've noticed that it Just
  Works[tm].
  [Ord trait](https://doc.rust-lang.org/std/primitive.tuple.html#impl-Ord)?
* Part B was suspiciously easy, easier than part A.


## 5th of December, puzzle #5
* **Puzzle:** Polymer assembly: string processing and recution.
* **Solution:** L-R scan with a stack.

* It's kind of a good thing, that Rust's `regex` doesn't support backreferences.
  I'd *so* very much trade one problem for two. :D
* Why can't Rust debug print a slice of a
  [vector of ints](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=e92b3fe86ad61b68a8c6d623e2fdb416)?
  Adding a reference there works, but I'm unsure why wouldn't it work without
  one in the first place.


### Other people did this
* Instead of tracking position and last character, you can just walk through the
  input left to right and
  [push/pop units to output as necessary](https://github.com/ttencate/aoc2018/blob/master/src/bin/05.rs).
  It has also made me realise that
  [Option::map](https://doc.rust-lang.org/std/option/enum.Option.html#method.map)
  is a thing, so I've decided to incorporate this. :3


## 5th of December, puzzle #4
* RE: time management: **I'll try not to stay up with AoC past 22:30**. Which
  might mean I won't do anything for some days. Now, should I try to catch up or
  leave the puzzles for later...?
* `rustup doc` opens local documentation in a browser; ~~it's a tad bit faster for
  looking up specific methods (online version lags a between loading a page and
  jumping to correct anchor).~~ Local documentation also suffers from the problem;
  there's a
  [huge jsified index being loaded on every page](https://github.com/rust-lang/rust/issues/56545).
* In retrospect, the actual day number wasn't relevant for neither part A or
  B of the puzzle; knowing this would simplify the structures. I could see that
  in part A, but thought it might be relevant for part B. Nerdsniped, eh? It's
  a good thing that I didn't end up implementing something akin to Python's
  `datetime`.
* Learned a thing in `Vim`: `Ctrl-w, H/J/KL` move current window to requested
  position, redoing the split in the process. Nice way to switch from horizontal
  to vertical split.
* I don't feel comfortable with borrowing just yet. Especially, when I'm mucking
  around with iterators and chain in `parse` or `unwrap`, I always get lost on
  whether I'm dealing with a reference or actual value.
* Similarily, not enough functional. Granted, simpler data structure would also
  help, but I feel like I couuld shorten a **lot** of `04a` with a careful use
  of `flatten`.


## 4th of December, puzzle #4
* **Puzzle:** Sleepy guards: schedule comparison and statistics.
* **Solution:** Hash of hashes and some needling.

As expected, starting at midnight is counterproductive. I really need to figure
out something else. Probably slow down.

* Had to switch to Rust 2018, to avail of
  [non-lexical lifetimes](https://rust-lang-nursery.github.io/edition-guide/rust-2018/ownership-and-lifetimes/non-lexical-lifetimes.html).
  I mean, I could have worked around it with an extra pair of braces, to create
  extra inner scope, but the release date for Rust 2018 is *tomorrow*, so
  I might as well start using it now.
* Couldn't alias type as it required a lifetime specification, and I know
  nothing about those yet :D


## 3th of December, puzzle #3
* **Puzzle:** Fabric cutting: rectangles overlap and such.
* **Solution:** Iterative map painting, straight-forward.

* Started late, had a break just before end of part A. Not sure how sustainable
  it'll be to do those puzzles in the evenings; I have a late thing tomorrow.
* Yeah, I've read about structs today. Does it show? 8)
* Initially, I've been parsing the line with `.split_whitespace()` as `regex`
  crate is not in `std` (why?). It was getting too tedious, so I've used
  `regex`.
* Fencepost errors. I knew I'll have at least one with a puzzle like this.
* One day I'll learn proper error handling. Until then, there are `asserts` and
  trust in puzzle's description.
* write code, dump resulting structure via `Debug` print, grep for one `false`
  that should be there, submit answer, spend next half an hour playing with
  a functional chain that will collapse that structure into nice single integer :D
* Speaking of which, `filter_map` is nice, but that condition inside is bit
  ugly. Having separate `filter` and `map` would require having two lambdas
  taking `(&id, &tainted)` as argument, so that's also a bit meh. Any better way?
* I need to check where all those structures were created; I think tuples are
  always on stack, so I was running a chance of overflowing the stack in part A.
* After doing part B, I've came back to part A and redone it with `Vec`; it
  looks cleaner, but I'm bit lost as to why do I need `&&` inside that last
  lambda.

### Other people did this
* Not only make a struct for each rectangle, but also implement `from_string`
  for it, which parses an input line.
  [Neat](https://github.com/k0nserv/advent-of-rust-2018/blob/master/src/day03.rs#L25)


## 2nd of December, puzzle #2
* **Puzzle:** Searching for correct boxes: strings diffing.
* **Solution:** Brute-force, with a sliver of optimisation. Still getting my
  Rust legs on.


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

### Other people did this
* Take a box id, replace one character by marker (`_`), save in a set if it's
  not already there.
  [(link)](https://www.reddit.com/r/adventofcode/comments/a2damm/2018_day2_part_2_a_linear_time_solution/eaxco3u/).


## 1st of December, puzzle #1

* **Puzzle:** Teleporter calibration: running sums of long list of integer.
* **Solution:** See the puzzle description. :D

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
