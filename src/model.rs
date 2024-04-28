// In model.rs
#[derive(Debug)]
pub struct Employee {
    pub id: i64,
    pub name: String,
    pub department: String,
    pub job_title: String,
}

#[derive(Debug)]
pub struct EvaluationCriterion {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub weightage: f64,
}

#[derive(Debug)]
pub struct EvaluationScore {
    pub id: i64,
    pub employee_id: i64,
    pub criterion_id: i64,
    pub score: f64,
}

