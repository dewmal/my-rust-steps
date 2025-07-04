use std::fmt::Display;
use std::io;
use std::str::FromStr;

#[repr(u8)]
#[derive(Debug, Clone)]
enum BatchStatus {
    Active = 1,
    Inactive = 0,
}
#[derive(Debug, PartialEq, Eq)]
struct ParseBatchStatusError;
impl FromStr for BatchStatus {
    type Err = ParseBatchStatusError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "1" => Ok(BatchStatus::Active),
            "0" => Ok(BatchStatus::Inactive),
            _ => Err(ParseBatchStatusError),
        }
    }
}

impl Display for BatchStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatchStatus::Active => write!(f, "Active"),
            BatchStatus::Inactive => write!(f, "Inactive"),
        }
    }
}
#[derive(Debug, Clone)]
struct Batch {
    name: String,
    status: BatchStatus,
}

fn main() {
    let mut batch_details: Vec<Batch> = vec![];

    // Main Menu Loop
    loop {
        let main_choice = print_main_menu();
        if main_choice == "EXIT" {
            break;
        }

        println!("Choice: {} {}", main_choice, main_choice.as_str() == "1");
        // Sub Menu Loop
        loop {
            let sub_choice = match main_choice.as_str() {
                "1" => print_student_menu(),
                "2" => print_batch_menu(),
                _ => "EXIT".to_string(),
            };
            if sub_choice == "EXIT" {
                break;
            }

            match sub_choice.trim() {
                "1" => match main_choice.as_str() {
                    "1" => add_student(&mut batch_details),
                    "2" => add_batch(&mut batch_details),
                    _ => (),
                },
                "2" => match main_choice.as_str() {
                    "1" => println!("Update Student"),
                    "2" => update_batch(&mut batch_details),
                    _ => (),
                },
                "3" => match main_choice.as_str() {
                    "1" => println!("View Student Profile"),
                    "2" => view_batches(&batch_details),
                    _ => (),
                },
                _ => (),
            }
        }
    }
}

fn add_batch(batch_details: &mut Vec<Batch>) {
    let mut batch_name = String::new();
    let mut batch_status = String::new();

    println!("Enter batch Name : ");
    io::stdin()
        .read_line(&mut batch_name)
        .expect("Failed to read line");

    println!("Enter batch Status : ");
    io::stdin()
        .read_line(&mut batch_status)
        .expect("Failed to read line");

    let new_batch = Batch {
        name: batch_name,
        status: batch_status
            .trim()
            .parse()
            .expect("Failed to parse batch status"),
    };

    batch_details.push(new_batch);
    println!("Batch added successfully");

    println!("Do you want to add another batch? (y/n): ");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    if choice.trim() == "y" {
        add_batch(batch_details);
    }
}

fn update_batch(batch_details: &mut Vec<Batch>) {
    let mut batch_name = String::new();
    let mut batch_status = String::new();

    println!("Enter batch Name : ");
    io::stdin()
        .read_line(&mut batch_name)
        .expect("Failed to read line");

    println!("Enter batch Status : ");
    io::stdin()
        .read_line(&mut batch_status)
        .expect("Failed to read line");

    let new_batch = Batch {
        name: batch_name,
        status: batch_status.parse().expect("Failed to parse batch status"),
    };

    for batch in batch_details.iter_mut() {
        if batch.name == new_batch.name {
            *batch = new_batch.clone();
            println!("Batch updated successfully");
            return;
        }
    }

    println!("Batch not found");
}

fn view_batches(batch_details: &Vec<Batch>) {
    for batch in batch_details {
        println!("Batch Name: {} / Batch Status {}", batch.name, batch.status);
    }
}

fn add_student(batch_details: &mut Vec<Batch>) {
    let mut batch_name = String::new();
    let batch_status = String::new();
    let student_name = String::new();
    let student_id = String::new();
    let student_email = String::new();
    let student_phone = String::new();

    println!("Enter batch Number (Students should be added) : ");
    io::stdin()
        .read_line(&mut batch_name)
        .expect("Failed to read line");
}

fn print_main_menu() -> String {
    let header_count = 41;
    for _ in 0..header_count {
        print!("-");
    }
    println!("\n|\tiCET STUDENT MANAGEMENT SYSTEM\t|");

    for _ in 0..header_count {
        print!("-");
    }

    let base_commands = vec![
        "Student Management",
        "Batch Management",
        "Grade Management",
        "Report Generation",
        "Exit",
    ];
    println!();
    for (index, command) in base_commands.iter().enumerate() {
        println!("[{}] {}", index + 1, command);
    }

    println!("Enter your choice: ");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    if choice.trim() == "5" {
        return "EXIT".to_string();
    }
    choice.trim().to_string()
}

fn print_student_menu() -> String {
    let header_count = 41;
    for _ in 0..header_count {
        print!("-");
    }
    println!("\n|\tStudent Management\t|");

    for _ in 0..header_count {
        print!("-");
    }

    let base_commands = vec![
        "Add Student",
        "Update Student",
        "View Student Profile",
        "Delete Student Profile",
        "Exit",
    ];
    println!();
    for (index, command) in base_commands.iter().enumerate() {
        println!("[{}] {}", index + 1, command);
    }

    println!("Enter your choice: ");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    if choice.trim() == "5" {
        return "EXIT".to_string();
    }

    choice
}

fn print_batch_menu() -> String {
    let header_count = 41;
    for _ in 0..header_count {
        print!("-");
    }
    println!("\n|\tBatch Management\t|");

    for _ in 0..header_count {
        print!("-");
    }

    let base_commands = vec!["Add Batch", "Update Batch", "View Batches", "Exit"];
    println!();
    for (index, command) in base_commands.iter().enumerate() {
        println!("[{}] {}", index + 1, command);
    }

    println!("Enter your choice: ");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    if choice.trim() == "4" {
        return "EXIT".to_string();
    }

    choice
}
