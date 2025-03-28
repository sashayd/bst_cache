use crate::bst_trait::BSTTrait;

pub struct BST<K, V> {
    root: Option<Box<Node<K, V>>>,
}

struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K: Copy+Ord, V: Copy> BSTTrait<K,V> for BST<K,V> {
    fn new() -> Self {
        BST{root: None}
    }

    fn insert(&mut self, key: K, value: V) {
        let mut n: &mut Option<Box<Node<K,V>>> = &mut self.root;
        loop {
            if n.is_none() {
                *n = Some(Box::new(Node{
                    key: key,
                    value: value,
                    left: None,
                    right: None,
                }));
                break;
            }
            let k: K = n.as_mut().unwrap().key;
            if k == key {
                n.as_mut().unwrap().value = value;
                break;
            } else if k < key {
                n = &mut n.as_mut().unwrap().right;
            } else {
                n = &mut n.as_mut().unwrap().left;
            }
        }
    }

    fn get(&self, key: K) -> Option<V> {
        let mut n: &Option<Box<Node<K,V>>> = &self.root;
        loop {
            if n.is_none() {
                return None;
            }
            let k: K = n.as_ref().unwrap().key;
            if k == key {
                return Some(n.as_ref().unwrap().value);
            }
            else if k < key {
                n = &n.as_ref().unwrap().right;
            } else {
                n = &n.as_ref().unwrap().left;
            }
        }
    }
}

