// https://www.hello-algo.com/chapter_computational_complexity/iteration_and_recursion/

pub fn run() {
    println!("for 循环{}", nested_for_loop(3));
    println!("递归 {}", recur(3));
    println!("尾递归 {}", tail_recur(3, 0));
    println!("斐波那契数列：递归 {}", fib(6));
    println!("使用迭代模拟递归 {}", for_loop_recur(3));
}
/* 双层 for 循环 */
fn nested_for_loop(n: i32) -> String {
    let mut res = vec![];
    for i in 1..=n {
        for j in 1..=n {
            res.push(format!("({} {}) ", i, j));
        }
    }
    res.join("")
}

/* 递归 */
fn recur(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    let res = recur(n - 1);

    n + res
}
/* 尾递归 */
fn tail_recur(n: i32, res: i32) -> i32 {
    if n == 0 {
        return res;
    }

    tail_recur(n - 1, res + n)
}

/* 斐波那契数列：递归 */
fn fib(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return n - 1;
    }
    fib(n - 1) + fib(n - 2)
}
/* 使用迭代模拟递归 */
fn for_loop_recur(n: i32) -> i32 {
    let mut stack = Vec::new();

    let mut res = 0;

    for i in (1..=n).rev() {
        stack.push(i);
    }

    while !stack.is_empty() {
        res += stack.pop().unwrap()
    }

    res
}
