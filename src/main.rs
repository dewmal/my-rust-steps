use std::fmt::{Display, write};
use std::io;
use std::io::{BufRead, Write};
use std::str::FromStr;

#[repr(u8)]
#[derive(Debug, Clone, PartialOrd, PartialEq)]
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
            "Active" => Ok(BatchStatus::Active),
            "Inactive" => Ok(BatchStatus::Inactive),
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

#[repr(u8)]
#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum StudentEnrollmentType {
    Online = 1,
    Physical = 0,
}

impl Display for StudentEnrollmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StudentEnrollmentType::Online => write!(f, "Online"),
            StudentEnrollmentType::Physical => write!(f, "Physical"),
        }
    }
}

impl FromStr for StudentEnrollmentType {
    type Err = ParseBatchStatusError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "1" => Ok(StudentEnrollmentType::Online),
            "0" => Ok(StudentEnrollmentType::Physical),
            "Online" => Ok(StudentEnrollmentType::Online),
            "Physical" => Ok(StudentEnrollmentType::Physical),
            _ => Err(ParseBatchStatusError),
        }
    }
}
#[derive(Debug, Clone)]
struct Student {
    id: String,
    name: String,
    nic: String,
    batch: String,
    enrollment_type: StudentEnrollmentType,
}

impl Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(
            f,
            format_args!(
                "ID: {}\nName: {}\nNIC: {}\nBatch: {}\nEnrollment Type: {}\n",
                self.id, self.name, self.nic, self.batch, self.enrollment_type
            ),
        )
    }
}

fn main() {
    let mut batch_details: Vec<Batch> = vec![];
    let mut student_details: Vec<Student> = vec![];

    load_batch(&mut batch_details);

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
                    "1" => add_student(&mut batch_details, &mut student_details),
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

            save_batch(&batch_details);
            save_student(&student_details);
        }
    }
}

fn save_student(student_details: &Vec<Student>) {
    // Save student details to a file
    println!("Saving student details to a file");
    let mut file = std::fs::File::create("student_details.txt").unwrap();
    for student in student_details {
        let line_to_write = format!(
            "{},{},{},{},{}\n",
            student.id, student.name, student.nic, student.batch, student.enrollment_type
        );
        file.write_all(line_to_write.as_bytes()).unwrap();
    }

    println!("Student details saved to a file");
}

fn save_batch(batch_details: &Vec<Batch>) {
    // Save batch details to a file
    println!("Saving batch details to a file");

    let mut file = std::fs::File::create("batch_details.txt").unwrap();
    for batch in batch_details {
        let line_to_write = format!("{},{}\n", batch.name, batch.status);
        file.write_all(line_to_write.as_bytes()).unwrap();
    }

    println!("Batch details saved to a file");
}

fn load_batch(original_batch_details: &mut Vec<Batch>) {
    // Load batch details from a file
    let mut batch_details: Vec<Batch> = vec![];
    let file = match std::fs::File::open("batch_details.txt") {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);
            for line in reader.lines() {
                let line = line.unwrap();
                let mut batch = line.split(",");
                let name = batch.next().unwrap().to_string();
                let status = batch.next().unwrap().parse::<BatchStatus>().unwrap();
                batch_details.push(Batch { name, status });
            }
            *original_batch_details = batch_details;
        }
        Err(e) => println!("Error opening file: {}", e),
    };
}

fn add_batch(batch_details: &mut Vec<Batch>) {
    let mut batch_name = String::new();
    let mut batch_status = String::new();

    print!("Enter batch Name : ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut batch_name)
        .expect("Failed to read line");

    print!("Enter batch Status : ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut batch_status)
        .expect("Failed to read line");

    let new_batch = Batch {
        name: batch_name.trim().to_string(),
        status: batch_status
            .trim()
            .parse()
            .expect("Failed to parse batch status"),
    };

    batch_details.push(new_batch);
    println!("Batch added successfully");

    print!("Do you want to add another batch? (y/n): ");
    io::stdout().flush().unwrap();
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

    print!("Enter batch Name : ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut batch_name)
        .expect("Failed to read line");

    print!("Enter batch Status : ");
    io::stdout().flush().unwrap();
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

fn add_student(batch_details: &mut Vec<Batch>, student_details: &mut Vec<Student>) {
    let mut batch_name = String::new();
    let mut student_enrollment_type_id = String::new();
    let mut student_name = String::new();
    let mut student_id = String::new();
    let mut student_nic = String::new();

    print!("Enter batch Number (Students should be added) : ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut batch_name)
        .expect("Failed to read line");

    let mut selected_batch: Option<Batch> = None;
    let batch_details_copy = batch_details.clone();
    for batch in batch_details_copy {
        if batch.name == batch_name.trim() && batch.status == BatchStatus::Active {
            selected_batch = Some(batch.clone());
            break;
        }
    }

    match selected_batch {
        Some(batch) => {
            print!("\nEnter student Name : ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut student_name)
                .expect("Failed to read line");

            print!("\nEnter student Enrollment Type (Online =1 /Physical = 0) : ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut student_enrollment_type_id)
                .expect("Failed to read line");

            let student_enrollment_type: StudentEnrollmentType = student_enrollment_type_id
                .trim()
                .parse()
                .expect("Failed to parse student enrollment type");

            student_id += match student_enrollment_type {
                StudentEnrollmentType::Online => "OR",
                StudentEnrollmentType::Physical => "PR",
            };

            student_id += batch_name.trim();
            student_id += &format!("{}", student_details.len() + 1);

            println!("Student ID: {}", student_id);

            print!("\nEnter student NIC : ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut student_nic)
                .expect("Failed to read line");

            let new_student = Student {
                name: student_name,
                enrollment_type: student_enrollment_type,
                id: student_id,
                nic: student_nic,
                batch: batch.name,
            };
            println!("Student: {}", new_student);
            student_details.push(new_student);
            println!("Student added successfully");
            print!("Do you want to add another student? (y/n): ");
            io::stdout().flush().unwrap();
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            if choice.trim() == "y" {
                add_student(batch_details, student_details);
            }
        }
        None => {
            println!("Batch not found");
            print!("Do you want to try again (y/n): ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");
            if choice.trim() == "y" {
                add_student(batch_details, student_details);
            }
        }
    }
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

    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
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

    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

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
