/// Calculate the product of the first n integers.
///
/// ```
/// use basic_recursion::factorial;
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(3), 6);
/// assert_eq!(factorial(10), 3628800);
/// ```
pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    return n * factorial(n - 1);
}

/// Determine if the given phrase is a palindrome. As a stretch goal, support phrases
/// such as "A man, a plan, a canal — Panama!".
///
/// ```
/// use basic_recursion::palindrome;
/// assert!(palindrome("racecar"));
/// assert!(!palindrome("hello"));
/// ```
pub fn palindrome(letters: &str) -> bool {
    if letters.len() <= 1 {
        return true;
    }
    let first = letters.chars().nth(0).unwrap();
    let last = letters.chars().nth(letters.len() - 1).unwrap();
    let rest = &letters[1..letters.len() - 1];
    return first == last && palindrome(rest);
}

/// Determine the greatest common divisor of two integers a and b. One approach is
/// described in Euclid's Elements roughly as:
///
/// - Repeatedly subtract the smaller number from the larger one, until no longer possible
/// - If the remainder is zero, then the smaller number must be the GCD
/// - Otherwise, repeat the process, using the number you were subtracting as the new "larger"
///     number, and the remainder as the new "smaller" number
///
/// For example if a = 1071 and b = 462 (taken from Wikipedia):
///
/// - Subtracting 462 twice from 1071 leaves a remainder of 147
/// - Subtracting 147 three times from 462 leaves a remainder of 21
/// - Subtracting 21 seven ties from 147 leaves a remainder of 0
///
/// So, 21 divides both 147 and 462, and is the greatest number to do this.
/// such as "A man, a plan, a canal — Panama!".
///
/// ```
/// use basic_recursion::gcd;
/// assert!(gcd(1071, 462) == 21);
/// assert!(gcd(15, 9) == 3);
/// assert!(gcd(9, 15) == 3);
/// assert!(gcd(3, 5) == 1);
/// ```
pub fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    return gcd(b % a, a);
}

/// Given a predicate function f (a function which returns true or false) and a list
/// xs, return a new list with only the items of xs where f(x) is true.
///
/// ```
/// use basic_recursion::filter_rec;
/// assert!(filter_rec(vec![-1, 3, -2, 5], &|x| x > 0) == vec![3, 5]);
/// ```
pub fn filter_rec<T: Clone>(xs: Vec<T>, f: &impl Fn(T) -> bool) -> Vec<T> {
    if xs.is_empty() {
        return vec![];
    }

    let first = xs.first().unwrap();
    let rest = filter_rec(xs[1..].to_vec(), f);

    if !f(first.clone()) {
        return rest;
    }

    let mut result = vec![first.clone()];
    result.extend(rest);
    return result;
}

/// Apply the function f cumulatively to the items of xs, reducing to a single value.
///
/// If initializer is provided, use it as the first value. Otherwise, use only the
/// values in xs.
/// xs, return a new list with only the items of xs where f(x) is true.
///
/// ```
/// use basic_recursion::reduce;
/// assert!(reduce(vec![1, 2, 3, 4], 0, &|acc, x| acc + x) == 10);
/// ```
pub fn reduce<T: Clone, I: Clone>(xs: Vec<T>, init: I, f: &impl Fn(I, T) -> I) -> I {
    if xs.is_empty() {
        return init;
    }
    return reduce(xs[1..].to_vec(), f(init, xs[0].clone()), f);
}

/// A newly born breeding pair of rabbits are placed in a field. At the age of one month,
/// the pair produces another breeding pair of rabbits. All pairs will breed every month,
/// and somehow never die. How many pairs will exist at month n?
///
/// ```
/// use basic_recursion::rabbit_pairs;
/// assert_eq!(rabbit_pairs(1), 1);
/// assert_eq!(rabbit_pairs(2), 1);
/// assert_eq!(rabbit_pairs(3), 2);
/// assert_eq!(rabbit_pairs(5), 5);
/// assert_eq!(rabbit_pairs(6), 8);
/// assert_eq!(rabbit_pairs(20), 6765);
/// ```
pub fn rabbit_pairs(n: u64) -> u64 {
    if n == 1 || n == 2 {
        return 1;
    }
    return rabbit_pairs(n - 1) + rabbit_pairs(n - 2);
}
