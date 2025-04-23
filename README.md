# *Exercises for Programmers* in Rust

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
- Ex09: [Paint Calculator](src/bin/main09.rs)
- Ex10: [Self-Checkout](src/bin/main10.rs)
- Ex11. [Currency Conversion](src/bin/main11.rs)
- Ex12. [Computing Simple Interest](src/bin/main12.rs)
- Ex13. [Determining Compound Interest](src/bin/main13.rs)
#### Chapter 4: Making Decisions
- Ex14. [Tax Calculator](src/bin/main14.rs)
- Ex15. [Password Validation](src/bin/main15.rs)
- Ex16. [Legal Driving Age](src/bin/main16.rs)
- Ex17. [Blood Alcohol Calculator](src/bin/main17.rs)
- Ex20. [Multistate Sales Tax Calculator](src/bin/main20.rs)
- Ex23. [Troubleshooting Car Issues](src/bin/main23.rs)
#### Chapter 5: Functions
- Ex24. [Anagram Checker](src/bin/main24.rs)
- Ex25. [Password Strength Indicator](src/bin/main25.rs)
- Ex27. [Validating Inputs](src/bin/main27.rs)
#### Chapter 6: Repetition
- Ex28. [Adding Numbers](src/bin/main28.rs)
- Ex30. [Multiplication Table](src/bin/main30.rs)
- Ex32. [Guess the Number Game](src/bin/main32.rs)
#### Chapter 7: Data Structures
- Ex33. [Magic 8 Ball](src/bin/main33.rs)
- Ex35: [Picking a Winner](src/bin/main35.rs)
- Ex37. [Password Generator](src/bin/main37.rs)
- Ex39: [Sorting Records](src/bin/main39.rs)
- Ex40. [Filtering Records](src/bin/main40.rs)
#### Chapter 8: Working with Files
- Ex41. [Name Sorter](src/bin/main41.rs)
- Ex43. [Website Generator](src/bin/main43.rs)
- Ex44: [Product Search](src/bin/main44.rs)
- Ex45: [Word Finder](src/bin/main45.rs)
- Ex46. [Word Frequency Finder](src/bin/main46.rs)
#### Chapter 9: Working with External Services
- Ex47. [Who’s in Space?](src/bin/main47.rs)
- Ex48. [Grabbing the Weather](src/bin/main48.rs)
- Ex49. [Flickr Photo Search](src/bin/main49.rs)
- Ex50. [Movie Recommendations](src/bin/main50.rs)
- Ex51: [Pushing Notes to Firebase](src/bin/main51.rs)
- Ex52. [Creating Your Own Time Service](src/bin/main52.rs)
#### Chapter 10: Full Programs
- Ex53. [Todo List](src/bin/main53.rs)
- Ex54. [URL Shortener](src/bin/main54.rs)
- Ex55. [Text Sharing](src/bin/main55.rs)
- Ex56. [Tracking Inventory](src/bin/main56.rs)
- Ex57. [Trivia App](src/bin/main57.rs)

## Technologies Used

- rust 1.85.1, cargo 1.85.1
- [libraries](Cargo.toml): [chrono](https://docs.rs/chrono/latest/chrono/), [once_cell](https://docs.rs/once_cell/latest/once_cell/), [regex](https://docs.rs/regex/latest/regex/), [rand](https://rand/docs.rs/latest/rand/), [itertools](https://docs.rs/itertools/latest/itertools/), [reqwest](https://docs.rs/reqwest/latest/reqwest/), [tokio](https://docs.rs/tokio/latest/tokio/), [serde](https://docs.rs/serde/latest/serde/), [serde_json](https://docs.rs/serde_json/latest/serde_json/), [actix-web](https://docs.rs/actix-web/latest/actix-web/), [redis](https://docs.rs/redis/latest/redis/), [tera](https://docs.rs/tera/latest/tera/), [mongodb](https://docs.rs/mongodb/latest/mongodb), [md-5](https://docs.rs/md-5/latest/md-5), [uuid](https://docs.rs/uuid/latest/uuid), [url](https://docs.rs/url/latest/url), [thiserror](https://docs.rs/thiserror/latest/thiserror), [clap](https://docs.rs/clap/latest/clap), [anyhow](https://docs.rs/anyhow/latest/anyhow), [proptest](https://docs.rs/proptest/latest/proptest)

## How to Run
Run the following directly under the project.
```
$ cargo run --bin main[nn]
```
### Ex45: Word Finder
`cargo run --bin main45 -- -o [output file name]`

### Ex48: Grabbing the Weather
- Register with OpenWeather and obtain an API key (no subscription required).
- Copy config template: `cp config/ex48_config.json.template config/ex48_config.json`
- Replace placeholders in config/ex51_config.json with API key
### Ex50: 50	Movie Recommendations
- PREREQUISITE: Register with OMDb and obtain an API key.
- `$ export API_KEY=<your api key>`

### Ex51: Pushing Notes to Firebase
#### Prerequisites
- A configured Firebase project
  - Ability to obtain an ID token as an Email/Password user
  - Access via a REST client (e.g., `curl`, Postman) using the ID token
#### Setup
  - Copy the template: `cp config/ex51_config.json.template config/ex51_config.json`
  - Replace placeholders in config/ex51_config.json with actual values.
#### Caution
The ID token is stored in output/id_token.txt. Keep it secure.

### Ex52: Creating Your Own Time Service
1. Run `cargo run --bin main52`.
2. From another terminal, run cargo run --bin main52_client.

### Ex53: Todo List
Start Redis in your local environment with the command below, then run cargo run --bin main53:
```
$ docker run --name redis-local -d -p 6379:6379 redis
```
### Ex55: Text Sharing
1. Prepare MongoDB
```
$ docker pull mongo
$ docker run --name mongodb-local -d -p 27017:27017 mongo
```
2. Run `cargo run --bin main55`
3. Open ``http://127.0.0.1:8080/ex55` in your browser.

### Ex56: Tracking Inventory
1. Run `cargo run --bin main56`.
2. Open `http://127.0.0.1:8080/ex56` in your browser.
3. Add inventory items.
4. The `data/ex56_inventory.json` file will update with each addition.

## Notes
- I relied on Vibe Coding just a little bit.

## References
- [Exercises for Programmers](https://www.oreilly.com/library/view/exercises-for-programmers/9781680501513/)
- [Rust](https://www.rust-lang.org/)
