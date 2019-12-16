use std::{
    ffi::{OsStr, OsString},
    path::Path,
};

/// Validates that the given string is a valid and existing path.
///
/// # Example
/// ```rust
/// App::new("Example")
///     .arg(
///         Arg::with_name("input")
///             .help("The path to the input file")
///             .takes_value(true)
///             .validator_os(is_valid_path),
///     );
/// ```
pub fn is_valid_path(path: &OsStr) -> Result<(), OsString> {
    if Path::new(path).exists() {
        Ok(())
    } else {
        Err(OsString::from("The given path does not exist"))
    }
}

/// Validates that the given string is a valid `i32`.
/// 
/// # Example
/// ```rust
/// App::new("Example")
///     .arg(
///         Arg::with_name("input")
///             .help("An i32 number")
///             .takes_value(true)
///             .validator(is_valid_i32),
///     );
/// ```
pub fn is_valid_i32(input: String) -> Result<(), String> {
    match input.parse::<i32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Could not parse the given input to i32")),
    }
}

/// Validates the the given string is a 6-digit `i32`.
/// 
/// # Example
/// ```rust
/// App::new("Example")
///     .arg(
///         Arg::with_name("number")
///             .help("A 6-digit number")
///             .takes_value(true)
///             .validator(is_six_digit)
///     );
/// ```
pub fn is_six_digit(string: String) -> Result<(), String> {
    let num = string.parse::<i32>().map_err(|err| err.to_string())?;
    if 100000 <= num && num <= 999999 {
        Ok(())
    } else {
        Err(String::from("The number wasn't a 6-digit number"))
    }
}