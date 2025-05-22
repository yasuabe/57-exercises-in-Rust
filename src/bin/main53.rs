/* ---------------
# Ex53: Todo List
------------------
- Command-line todo list app.
- Store data in Redis.

## Commands:
- add: <task>  – Add a new task (empty tasks are not allowed)
- list         – Show all tasks
- remove: <id> – Remove completed task by ID
- exit         – Quit the app
*/
use redis::Commands;
use exercises_for_programmer::utils::std_util::read_input;

enum Command<'a> {
    NoInput,
    Add(String),
    List,
    Remove(i32),
    Exit,
    Error(&'a str),
}
fn add_task(con: &mut redis::Connection, task: &str) -> redis::RedisResult<()> {
    let id: i32 = con.incr("task", 1)?;
    let _: ()   = con.hset("tasks", id, task)?;
    Ok(())
}
fn list_tasks(con: &mut redis::Connection) -> redis::RedisResult<()> {
    let val: Vec<(String, String)> = con.hgetall("tasks")?;
    for (id, task) in val {
        println!("{}: {}", id, task);
    }
    Ok(())
}
fn remove_task(con: &mut redis::Connection, id: i32) -> redis::RedisResult<()> {
    let _: () = con.hdel("tasks", id)?;
    Ok(())
}
fn parse_input(input: &str) -> Command {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Command::NoInput
    } else if trimmed == "exit" {
        Command::Exit
    } else if trimmed == "list" {
        Command::List
    } else if let Some(task) = trimmed.strip_prefix("add:").map(str::trim) {
        match task {
            "" => Command::Error("Task cannot be empty"),
            _  => Command::Add(task.to_string()),
        }
    } else if let Some(id_str) = trimmed.strip_prefix("remove:").map(str::trim) {
        match id_str.parse::<i32>() {
            Ok(id) => Command::Remove(id),
            _      => Command::Error("ID must be a number"),
        }
    } else {
        Command::Error("Unknown command")
    }
}
fn connect_to_redis() -> redis::RedisResult<redis::Connection> {
    redis::Client::open("redis://127.0.0.1:6379/")?.get_connection()
}
fn run_prompt_loop(con: &mut redis::Connection) -> redis::RedisResult<()> {
    loop {
        match parse_input(&read_input("> ")) {
            Command::NoInput    => continue,
            Command::Add(task)  => add_task(con, &task)?,
            Command::List       => list_tasks(con)?,
            Command::Remove(id) => remove_task(con, id)?,
            Command::Error(msg) => println!("{}", msg),
            Command::Exit       => break,
        }
    }
    Ok(())
}
fn main() -> redis::RedisResult<()> {
    let mut con = connect_to_redis()?;
    run_prompt_loop(&mut con)?;
    Ok(())
}
