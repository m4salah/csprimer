use std::sync::LazyLock;

fn main() {
    // valid case
    assert!(luhn("17893729974").unwrap());
    // invalid case
    assert!(!luhn("17893729975").unwrap());

    println!("Ok!");
}

static LUHN_LOOKUP: LazyLock<[[u32; 10]; 2]> = LazyLock::new(generate_luhn_lookup);

fn generate_luhn_lookup() -> [[u32; 10]; 2] {
    let mut lookup = [0u32; 10];
    for i in 0..10 {
        lookup[i] = if i * 2 > 9 { (i * 2) - 9 } else { i * 2 } as u32;
    }
    [[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], lookup]
}

pub fn luhn(cc_number: &str) -> Result<bool, String> {
    let sum = cc_number
        .chars()
        .rev()
        .enumerate()
        .try_fold(0, |acc, (i, c)| {
            if let Some(digit) = c.to_digit(10) {
                return Ok(acc + LUHN_LOOKUP[i % 2][digit as usize]);
            }
            return Err(format!("Invalid character '{}' in credit card number", c));
        })?;
    Ok(sum % 10 == 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263982640269299").unwrap());
        assert!(luhn("4539319503436467").unwrap());
        assert!(luhn("79927398713").unwrap());
        // from wikipedia
        assert!(luhn("17893729974").unwrap());
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223982640269299").unwrap());
        assert!(!luhn("4539319503436476").unwrap());
        assert!(!luhn("8273123273520569").unwrap());
        // from wikipedia but invalid
        assert!(!luhn("17893729975").unwrap());
    }

    #[test]
    fn test_non_digit_cc_number() {
        assert!(luhn("foo").is_err());
        assert!(luhn("foo 0 0").is_err());
    }

    #[test]
    fn test_two_digit_cc_number() {
        assert!(luhn(" 0 0 ").is_err());
    }
}
