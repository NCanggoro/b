#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use serde::{Serialize, Deserialize};

enum Status {
    Ongoing,
    Done,
}

#[derive(Serialize, Deserialize)]
struct Todos {
    ongoing: Vec<String>,
    done: Vec<String>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet() -> Result<Todos, String> {
    let todo_list = read_todo_file();
    if let Ok(todo_list) = todo_list {
        return Ok(todo_list);
    } else {
        return Err("Failed".into());
    }
}

#[tauri::command]
fn add_todo(todo: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("../todo")
        .unwrap();
    if let Err(e) = writeln!(file, "{}", format!("ongoing: {todo}\n")) {
        format!("{e}");
    }
    Ok(())
}

#[tauri::command]
fn edit_todo(todos: Todos) -> Result<(), String> {
    let mut ongoing: Vec<String> = todos
        .ongoing
        .iter()
        .map(|x| format!("ongoing: {}\n", x))
        .collect();

    let done: Vec<String> = todos
        .done
        .iter()
        .map(|x| format!("done: {}\n", x))
        .collect();

    ongoing.extend(done);
    let todo_list = ongoing.iter().map(|x| x.as_bytes()).collect::<Vec<_>>();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("../todo")
        .unwrap();

    for item in &todo_list {
        file.write_all(item).expect("Unable to write data");
    }

    Ok(())
}

#[tauri::command]
fn delete_todo(todos: Todos) -> Result<(), String> {
    let mut ongoing: Vec<String> = todos
        .ongoing
        .iter()
        .map(|x| format!("ongoing: {}\n", x))
        .collect();

    let done: Vec<String> = todos
        .done
        .iter()
        .map(|x| format!("done: {}\n", x))
        .collect();

    ongoing.extend(done);
    let todo_list = ongoing.iter().map(|x| x.as_bytes()).collect::<Vec<_>>();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("../todo")
        .unwrap();

    for item in &todo_list {
        file.write_all(item).expect("Unable to write data");
    }

    Ok(())
}

#[tauri::command]
fn done_todo(lists: Vec<String>, index: usize, title: &str) -> Result<(), String> {
    let mut todo_list = read_todo_file().unwrap();

    let mut ongoing: Vec<String> = todo_list
        .ongoing
        .iter()
        .map(|x| format!("ongoing: {}\n", x))
        .collect();

    let done: Vec<String> = todo_list
        .done
        .iter()
        .map(|x| format!("done: {}\n", x))
        .collect();

    let todo = ongoing
        .iter()
        .enumerate()
        .filter(|&(i, _)| i == index)
        .map(|(_, v)| format!("done: {}", v))
        .collect::<Vec<_>>();

    let todo: Vec<&str> = todo.iter().map(AsRef::as_ref).collect();
    std::mem::replace(
        &mut ongoing[index],
        todo[0].to_string().replace(" ongoing:", ""),
    );
    ongoing.extend(done);
    let todo_list = ongoing.iter().map(|x| x.as_bytes()).collect::<Vec<_>>();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("../todo")
        .unwrap();

    for item in &todo_list {
        file.write_all(item).expect("Unable to write data");
    }
    Ok(())
}

fn read_todo_file() -> Result<Todos, String> {
    let mut todos = Vec::<String>::new();
    let mut dones = Vec::<String>::new();
    let file_path = "../todo";

    load_files(&mut todos, &mut dones, &file_path);
    let todo_list = Todos {
        ongoing: todos,
        done: dones,
    };
    Ok(todo_list)
}

fn load_files(todos: &mut Vec<String>, dones: &mut Vec<String>, file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    for (index, line) in io::BufReader::new(file).lines().enumerate() {
        match parse_item(&line?) {
            Some((Status::Ongoing, title)) => todos.push(title.to_string()),
            Some((Status::Done, title)) => dones.push(title.to_string()),
            None => { }
        }
    }
    Ok(())
}

fn parse_item(line: &str) -> Option<(Status, &str)> {
    let ongoing_item = line
        .strip_prefix("ongoing: ")
        .map(|title| (Status::Ongoing, title));
    let done_item = line
        .strip_prefix("done: ")
        .map(|title| (Status::Done, title));
    ongoing_item.or(done_item)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            add_todo,
            edit_todo,
            delete_todo,
            done_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }
