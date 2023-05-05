use std::{collections::HashMap, io};

mod company;
use company::Company;

enum Command {
    Add { user: String, department: String },
    GetPeopleDepartment(String),
    GetAllPeopleByDepartment,
    NoOp,
}

fn main() {
    let mut company = Company::new();
    handle_input(&mut company);
}

fn handle_input(company: &mut Company) {
    let mut user_input = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut user_input) {
        Ok(_n) => {
            let sanitized_user_input = user_input.trim();
            if sanitized_user_input.to_lowercase() != "quit" {
                execute_command(sanitized_user_input, company);
                handle_input(company)
            }
            println!("Bye")
        }
        Err(error) => println!("error: {error}"),
    }
}

fn execute_command(command_text: &str, company: &mut Company) {
    let cmd: Command = parse(command_text);
    match cmd {
        Command::Add { user, department } => company.add_user_to_department(&user, &department),
        Command::GetPeopleDepartment(department) => {
            let people_department = company.get_people_department(&department);
            print_people_department(&department, people_department)
        }
        Command::GetAllPeopleByDepartment => {
            let people_by_dep = company.get_all_people_by_department();
            print_all_people_by_dep(people_by_dep)
        }
        Command::NoOp => (),
    }
}

fn parse(command_text: &str) -> Command {
    let sanitized_cmd = command_text.trim().to_lowercase();
    let cmd_items: Vec<&str> = command_text.split_whitespace().collect();
    if sanitized_cmd.starts_with("add") {
        Command::Add {
            user: cmd_items[1].to_string(),
            department: cmd_items[3].to_string(),
        }
    } else if sanitized_cmd.starts_with("list dep") {
        Command::GetPeopleDepartment(cmd_items[2].to_string())
    } else if sanitized_cmd.starts_with("list company") {
        Command::GetAllPeopleByDepartment
    } else {
        Command::NoOp
    }
}

fn print_people_department(department: &str, people_department: &Vec<String>) {
    println!("");
    println!("-------------------------");
    println!("Department: {department}");
    println!("-------------------------");
    for user in people_department {
        println!("{user}")
    }
    println!("");
}

fn print_all_people_by_dep(people_by_dep: &HashMap<String, Vec<String>>) {
    for (dep, people) in people_by_dep {
        print_people_department(dep, people)
    }
}
