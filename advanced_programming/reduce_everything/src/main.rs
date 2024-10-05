fn my_reduce<T: Copy, R>(init: R, mut f: impl FnMut(R, T) -> R, xs: Vec<T>) -> R {
    let mut res = init;
    for i in xs {
        res = f(res, i);
    }
    return res;
}

fn my_map<T: Copy, R>(mut f: impl FnMut(T) -> R, xs: Vec<T>) -> Vec<R> {
    return my_reduce(
        Vec::new(),
        |mut acc, x| {
            acc.push(f(x));
            return acc;
        },
        xs,
    );
}

fn my_filter<T: Copy>(mut f: impl FnMut(T) -> bool, xs: Vec<T>) -> Vec<T> {
    return my_reduce(
        Vec::new(),
        |mut acc, x| {
            if f(x) {
                acc.push(x);
            }
            return acc;
        },
        xs,
    );
}

fn main() {
    let h = my_map(|x| x + 1, vec![1, 2, 3, 4]);
    println!("Map: {:?}", h);

    let h = my_filter(|x| x % 2 == 0, vec![1, 2, 3, 4]);
    println!("Filter: {:?}", h);

    let h = my_reduce(0, |acc, x| acc + x, vec![1, 2, 3, 4]);
    println!("Reduce: {:?}", h);
}
