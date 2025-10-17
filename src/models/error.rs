use std::io::{self};

#[derive(Debug, thiserror::Error)]
pub enum CommandArgsError {
    #[error("Missing <database path> and <command>")]
    MissingArgs,
    #[error("Missing <command>")]
    MissingCommand,
    #[error("Missing or invalid command passed: {0}")]
    InvalidCommand(String),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error), // automatically adds From<io::Error>
}

#[derive(Debug, thiserror::Error)]
pub enum SQLCommandError {
    #[error("No table named \"{0}\" found")]
    UnknownTable(String),
    
    #[error("SQL Command: \"{0}\" is not supported.")]
    UnsupportedCommand(String)
}


#[derive(Debug, thiserror::Error)]
pub enum SQLSyntaxError {
    #[error("SQL Syntax Error: Unexpected token \"{0}\".")]
    UnexpectedToken(String),
    
    #[error("SQL Syntax Error: Value is unsupported \"{0}\".")]
    UnsupportedValue(String)

}

#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error(transparent)]
    Args(#[from] CommandArgsError),

    #[error(transparent)]
    Sql(#[from] SQLCommandError),
}

