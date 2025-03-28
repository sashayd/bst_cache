mod bst_trait;
mod bst;

use bst_trait::BSTTrait;

fn main() {
    println!("Hello, world!");
    let mut t: bst::BST<i32, f32> = bst::BST::new();
    t.insert(3, 4.13);
    t.insert(4, 123.3);
    t.insert(1, 12345.4);
    let r: Option<f32> = t.get(1);
    if let Some(v) = r {
        println!("The value of 1 is {}", v);
    } else {
        println!("didn't find");
    }
}
