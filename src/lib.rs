//! Luhn algorithm implementation in Rust.
//!
//! This crate provides functionality to generate and validate numbers using the Luhn algorithm,
//! commonly used for validating various identification numbers such as credit card numbers,
//! Canadian Social Insurance Numbers, and other identification numbers.
//!
//! # Examples
//!
//! ```
//! use luhn_tools::{generate, validate, GenerateOptions};
//!
//! // Generate a Luhn number
//! let result = generate("7992739871", None).unwrap();
//! assert_eq!(result, "79927398713");
//!
//! // Validate a Luhn number
//! let is_valid = validate("79927398713").unwrap();
//! assert!(is_valid);
//! ```

#![cfg_attr(not(feature = "std"), no_std)] // Allow no_std usage

use std::error::Error;
use std::fmt;

/// Configuration options for generating Luhn numbers.
#[derive(Default, Clone)] // Added Clone here
pub struct GenerateOptions {
    /// If true, returns only the checksum digit.
    /// If false, returns the original number with the checksum digit appended.
    pub checksum_only: bool,
}

#[derive(Debug, PartialEq)]
pub enum LuhnError {
    /// Input string is empty
    EmptyString,
    /// Input contains whitespace
    ContainsSpaces,
    /// Input contains a negative number
    NegativeNumber,
    /// Input contains a floating point number
    FloatingPoint,
    /// Input contains non-numeric characters
    NonNumeric,
    /// Input length is invalid (too short or too long)
    InvalidLength(String),
    /// Error parsing number
    ParseError(String),
}

impl fmt::Display for LuhnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LuhnError::EmptyString => write!(f, "string cannot be empty"),
            LuhnError::ContainsSpaces => write!(f, "string cannot contain spaces"),
            LuhnError::NegativeNumber => write!(f, "negative numbers are not allowed"),
            LuhnError::FloatingPoint => write!(f, "floating point numbers are not allowed"),
            LuhnError::NonNumeric => write!(f, "string must be convertible to a number"),
            LuhnError::InvalidLength(msg) => write!(f, "{}", msg),
            LuhnError::ParseError(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for LuhnError {}

/// Validates input string against common error conditions.
///
/// # Arguments
/// * `value` - The string to validate
///
/// # Returns
/// * `Ok(())` if validation passes
/// * `Err(String)` with error message if validation fails
fn handle_errors(value: &str) -> Result<(), LuhnError> {
    if value.is_empty() {
        return Err(LuhnError::EmptyString);
    }

    if value.contains(' ') {
        return Err(LuhnError::ContainsSpaces);
    }

    if value.contains('-') {
        return Err(LuhnError::NegativeNumber);
    }

    if value.contains('.') {
        return Err(LuhnError::FloatingPoint);
    }

    if !value.chars().all(|c| c.is_ascii_digit()) {
        return Err(LuhnError::NonNumeric);
    }

    Ok(())
}

/// Calculates the Luhn checksum for a given numeric string.
///
/// # Arguments
/// * `value` - A string slice containing only numeric characters
///
/// # Returns
/// * `u8` - The calculated checksum digit
///
/// # Panics
/// * Panics if the input string contains non-numeric characters
fn generate_checksum(value: &str) -> u8 {
    let mut double = true;
    let sum: u32 = value
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .fold(0, |acc, digit| {
            let mut result = acc;
            if double {
                let temp = digit * 2;
                if temp >= 10 {
                    result += temp / 10 + temp % 10;
                } else {
                    result += temp;
                }
            } else {
                result += digit;
            }
            double = !double;
            result
        });

    ((10 - (sum % 10)) % 10) as u8
}

/// Generates a Luhn number or checksum from the input value.
///
/// # Arguments
/// * `value` - A string slice that holds the number to process
/// * `options` - Optional configuration for generation
///
/// # Returns
/// * `Ok(String)` - The generated Luhn number or checksum
/// * `Err(String)` - Error message if validation fails
///
/// # Examples
/// ```
/// use luhn_tools::{generate, GenerateOptions};
///
/// // Generate full Luhn number
/// let result = generate("7992739871", None).unwrap();
/// assert_eq!(result, "79927398713");
///
/// // Generate only checksum
/// let options = Some(GenerateOptions { checksum_only: true });
/// let checksum = generate("7992739871", options).unwrap();
/// assert_eq!(checksum, "3");
/// ```
///
/// # Errors
/// Returns an error if:
/// * The input string is empty
/// * The input contains spaces
/// * The input contains negative numbers
/// * The input contains floating point numbers
/// * The input contains non-numeric characters
#[cfg(feature = "std")]
pub fn generate(value: &str, options: Option<GenerateOptions>) -> Result<String, LuhnError> {
    handle_errors(value)?;

    let checksum = generate_checksum(value);

    Ok(match options {
        Some(opts) if opts.checksum_only => checksum.to_string(),
        _ => format!("{}{}", value, checksum),
    })
}

/// Validates whether a number satisfies the Luhn algorithm.
///
/// # Arguments
/// * `value` - A string slice that holds the number to validate
///
/// # Returns
/// * `Ok(bool)` - True if the number is valid, false otherwise
/// * `Err(String)` - Error message if validation fails
///
/// # Examples
/// ```
/// use luhn_tools::validate;
///
/// assert!(validate("79927398713").unwrap());
/// assert!(!validate("79927398714").unwrap());
/// ```
///
/// # Errors
/// Returns an error if:
/// * The input string is empty
/// * The input contains spaces
/// * The input contains negative numbers
/// * The input contains floating point numbers
/// * The input contains non-numeric characters
/// * The input is only one character long
#[cfg(feature = "std")]
pub fn validate(value: &str) -> Result<bool, LuhnError> {
    handle_errors(value)?;

    if value.len() == 1 {
        return Err(LuhnError::InvalidLength(
            "string must be longer than 1 character".to_string(),
        ));
    }

    let (value_without_checksum, _) = value.split_at(value.len() - 1);
    Ok(value == generate(value_without_checksum, None)?)
}

/// Generates a random number of specified length with a valid Luhn checksum.
///
/// # Arguments
/// * `length` - A string slice containing the desired length of the number
///
/// # Returns
/// * `Ok(String)` - A random number of the specified length with valid Luhn checksum
/// * `Err(String)` - Error message if validation fails
///
/// # Examples
/// ```
/// use luhn_tools::{random, validate};
///
/// let random_number = random("10").unwrap();
/// assert_eq!(random_number.len(), 10);
/// assert!(validate(&random_number).unwrap());
/// ```
///
/// # Errors
/// Returns an error if:
/// * The length string is empty
/// * The length string contains non-numeric characters
/// * The requested length is less than 2
/// * The requested length is greater than 100
#[cfg(all(feature = "random", feature = "std"))]
pub fn random(length: &str) -> Result<String, LuhnError> {
    handle_errors(length)?;

    let length_as_int: usize = length
        .parse()
        .map_err(|_| LuhnError::ParseError("failed to parse length".to_string()))?;

    if length_as_int > 100 {
        return Err(LuhnError::InvalidLength(
            "string must be less than 100 characters".to_string(),
        ));
    }

    if length_as_int < 2 {
        return Err(LuhnError::InvalidLength(
            "string must be greater than 1".to_string(),
        ));
    }

    use rand::Rng;
    let mut rng = rand::thread_rng();

    let mut random = String::with_capacity(length_as_int - 1);

    loop {
        random.clear();

        // Generate all digits randomly (0-9)
        for _ in 0..(length_as_int - 1) {
            random.push(char::from_digit(rng.gen_range(0..10), 10).unwrap());
        }

        // Add checksum and check if valid
        if let Ok(result) = generate(&random, None) {
            if validate(&result).unwrap_or(false) {
                return Ok(result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "random")]
    use std::collections::HashSet;

    #[cfg(feature = "std")]
    mod generate {
        use super::*;

        #[test]
        fn test_error_cases() {
            assert_eq!(generate("", None).unwrap_err(), LuhnError::EmptyString);
            assert_eq!(generate("1a", None).unwrap_err(), LuhnError::NonNumeric);
            assert_eq!(
                generate(" 123 ", None).unwrap_err(),
                LuhnError::ContainsSpaces
            );
            assert_eq!(
                generate("-123", None).unwrap_err(),
                LuhnError::NegativeNumber
            );
            assert_eq!(
                generate("123.45", None).unwrap_err(),
                LuhnError::FloatingPoint
            );
        }

        #[test]
        fn test_generate_without_options() {
            assert_eq!(generate("1", None).unwrap(), "18");
            assert_eq!(generate("7992739871", None).unwrap(), "79927398713");
            assert_eq!(generate("0123", None).unwrap(), "01230");
        }

        #[test]
        fn test_generate_with_checksum_false() {
            let options = Some(GenerateOptions {
                checksum_only: false,
            });
            assert_eq!(generate("1", options.clone()).unwrap(), "18");
            assert_eq!(generate("12", options.clone()).unwrap(), "125");
            assert_eq!(generate("123", options.clone()).unwrap(), "1230");
            assert_eq!(generate("1234", options.clone()).unwrap(), "12344");
            assert_eq!(generate("12345", options.clone()).unwrap(), "123455");
            assert_eq!(generate("123456", options.clone()).unwrap(), "1234566");
            assert_eq!(generate("1234567", options.clone()).unwrap(), "12345674");
            assert_eq!(generate("12345678", options.clone()).unwrap(), "123456782");
            assert_eq!(
                generate("123456789", options.clone()).unwrap(),
                "1234567897"
            );
            assert_eq!(generate("7992739871", options).unwrap(), "79927398713");
        }

        #[test]
        fn test_generate_with_checksum_only() {
            let options = Some(GenerateOptions {
                checksum_only: true,
            });
            assert_eq!(generate("1", options.clone()).unwrap(), "8");
            assert_eq!(generate("12", options.clone()).unwrap(), "5");
            assert_eq!(generate("123", options.clone()).unwrap(), "0");
            assert_eq!(generate("1234", options.clone()).unwrap(), "4");
            assert_eq!(generate("12345", options.clone()).unwrap(), "5");
            assert_eq!(generate("123456", options.clone()).unwrap(), "6");
            assert_eq!(generate("1234567", options.clone()).unwrap(), "4");
            assert_eq!(generate("12345678", options.clone()).unwrap(), "2");
            assert_eq!(generate("123456789", options.clone()).unwrap(), "7");
            assert_eq!(generate("7992739871", options).unwrap(), "3");
        }

        #[test]
        fn test_edge_cases() {
            assert_eq!(generate("0", None).unwrap(), "00");
            assert_eq!(generate("00123", None).unwrap(), "001230");
        }
    }

    #[cfg(feature = "std")]
    mod validate {
        use super::*;

        #[test]
        fn test_error_cases() {
            assert_eq!(validate("").unwrap_err(), LuhnError::EmptyString);
            assert_eq!(
                validate("1").unwrap_err(),
                LuhnError::InvalidLength("string must be longer than 1 character".to_string())
            );
            assert_eq!(validate("1a").unwrap_err(), LuhnError::NonNumeric);
        }

        #[test]
        fn test_invalid_checksums() {
            assert!(!validate("10").unwrap());
            assert!(!validate("120").unwrap());
            assert!(!validate("1231").unwrap());
        }

        #[test]
        fn test_valid_checksums() {
            assert!(validate("18").unwrap());
            assert!(validate("125").unwrap());
            assert!(validate("1230").unwrap());
            assert!(validate("01230").unwrap());
            assert!(validate("001230").unwrap());
        }
    }

    #[cfg(all(feature = "random", feature = "std"))]
    mod random {
        use super::*;

        #[test]
        fn test_error_cases() {
            assert_eq!(random("").unwrap_err(), LuhnError::EmptyString);
            assert_eq!(random("1a").unwrap_err(), LuhnError::NonNumeric);
            assert_eq!(
                random("1").unwrap_err(),
                LuhnError::InvalidLength("string must be greater than 1".to_string())
            );
            assert_eq!(
                random("101").unwrap_err(),
                LuhnError::InvalidLength("string must be less than 100 characters".to_string())
            );
        }

        #[test]
        fn test_valid_lengths() {
            let lengths = ["2", "25", "50", "99"];
            for length in lengths.iter() {
                let value = random(length).unwrap();
                assert!(validate(&value).unwrap());
                assert_eq!(value.len(), length.parse::<usize>().unwrap());
            }
        }

        #[test]
        fn test_randomness() {
            let mut results = HashSet::new();
            for _ in 0..100 {
                results.insert(random("10").unwrap());
            }
            assert_eq!(results.len(), 100); // All numbers should be unique
        }

        #[test]
        fn test_distribution() {
            let mut counts = [0; 10];
            let iterations = 1000;

            for _ in 0..iterations {
                let number = random("2").unwrap();
                let first_digit = number.chars().next().unwrap().to_digit(10).unwrap() as usize;
                counts[first_digit] += 1;
            }

            let expected = iterations / 10;
            let min_threshold = expected * 6 / 10;

            // println!("\nExpected count per digit: {}", expected);
            // println!("Minimum threshold (60%): {}\n", min_threshold);
            // println!("Actual counts per digit:");
            for (digit, count) in counts.iter().enumerate() {
                println!(
                    "Digit {}: {} {}",
                    digit,
                    count,
                    if *count < min_threshold {
                        "<!- BELOW THRESHOLD"
                    } else {
                        ""
                    }
                );
            }
            // println!("");

            for count in counts.iter() {
                assert!(*count > (expected * 6 / 10)); // Within 40% below expected
                assert!(*count < (expected * 14 / 10)); // Within 40% above expected
            }
        }
    }
}
