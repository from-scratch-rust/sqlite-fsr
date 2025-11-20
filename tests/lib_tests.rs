use sqlite_fsr::run;
use sqlite_fsr::models::error::*;


#[test]
fn test_run_fails_when_missing_all_args() {
    let args = vec![String::new()];
    let result = run(&args).unwrap_err();
    assert!(matches!(result, RunError::Args(CommandArgsError::MissingArgs)));
}

#[test]
fn test_run_fails_when_missing_one_args() {
    let args = vec![String::new(), String::from("./tests/assets/sample.db")];
    let result = run(&args).unwrap_err();
    assert!( matches!(result, RunError::Args(CommandArgsError::MissingCommand)));
}

#[test]
fn test_run_fails_when_invalid_command() {
    let args = vec![String::new(), String::from("./tests/assets/sample.db"), String::from(".dbpictures")];
    let result = run(&args).unwrap_err();
    assert!( matches!(result, RunError::Args(CommandArgsError::InvalidCommand(_))));
}

#[test]
fn test_run_succeeds_when_valid_command() {
    let args = vec![String::new(), String::from("./tests/assets/sample.db"), String::from(".dbinfo")];
    let result = run(&args);
    assert!(result.is_ok());
}


#[test]
fn test_run_returns_correct_output() {
    let args = vec![String::new(), String::from("./tests/assets/sample.db"), String::from("SELECT * FROM apples;")];
    let result = run(&args).unwrap();
    assert_eq!(result, String::from("1 Granny Smith Light Green\n2 Fuji Red\n3 Honeycrisp Blush Red\n4 Golden Delicious Yellow\n"));
}


#[test]
fn test_run_fails_when_invalid_filepath() {
    let args = vec![String::new(), String::from("../tests/assets/fake/index.db"), String::from(".dbinfo")];
    let result = run(&args).unwrap_err();
    assert!(matches!(result, RunError::Args(CommandArgsError::Io(_))));
}


