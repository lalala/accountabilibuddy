use std::fmt;
use time;

use super::Task;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Todo {
  pub id: u32,
  message: String,
  done: bool,
  created_at: u64,
  tasks: Vec<Task>,
  count: u32,
}


impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let time = time::at(time::Timespec::new(self.created_at as i64, 0));
        let formatted_time = time::strftime("%Y-%m-%d", &time).unwrap();
        
        let checkbox = if self.done { "☑" } else { "☐" };
        let mut tasks_string: String = "".to_string();
        // let mut other_string:String = "".to_string();

        for task in self.tasks.iter() {
            tasks_string = tasks_string + &format!("{}\n", task);
        }

        write!(f, "{id:>0$}. {message:<80} (added {time}) {checkbox}\n{tasks}", 4, checkbox=checkbox, id=self.id, time=formatted_time, message=self.message, tasks=tasks_string)
    }
}

impl Todo {

    pub fn new(id: u32, message: &str, created_at: time::Tm, tasks: Vec<Task>) -> Todo {
        let created_at: u64 = created_at.to_timespec().sec as u64;
        Todo {
            id: id,
            message: message.to_string(),
            created_at: created_at,
            done: false,
            tasks: tasks,
            count: 0,
        }
    }

    pub fn add_task(&mut self, message: &str) {
        self.count += 1;
        self.tasks.push(Task::new(self.count, message));
    }

    pub fn toggle_done(&mut self) {
        self.done = !self.done;
    }
    
    pub fn toggle_task_done(&mut self, task_id: u32) {
        for task in self.tasks.iter_mut() {
            if task.id == task_id {
                task.toggle_done();
            }
        }
    }

    pub fn delete_task(&mut self, id: u32) {
        self.tasks.retain(|task| task.id != id);
    }
    // pub fn tasks(&self) -> Vec<Task> {
    //   self.tasks
    // }
    // pub fn add_task(&mut self, task: &mut Task) {
    //     self.len = self.len + 1;
    //     task.set_id(self.len);
    //     let task = task.clone();
    //     self.sub_tasks.push(task);
    // }

    // pub fn has_tasks(&self) -> bool {
    //     self.tasks.len() > 0
    // }

    // pub fn set_id(&mut self, id: u32) {
    //     self.id = id;
    // }
}