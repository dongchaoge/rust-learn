pub fn run() {
    let values = vec![1, 2, 3];

    // 模拟for

    {
        let result = match IntoIterator::into_iter(values) {
            mut iter => loop {
                match iter.next() {
                    Some(x) => {
                        println!("{}", x);
                    }
                    None => break,
                }
            },
        };
        println!("{:?}", result);
        result
    }
}
