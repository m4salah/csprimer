use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum Value {
    Int(i32),
    Str(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{i}"),
            Value::Str(c) => write!(f, "\"{c}\""),
        }
    }
}

#[derive(Debug)]
enum Pretty<T: Display> {
    Atomic(T),
    List(Vec<Pretty<T>>),
}

impl<T: Display> Pretty<T> {
    pub fn pprint(&self, indent: &str) -> String {
        let mut result = vec![];
        match self {
            Pretty::Atomic(e) => {
                result.push(format!("{e}"));
            }
            Pretty::List(vec) => {
                result.push("[".to_string());
                for (i, v) in vec.iter().enumerate() {
                    // if it's the last element, don't add a comma and newline
                    result.push(v.pprint(&format!(" {indent}")));
                    if i < vec.len() - 1 {
                        result.push(format!(",\n{indent}"));
                    }
                }
                result.push("]".to_string());
            }
        };
        return format!("{}", result.join(""));
    }
}

fn main() {
    let atomic = Pretty::Atomic(Value::Str("hello".to_string()));
    let nested_list = Pretty::List(vec![
        Pretty::Atomic(Value::Int(1)),
        Pretty::Atomic(Value::Int(2)),
        Pretty::Atomic(Value::Int(3)),
        Pretty::List(vec![
            Pretty::Atomic(Value::Str("a".to_string())),
            Pretty::Atomic(Value::Str("b".to_string())),
            Pretty::List(vec![Pretty::Atomic(Value::Str("c".to_string()))]),
            Pretty::Atomic(Value::Str("foo".to_string())),
        ]),
        Pretty::Atomic(Value::Int(4)),
    ]);

    println!("{}", atomic.pprint(" "));
    println!("{}", nested_list.pprint(" "));
}
