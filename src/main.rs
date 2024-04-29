mod model;

use std::error::Error;

use sqlite::{Connection, Result};

fn create_tables(conn: &Connection) -> Result<()> {
    // Create the employees table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS employees (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            department TEXT NOT NULL,
            job_title TEXT NOT NULL
        )",
    )?;

    // Create the evaluation_criteria table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS evaluation_criteria (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            weightage REAL NOT NULL
        )",
    )?;

    // Create the evaluation_scores table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS evaluation_scores (
            id INTEGER PRIMARY KEY,
            employee_id INTEGER NOT NULL,
            criterion_id INTEGER NOT NULL,
            score REAL NOT NULL,
            FOREIGN KEY (employee_id) REFERENCES employees(id),
            FOREIGN KEY (criterion_id) REFERENCES evaluation_criteria(id)
        )",
    )?;

    Ok(())
}


// Define your data models
use model::{Employee, EvaluationCriterion, EvaluationScore};

// Function to insert a new employee into the database
fn insert_employee(conn: &Connection, employee: &Employee) -> Result<()> {
    conn.execute(
        format!("INSERT INTO employees (name, department, job_title) VALUES ('{}', '{}', '{}')", 
        employee.name, employee.department, employee.job_title)
    )?;
    Ok(())
}

// Function to insert a new evaluation criterion into the database
fn insert_evaluation_criterion(conn: &Connection, criterion: &EvaluationCriterion) -> Result<()> {
    conn.execute(
        format!("INSERT INTO evaluation_criteria (name, description, weightage) VALUES ('{}', '{}', {})",
        criterion.name, criterion.description, criterion.weightage)
    )?;
    Ok(())
}

// Function to insert a new evaluation score into the database
fn insert_evaluation_score(conn: &Connection, score: &EvaluationScore) -> Result<()> {
    conn.execute(
        format!("INSERT INTO evaluation_scores (employee_id, criterion_id, score) VALUES ({}, {}, {})",
        score.employee_id, score.criterion_id, score.score)
    )?;
    Ok(())
}

// Function to retrieve all employees from the database
fn get_employees(conn: &Connection) -> Result<Vec<Employee>> {
    let mut stmt = conn.prepare("SELECT id, name, department, job_title FROM employees")?;
    let mut employees = Vec::new();

    for row in stmt
        .into_iter()
        .map(|row| row.unwrap()) {
            employees.push ( Employee { 
                id: row.read::<i64, _>("id"),
                name: row.read::<&str, _>("name").to_owned(), 
                department: row.read::<&str, _>("department").to_owned(), 
                job_title: row.read::<&str, _>("job_title").to_owned() 
            }
            );
        }

    Ok(employees)
}

// Function to retrieve all evaluation criteria from the database
fn get_evaluation_criteria(conn: &Connection) -> Result<Vec<EvaluationCriterion>> {
    let mut stmt = conn.prepare("SELECT id, name, description, weightage FROM evaluation_criteria")?;
    let mut criteria = Vec::new();

    for row in stmt.into_iter().map(|row| row.unwrap()) {
        criteria.push(EvaluationCriterion {
            id: row.read::<i64, _>("id"),
            name: row.read::<&str, _>("name").to_owned(),
            description: row.read::<&str, _>("description").to_owned(),
            weightage: row.read::<f64, _>("weightage"),
        });
    }

    Ok(criteria)
}

// Function to retrieve all evaluation scores from the database
fn get_evaluation_scores(conn: &Connection) -> Result<Vec<EvaluationScore>> {
    let mut stmt = conn.prepare("SELECT id, employee_id, criterion_id, score FROM evaluation_scores")?;
    let mut scores = Vec::new();

    for row in stmt.into_iter().map(|row| row.unwrap()) {
        scores.push(EvaluationScore {
            id: row.read::<i64, _>("id"),
            employee_id: row.read::<i64, _>("employee_id"),
            criterion_id: row.read::<i64, _>("criterion_id"),
            score: row.read::<f64, _>("score"),
        });
    }

    Ok(scores)
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    // Connect to the SQLite database
    let conn = Connection::open("employee_performance.db")?;

    // Create tables if they don't exist
    create_tables(&conn)?;

    println!("Welcome to the Employee Evaluation System");

    loop {
        println!("\nMenu:");
        println!("1. Insert Employee");
        println!("2. Insert Evaluation Criterion");
        println!("3. Insert Evaluation Score");
        println!("4. View Employees");
        println!("5. View Evaluation Criteria");
        println!("6. View Evaluation Scores");
        println!("7. Exit");

        // Read user input
        println!("Enter your choice:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let choice: u32 = input.trim().parse().expect("Please enter a valid number");

        match choice {
            1 => {
                println!("Enter employee details:");
                println!("Name:");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name)?;
                let name = name.trim().to_string();

                println!("Department:");
                let mut department = String::new();
                std::io::stdin().read_line(&mut department)?;
                let department = department.trim().to_string();

                println!("Job Title:");
                let mut job_title = String::new();
                std::io::stdin().read_line(&mut job_title)?;
                let job_title = job_title.trim().to_string();

                let employee = Employee {
                    id: 0, // Assuming the database assigns IDs automatically
                    name,
                    department,
                    job_title,
                };
                insert_employee(&conn, &employee)?;
                println!("Employee inserted successfully");
            }
            2 => {
                println!("Enter evaluation criterion details:");
                println!("Name:");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name)?;
                let name = name.trim().to_string();

                println!("Description:");
                let mut description = String::new();
                std::io::stdin().read_line(&mut description)?;
                let description = description.trim().to_string();

                println!("Weightage:");
                let mut weightage_str = String::new();
                std::io::stdin().read_line(&mut weightage_str)?;
                let weightage: f64 = weightage_str.trim().parse().expect("Please enter a valid number");

                let criterion = EvaluationCriterion {
                    id: 0, // Assuming the database assigns IDs automatically
                    name,
                    description,
                    weightage,
                };
                insert_evaluation_criterion(&conn, &criterion)?;
                println!("Evaluation criterion inserted successfully");
            }
            3 => {
                println!("Enter evaluation score details:");
                println!("Employee ID:");
                let mut employee_id_str = String::new();
                std::io::stdin().read_line(&mut employee_id_str)?;
                let employee_id: i64 = employee_id_str.trim().parse().expect("Please enter a valid number");

                println!("Criterion ID:");
                let mut criterion_id_str = String::new();
                std::io::stdin().read_line(&mut criterion_id_str)?;
                let criterion_id: i64 = criterion_id_str.trim().parse().expect("Please enter a valid number");

                println!("Score:");
                let mut score_str = String::new();
                std::io::stdin().read_line(&mut score_str)?;
                let score: f64 = score_str.trim().parse().expect("Please enter a valid number");

                let score = EvaluationScore {
                    id: 0, // Assuming the database assigns IDs automatically
                    employee_id,
                    criterion_id,
                    score,
                };
                insert_evaluation_score(&conn, &score)?;
                println!("Evaluation score inserted successfully");
            }
            4 => {
                println!("Employees:");
                let employees = get_employees(&conn)?;
                for employee in &employees {
                    println!("{:?}", employee);
                }
            }
            5 => {
                println!("Evaluation Criteria:");
                let criteria = get_evaluation_criteria(&conn)?;
                for criterion in &criteria {
                    println!("{:?}", criterion);
                }
            }
            6 => {
                println!("Evaluation Scores:");
                let scores = get_evaluation_scores(&conn)?;
                for score in &scores {
                    println!("{:?}", score);
                }
            }
            7 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 7."),
        }
    }

    Ok(())
}
