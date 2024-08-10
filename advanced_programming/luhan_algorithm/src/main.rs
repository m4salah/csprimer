fn main() {
    println!("Hello, world!");
}

/// if the length of string is less than 2 we return false immediatly
/// other than that we
pub fn luhn(cc_number: &str) -> bool {
    if cc_number.len() < 2 {
        return false;
    }
    let mut sum = 0;
    let mut digits = 0;
    let mut double = false;
    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            // we found a digit
            digits += 1;
            let double_digit = digit * 2;
            if double {
                // the double case
                sum += if double_digit > 9 {
                    double_digit - 9
                } else {
                    double_digit
                }
            } else {
                // the normal case
                sum += digit
            };
            // flip double
            double = !double;
        } else if c.is_whitespace() {
            continue;
        } else {
            return false;
        }
    }
    digits > 1 && sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
        // from wikipedia
        assert!(luhn("17893729974"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
        // from wikipedia but invalid
        assert!(!luhn("17893729975"));
    }

    #[test]
    fn test_non_digit_cc_number() {
        assert!(!luhn("foo"));
        assert!(!luhn("foo 0 0"));
    }

    #[test]
    fn test_empty_cc_number() {
        assert!(!luhn(""));
        assert!(!luhn(" "));
        assert!(!luhn("  "));
        assert!(!luhn("    "));
    }

    #[test]
    fn test_single_digit_cc_number() {
        assert!(!luhn("0"));
    }

    #[test]
    fn test_two_digit_cc_number() {
        assert!(luhn(" 0 0 "));
    }
}
