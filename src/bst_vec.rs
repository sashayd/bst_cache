use crate::bst_trait::BSTTrait;

pub struct BST<K, V> {
    buffer: Vec<Node<K, V>>,
}

struct Node<K, V> {
    key: K,
    value: V,
    left: usize,
    right: usize,
}

impl<K: Copy+Ord, V: Copy> BSTTrait<K,V> for BST<K, V> {
    fn new() -> Self {
        BST{buffer: Vec::<Node<K, V>>::new()}
    }

    fn insert(&mut self, key: K, value: V) {
        let ln: usize = self.buffer.len();
        if ln == 0 {
            self.buffer.push(Node{key: key, value: value, left: 0, right: 0});
            return;
        }
        let mut n: usize = 0;
        loop {
            let node: &Node<K, V> = &self.buffer[n];
            let k: K = node.key;
            if key == k {
                self.buffer[n].value = value;
                return;
            } else if key < k {
                if node.left == 0 {
                    self.buffer.push(Node{key: key, value: value, left: 0, right: 0});
                    self.buffer[n].left = ln;
                    return;
                }
                n = node.left;
            } else {
                if node.right == 0 {
                    self.buffer.push(Node{key: key, value: value, left: 0, right: 0});
                    self.buffer[n].right = ln;
                    return;
                }
                n = node.right;
            }
        }
    }

    fn get(&self, key: K) -> Option<V> {
        if self.buffer.len() == 0 {
            return None;
        }
        let mut n: usize = 0;
        loop {
            let node: &Node<K,V> = &self.buffer[n];
            let k: K = node.key;
            if key == k {
                return Some(node.value);
            } else if key < k {
                if node.left == 0 {
                    return None;
                }
                n = node.left;
            } else {
                if node.right == 0 {
                    return None;
                }
                n = node.right;
            }
        }
    }
}

