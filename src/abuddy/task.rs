use std::fmt;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Task {
    pub id: u32,
    message: String,
    done: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let checkbox = if self.done { "☑" } else { "☐" };
        write!(f, "    {id:>0$}. {message:<97} {checkbox}", 2, id=self.id, checkbox=checkbox, message=self.message)
    }
}

impl Task {
    pub fn new(id: u32, message: &str) -> Task {
        Task {
            message: message.to_string(),
            done: false,
            id: id,
        }
    }

    pub fn toggle_done(&mut self) {
        self.done = !self.done;
    }
}