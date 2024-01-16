// 循环引用
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
// 树结构
pub fn run() {
    // 叶子节点
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    println!("{}", &leaf.value);
    println!("{:?}", &leaf.children);
    {
        // 分支节点
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        branch.children.borrow_mut().push(leaf.clone());

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        println!("leaf parent = {:?}", leaf.parent);
    }
    // leaf.parent.borrow().upgrade() 的目的是获取 leaf 的父节点。
    // leaf.parent.borrow() 获取到 Ref<Weak<Node>>，
    // 然后通过 .upgrade() 尝试将 Weak<Node> 转换为 Option<Rc<Node>>，即获取父节点的强引用。
    // 这样可以避免循环引用问题，因为父节点是使用 Weak 引用的。
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
