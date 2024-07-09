// File containing all SQL Commands

pub const CREATE_TABLE: &str = "CREATE TABLE list (
                row INTEGER,
                description VARCHAR(100),
                environment VARCHAR(50),
                status BOOLEAN
    )";

pub const INSERT_INTO_TABLE: &str = "INSERT INTO list (row, description, environment, status) VALUES (?1, ?2, ?3, ?4)";