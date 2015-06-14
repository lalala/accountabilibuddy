

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::string::String;
use time;

use rustc_serialize::json;
use super::Todo;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Abuddy {
    todos: Vec<Todo>,
    count: u32,
    save_file: String,
}

impl Abuddy {

    pub fn from_file(file_name: &str) -> Abuddy {
        let file = match OpenOptions::new().read(true).create(true).open(file_name) {
            Ok(file) => file,
            Err(err) => panic!("{}", err),
        };

        let mut reader = BufReader::new(&file);
        let mut program_state = &mut String::new();

        reader.read_to_string(&mut program_state)
            .ok()
            .expect("Failed to parse file.");    

        match json::decode(program_state) {
            Ok(object) => { object },
            Err(_) => {
                println!("Failed to read save file, creating.");
                Abuddy {
                    save_file: file_name.to_string(),
                    count: 0,
                    todos: vec![],
                }
            }
        }
    }

    pub fn add_todo(&mut self, message: &str) {
        self.count = self.count + 1;
        let todo = Todo::new(self.count, message, time::now(), Vec::new());
        self.todos.push(todo);

    }

    pub fn add_task(&mut self, id: u32, message: &str) {
        for todo in self.todos.iter_mut() {
            if todo.id == id {
                todo.add_task(message);
            }
        }
    }
    pub fn toggle_done(&mut self, id: u32) {
        for todo in self.todos.iter_mut() {
            if todo.id == id {
                todo.toggle_done();
            }
        }
    }
    pub fn toggle_task_done(&mut self, todo_id: u32, task_id: u32) {
        for todo in self.todos.iter_mut() {
            if todo.id == todo_id {
                todo.toggle_task_done(task_id);
            }
        }
    }

    pub fn delete_todo(&mut self, id: u32) {

        self.todos.retain(|todo| todo.id != id);
        println!("idx: {}", id);
        // self.todos.remove(idx);

    }

    pub fn delete_task(&mut self, todo_id: u32, task_id: u32) {
        let mut todo = self.todos.iter_mut().filter(|&ref todo| todo.id == todo_id).collect::<Vec<&mut Todo>>();
        todo[0].delete_task(task_id);
        // {
            // Ok(vec) => vec,
            // _ => panic!("Todo not found!"),
        // };

        // println!("{}", todo[0]);

    }

    // pub fn remove_todo(id: u32) {
    //     self.todos.
    // }

    pub fn print(&self) {
        println!("   Todos: ");
        for todo in self.todos.iter() {
            println!("{}", todo);
        }
    }

    pub fn save(&self) {

        let file = match OpenOptions::new().write(true).create(true).truncate(true).open(&self.save_file) {
            Ok(file) => file,
            Err(err) => panic!("{}", err),
        };

        let self_encoded = json::encode(&self).unwrap();
        let mut writer = BufWriter::new(&file);
        writer.write_all(self_encoded.as_bytes()).ok().expect("Failed to save")
    }
}