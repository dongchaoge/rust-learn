// struct Cacher<T, E>
// where
//     T: Fn(E) -> E,
//     E: Copy,
// {
//     query: T,
//     value: Option<E>,
// }

// impl<T, E> Cacher<T, E>
// where
//     T: Fn(E) -> E,
//     E: Copy,
// {
//     fn new(query: T) -> Cacher<T, E> {
//         Cacher { query, value: None }
//     }
//     fn value(&mut self, value: E) -> E {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.query)(value);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

pub fn run() {
    // println!("closure");
    // let mut c = Cacher::new(|a| a + 1);
    // let v1 = c.value(1);
    // let v2 = c.value(2);

    // println!("v1:{v1} == v2:{v2}");

    fn factory() -> Box<dyn Fn(i32) -> i32> {
        let num = 5;

        Box::new(move |x| x + num)
    }

    let f = factory();

    let answer = f(1);
    println!("{answer}")
}
