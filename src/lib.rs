#[derive(Default, Clone)]  // Added Clone here
pub struct GenerateOptions {
    pub checksum_only: bool,
}

fn handle_errors(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err("string cannot be empty".to_string());
    }

    if value.contains(' ') {
        return Err("string cannot contain spaces".to_string());
    }

    if value.contains('-') {
        return Err("negative numbers are not allowed".to_string());
    }

    if value.contains('.') {
        return Err("floating point numbers are not allowed".to_string());
    }

    if !value.chars().all(|c| c.is_ascii_digit()) {
        return Err("string must be convertible to a number".to_string());
    }

    Ok(())
}

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

pub fn generate(value: &str, options: Option<GenerateOptions>) -> Result<String, String> {
    handle_errors(value)?;

    let checksum = generate_checksum(value);
    
    Ok(match options {
        Some(opts) if opts.checksum_only => checksum.to_string(),
        _ => format!("{}{}", value, checksum)
    })
}

pub fn validate(value: &str) -> Result<bool, String> {
    handle_errors(value)?;

    if value.len() == 1 {
        return Err("string must be longer than 1 character".to_string());
    }

    let (value_without_checksum, _) = value.split_at(value.len() - 1);
    Ok(value == generate(value_without_checksum, None)?)
}

pub fn random(length: &str) -> Result<String, String> {
    handle_errors(length)?;

    let length_as_int: usize = length.parse()
        .map_err(|_| "failed to parse length")?;

    if length_as_int > 100 {
        return Err("string must be less than 100 characters".to_string());
    }

    if length_as_int < 2 {
        return Err("string must be greater than 1".to_string());
    }

    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    let mut random = String::with_capacity(length_as_int - 1);
    
    // First digit (1-9)
    random.push(char::from_digit(rng.gen_range(1..10), 10).unwrap());
    
    // Remaining digits (0-9)
    for _ in 1..(length_as_int - 1) {
        random.push(char::from_digit(rng.gen_range(0..10), 10).unwrap());
    }

    generate(&random, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod generate {
        use super::*;

        #[test]
        fn test_error_cases() {
            assert!(generate("", None).is_err());
            assert!(generate("1a", None).is_err());
            assert!(generate(" 123 ", None).is_err());
            assert!(generate("-123", None).is_err());
            assert!(generate("123.45", None).is_err());
        }

        #[test]
        fn test_generate_without_options() {
            assert_eq!(generate("1", None).unwrap(), "18");
            assert_eq!(generate("7992739871", None).unwrap(), "79927398713");
        }

        #[test]
        fn test_generate_with_checksum_false() {
            let options = Some(GenerateOptions { checksum_only: false });
            assert_eq!(generate("1", options.clone()).unwrap(), "18");
            assert_eq!(generate("12", options.clone()).unwrap(), "125");
            assert_eq!(generate("123", options.clone()).unwrap(), "1230");
            assert_eq!(generate("1234", options.clone()).unwrap(), "12344");
            assert_eq!(generate("12345", options.clone()).unwrap(), "123455");
            assert_eq!(generate("123456", options.clone()).unwrap(), "1234566");
            assert_eq!(generate("1234567", options.clone()).unwrap(), "12345674");
            assert_eq!(generate("12345678", options.clone()).unwrap(), "123456782");
            assert_eq!(generate("123456789", options.clone()).unwrap(), "1234567897");
            assert_eq!(generate("7992739871", options).unwrap(), "79927398713");
        }

        #[test]
        fn test_generate_with_checksum_only() {
            let options = Some(GenerateOptions { checksum_only: true });
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

    mod validate {
        use super::*;

        #[test]
        fn test_error_cases() {
            assert!(validate("").is_err());
            assert!(validate("1").is_err());
            assert!(validate("1a").is_err());
            assert!(validate(" 1230 ").is_err());
            assert!(validate("-1230").is_err());
            assert!(validate("123.40").is_err());
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
            assert!(validate("001230").unwrap());
        }
    }

    mod random {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn test_error_cases() {
            assert!(random("").is_err());
            assert!(random("1a").is_err());
            assert!(random("1").is_err());
            assert!(random("101").is_err());
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
            for count in counts.iter() {
                assert!(*count > (expected * 6 / 10)); // Within 40% below expected
                assert!(*count < (expected * 14 / 10)); // Within 40% above expected
            }
        }
    }
}

