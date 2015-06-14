extern crate rustc_serialize;
extern crate time;
extern crate getopts;

use getopts::Options;
use std::env;
use std::process::exit;
mod abuddy;
use abuddy::Abuddy;

fn print_usage(opts: Options) {
  let brief = format!("Usage: {} [options]", "accountabilibuddy");
  print!("{}", opts.usage(&brief));
}

fn main() {

    let argv: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    
    let home = match env::home_dir() {
        Some(p) => p,
        None => panic!("Couldn't find a home dir."),
    };

    let save_file = home.to_str().unwrap().to_string() + "/.accountabilibuddy";

    opts.optflag("a", "add", "Add a new todo.");
    opts.optflag("h", "help", "Show this menu");
    opts.optflag("t", "add-task", "Add a task");
    opts.optflag("s", "set-done", "Set todo or task to done");
    opts.optflag("d", "delete", "Delete todo or task");

    let valid_options = match opts.parse(&argv[1..]) {
      Ok(option) => { option },
      Err(err) => { panic!("{}", err) },
    };

    if valid_options.opt_present("h") {
      print_usage(opts);
      exit(0);
    }

    let mut abuddy = Abuddy::from_file(&save_file);

    if valid_options.opt_present("a") {
      abuddy.add_todo(&argv[2]);
      abuddy.save();
    }

    if valid_options.opt_present("t") {

      let id: u32 = match argv[2].trim().parse() {
        Ok(num) => num,
        Err(_) => exit(1),
      };

      abuddy.add_task(id, &argv[3]);

      abuddy.save();
    }

    if valid_options.opt_present("s") {
      
      let id: u32 = match argv[2].trim().parse() {
        Ok(num) => num,
        Err(_) => exit(1),
      };

      match argv[3].trim().parse() {
        Ok(num) => abuddy.toggle_task_done(id, num),
        Err(_) => abuddy.toggle_done(id),
      };

      abuddy.save();
    }

    if valid_options.opt_present("d") {

      let id: u32 = match argv[2].trim().parse() {
        Ok(num) => num,
        Err(_) => exit(1),
      };

      if argv.len() > 3 {
        match argv[3].trim().parse() {
          Ok(task_id) => abuddy.delete_task(id, task_id),
          Err(_) => exit(1),
        };
      } else {
        abuddy.delete_todo(id);
      }
      
      abuddy.save();
    }

    abuddy.print();
}
