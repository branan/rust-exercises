#[derive(Debug)]
enum Cell {
    I(i32),
    F(f64),
    T(String),
}

fn main() {
    let v = vec![Cell::I(1), Cell::F(2.1), Cell::T(String::from("Hello"))];
    println!("{:?}", v)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
