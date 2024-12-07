# advent of code 2024

🎄 This year I'm starting out with using it to learn Rust from scratch, so let's see how long it takes until I switch back to trusted old Python.

1. [Solution](https://github.com/alex-schaaf/adventofcode2024/blob/main/day01/src/main.rs). An easy start this year, which gave me a good opportunity to learn about `Vec` and `HashMap` data structures in Rust. I really like the `Option<T>` enum approach to handling optional values.
2. [Solution](https://github.com/alex-schaaf/adventofcode2024/blob/main/day02/src/main.rs). This one made me learn about cloning vectors, how to turn vectors into iterators, and anonymous functions. I got a bit stuck at first, as I was including 0 as an acceptable value when checking for the gradient monotony. Luckily writing some tests for my gradient monotony function got me to figure that out quickly!
    - Learnt about the `Vec::windows` method, that returns an iterator over a window of given size. So you can just `.map` over it using an anonymous diff function to calculate the gradient array.
3. [Solution](https://github.com/alex-schaaf/adventofcode2024/blob/main/day03/src/main.rs). Simple regex parsing. This time I learned more about how to implement functions that return an `Option<T>`. Combined with the `?` shorthand for propagating errors, this makes for really nice happy-path coding without the verbosity that you see in Go code.
4.
5.
6.
7. [Solution](https://github.com/alex-schaaf/adventofcode2024/blob/main/day07/src/main.rs). I got a bit annoyed here with Rust, as creating the cartesian product of operators felt like an unneccessary hassle to get Rust to do what I wanted it to do. A stark contrast to Python's `itertools`. Also first time in my life I had to deal with an integer overflow, because I was using first `i32` for the calculations, but it required `i64`. Thankfully Rust just tells you that when trying to parse a stringified number.