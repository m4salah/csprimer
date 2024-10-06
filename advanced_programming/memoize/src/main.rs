use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

fn memoize<F, I, R>(func: F) -> impl Fn(I) -> R
where
    I: Hash + Eq + Clone,
    R: Clone,
    F: Fn(I) -> R,
{
    // Use RefCell to allow interior mutability of cache inside the closure.
    let cache = RefCell::new(HashMap::<I, R>::new());

    move |input: I| {
        // Borrow the cache and check if the input is already present.
        let mut cache = cache.borrow_mut();
        match cache.get(&input) {
            Some(value) => value.clone(),
            None => {
                let result = func(input.clone());
                cache.insert(input, result.clone());
                result
            }
        }
    }
}

fn fetch(url: &str) -> String {
    let response = reqwest::blocking::get(url).unwrap();
    response.text().unwrap()
}

// fibonacci function
fn fib(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let fib = memoize(fib);
    println!("Cached Version Fibonacci(30) = {}", fib(40));

    let cached_fetch = memoize(fetch);
    println!(
        "Cached Fetch(google) = {}",
        &cached_fetch("http://google.com")[..80]
    );
    println!(
        "Cached Fetch(google) = {}",
        &cached_fetch("http://google.com")[80..160]
    );
    println!(
        "Cached Fetch(google) = {}",
        &cached_fetch("http://google.com")[160..240]
    );
    println!(
        "Cached Fetch(google) = {}",
        &cached_fetch("http://google.com")[240..320]
    );
}
