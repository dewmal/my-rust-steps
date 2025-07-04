use std::io;

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

    choice
}

fn print_student_menu() -> String {
    let header_count = 41;
    for _ in 0..header_count {
        print!("-");
    }
    println!("\n|\tiCET STUDENT MANAGEMENT SYSTEM\t|");

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

    choice
}

fn main() {
    let choice = print_main_menu();
    let sub_choice = match choice.trim().parse::<usize>() {
        Ok(choice) => match choice {
            1 => print_student_menu(),
            // 2 => println!("You selected Batch Management"),
            // 3 => println!("You selected Grade Management"),
            // 4 => println!("You selected Report Generation"),
            // 5 => println!("You selected Exit"),
            _ => "-1".to_string(),
        },
        Err(e) => {
            println!("Invalid input: {}", e);
            "-1".to_string()
        }
    };

    match sub_choice.trim().parse::<usize>() {
        Ok(choice) => match choice {
            1 => println!("You selected Add Student"),
            // 2 => println!("You selected Update Student"),
            // 3 => println!("You selected Delete Student"),
            // 4 => println!("You selected Display Student"),
            // 5 => println!("You selected Exit"),
            _ => println!("Invalid choice"),
        },
        Err(e) => println!("Invalid input: {}", e),
    }
}
