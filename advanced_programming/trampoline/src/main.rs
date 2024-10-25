enum Trampoline<T> {
    Continue(Box<dyn FnOnce() -> Self>),
    Done(T),
}

fn run_trampoline<T>(mut t: Trampoline<T>) -> T {
    loop {
        match t {
            Trampoline::Continue(continue_fn) => t = continue_fn(),
            Trampoline::Done(result) => return result,
        }
    }
}

fn factorial_trampoline(n: u64, acc: u64) -> Trampoline<u64> {
    if n == 0 {
        return Trampoline::Done(acc);
    }

    return Trampoline::Continue(Box::new(move || {
        return factorial_trampoline(n - 1, acc * n);
    }));
}

fn factorial_tail(n: u64, acc: u64) -> u64 {
    if n == 0 {
        return acc;
    }

    return factorial_tail(n - 1, n * acc);
}

fn main() {
    println!("{}", run_trampoline(factorial_trampoline(20, 1)));
    println!("{}", factorial_tail(20, 1));
}
