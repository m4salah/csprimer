use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

fn memoize<F, I, R>(func: F) -> impl Fn(I) -> R
where
    I: Hash + Eq + Clone,
    R: Clone,
    F: Fn(I) -> R,
{
    let cache = Arc::new(RwLock::new(HashMap::<I, R>::new()));

    move |input: I| {
        {
            // Check if value exists in cache with a read lock.
            let cache_read = cache.read().unwrap();
            if let Some(value) = cache_read.get(&input) {
                return value.clone();
            }
        }
        let mut cache = cache.write().unwrap();
        let result = func(input.clone());
        cache.insert(input, result.clone());
        result
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
