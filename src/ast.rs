#[derive(Debug)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: String,
    pub constraints: Vec<String>,
}

#[derive(Debug)]
pub enum Statement {
    CreateTable {
        name: String,
        columns: Vec<ColumnDef>,
    },
}
