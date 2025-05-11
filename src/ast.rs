#[derive(Debug)]
pub struct SelectStatement {
    pub columns: Vec<String>,
    pub table: String,
    pub condition: Option<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Equals(String, i64),
}
