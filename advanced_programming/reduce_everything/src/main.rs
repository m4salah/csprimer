fn my_map<T: Copy, R>(mut f: impl FnMut(T) -> R, xs: Vec<T>) -> Vec<R> {
    return xs.iter().fold(Vec::new(), |mut acc, x| {
        acc.push(f(*x));
        return acc;
    });
}
fn main() {
    let h = my_map(|x| x + 1, vec![1, 2, 3, 4]);
    println!("{:?}", h);
}
