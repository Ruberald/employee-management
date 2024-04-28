mod model;

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
        format!("INSERT INTO employees (name, department, job_title) VALUES ({}, {}, {})", 
        &employee.name, &employee.department, &employee.job_title)
    )?;
    Ok(())
}

// Function to insert a new evaluation criterion into the database
fn insert_evaluation_criterion(conn: &Connection, criterion: &EvaluationCriterion) -> Result<()> {
    conn.execute(
        format!("INSERT INTO evaluation_criteria (name, description, weightage) VALUES ({}, {}, {})",
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

fn main() -> Result<()> {
    // Connect to the SQLite database
    let conn = Connection::open("employee_performance.db")?;

    // Create tables
    create_tables(&conn)?;

    // Insert sample data
    let employee = Employee {
        id: 1,
        name: "John Doe".to_string(),
        department: "Engineering".to_string(),
        job_title: "Software Engineer".to_string(),
    };
    insert_employee(&conn, &employee)?;

    let criterion = EvaluationCriterion {
        id: 1,
        name: "Quality of Work".to_string(),
        description: "Ability to produce high-quality work".to_string(),
        weightage: 0.5,
    };
    insert_evaluation_criterion(&conn, &criterion)?;

    let score = EvaluationScore {
        id: 1,
        employee_id: 1,
        criterion_id: 1,
        score: 4.5,
    };
    insert_evaluation_score(&conn, &score)?;

    println!("Data inserted successfully");

    Ok(())
}
