fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
