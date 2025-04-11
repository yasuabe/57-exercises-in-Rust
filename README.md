# *Exercises for Programmer* in Rust

## Overview
A personal project to get started with Rust by solving the exercises from the book *Exercises for Programmers* in Rust.

### Solved Exercises
#### Chapter 2: Input, Processing, and Output
- Ex01. [Saying Hello](src/bin/main01.rs)
- Ex02. [Counting the Number of Characters](src/bin/main02.rs)
- Ex03. [Printing Quotes](src/bin/main03.rs)
- Ex04. [Mad Lib](src/bin/main04.rs)
- Ex05. [Simple Math](src/bin/main05.rs)
- Ex06. [Retirement Calculator](src/bin/main06.rs)
#### Chapter 3: Calculations
- Ex07. [Area of a Rectangular Room](src/bin/main07.rs)
- Ex08. [Pizza Party](src/bin/main08.rs)
- Ex13. [Determining Compound Interest](src/bin/main13.rs)
#### Chapter 4:
- Ex14. [Tax Calculator](src/bin/main14.rs)
- Ex23. [Troubleshooting Car Issues](src/bin/main23.rs)
#### Chapter 5:
- Ex24. [Anagram Checker](src/bin/main24.rs)
- Ex27. [Validating Inputs](src/bin/main27.rs)
#### Chapter 6:
- Ex28. [Adding Numbers](src/bin/main28.rs)
- Ex32. [Guess the Number Game](src/bin/main32.rs)
#### Chapter 7:
- Ex33. [Magic 8 Ball](src/bin/main33.rs)
- Ex40. [Filtering Records](src/bin/main40.rs)
#### Chapter 8:
- Ex41. [Name Sorter](src/bin/main41.rs)
- Ex46. [Word Frequency Finder](src/bin/main46.rs)
#### Chapter 9:
- Ex47. [Who’s in Space?](src/bin/main47.rs)
- Ex52. [Creating Your Own Time Service](src/bin/main52.rs)
#### Chapter 10:
- Ex53. [Todo List](src/bin/main53.rs)

## Technologies Used

- rust 1.85.1, cargo 1.85.1
- [libraries](Cargo.toml): [chrono](https://docs.rs/chrono/latest/chrono/), [once_cell](https://docs.rs/once_cell/latest/once_cell/), [regex](https://docs.rs/regex/latest/regex/),
 [rand](https://rand/docs.rs/latest/rand/),
 [itertools](https://docs.rs/itertools/latest/itertools/),
 [reqwest](https://docs.rs/reqwest/latest/reqwest/),
 [tokio](https://docs.rs/tokio/latest/tokio/), 
 [serde](https://docs.rs/serde/latest/serde/), 
 [serde_json](https://docs.rs/serde_json/latest/serde_json/), 
 [actix-web](https://docs.rs/actix-web/latest/actix-web/),

## How to Run
Run the following directly under the project.
```
$ cargo run --bin main[nn]
```
### Ex52: Creating Your Own Time Service
1. Run cargo run --bin main52.
2. From another terminal, run cargo run --bin main52_client.

### Ex53: Todo List
Start Redis in your local environment with the command below, then run cargo run --bin main53:
```
$ docker run --name redis-local -d -p 6379:6379 redis
```

## Notes
- I relied on Vibe Coding just a little bit.

## References
- [Exercises for Programmers](https://www.oreilly.com/library/view/exercises-for-programmers/9781680501513/)
- [Rust](https://www.rust-lang.org/)
