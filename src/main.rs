use std::io::{self, Write};

struct Task {
    description : String,
    done : bool
}

impl Task {
    fn new(description : &str) -> Task {
        Task {
            description : description.to_string(),
            done : false
        }
    }
}

struct TodoApp {
    tasks : Vec<Task>
}

impl TodoApp {
    fn new() -> TodoApp {
        TodoApp {
            tasks : Vec::new()
        }
    }

    fn add_new_task(&mut self, description : &str){
        let task = Task::new(description);
        self.tasks.push(task);
    }

    fn mark_task_as_done(&mut self, index : usize){
        if let Some(task) = self.tasks.get_mut(index) {
            task.done = true;
        }
    }

    fn show_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[X]" } else { "[]" };
            println!("{} {} : {}", index + 1, task.description, status);
        }
    }
}

fn main(){
    let mut todoApp = TodoApp::new();

    loop {
        println!();
        println!("1. Add new Task");
        println!("2. Mark Task As Done");
        println!("3. Show Tasks");
        println!("4. Exit");
        println!("--------------------");
        println!();

        let choice = match get_numeric_input("Enter your choice: ") {
        
        Some(value) => value,
        None => {
            println!("Invalid input, enter a valid number");
            continue;
        }
    };

    match choice {
        1 => {
            let description = get_string_input("Enter task description: ");
            todoApp.add_new_task(&description);
        }
        2 => {
            let index = match get_numeric_input("Enter the task index to mark as done: "){
                Some(value) => value as usize,
                None => {
                    println!("Invalid input, enter a valid number");
                    continue;
                }
            };
            todoApp.mark_task_as_done(index - 1);
        }
        3 => {
            todoApp.show_tasks();
        }
        4 => {
            break;
        }
        _ => println!("Invalid option, please input a number between 1 and 4")
    };
}
}


fn get_string_input(prompt: &str) -> String {
    print!("{}", prompt);

    std::io::stdout().flush().ok();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read the line");
    input.trim().to_string()
}

fn get_numeric_input(prompt: &str) -> Option<u8> {
    print!("{}", prompt);

    std::io::stdout().flush().ok();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read the line");
    match input.trim().parse() {
        Ok(value) => Some(value) ,
        Err(_) => {
            println!("Invalid input, enter proper number");
            None
        }
    }
}