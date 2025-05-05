use crate::bst_trait::BSTTrait;
use std::fmt::Debug;

#[derive(Debug)]
pub struct BST<K, V> {
    key_buffer: Vec<NodeLine<K>>,
    value_buffer: Vec<[V; 7]>,
}

#[repr(align(64))]
#[derive(Debug)]
struct NodeLine<K> {
    keys: [K; 7],
    indices: [u32; 8],
    // padding: u32, // probably not needed, as repr(align(64)) will do the padding for us...
}

impl<K: Copy+Ord, V: Copy> BSTTrait<K, V> for BST<K, V> {
    fn new() -> Self {
        BST{key_buffer: Vec::<NodeLine<K>>::new(), value_buffer: Vec::<[V; 7]>::new()}
    }

    fn insert(&mut self, key: K, value: V) { 
        if self.key_buffer.len() == 0 {
            self.key_buffer.push(NodeLine::<K>{keys: [key; 7], indices: [0; 8]});
            self.value_buffer.push([value; 7]);
            return;
        }
        let len: u32 = self.key_buffer.len() as u32;
        let mut i: usize = 0;
        loop {
            let node_line: &mut NodeLine<K> = &mut self.key_buffer[i];
            let mut j: usize = 0;
            for _ in 0..2 {
                let k: K = node_line.keys[j];
                if key == k {
                    self.value_buffer[i][j] = value;
                    return;
                } else if key < k {
                    j = (j<<1)+1;
                } else {
                    j=(j+1)<<1;
                }
                if node_line.keys[j] == k {
                    node_line.keys[j] = key;
                    self.value_buffer[i][j] = value;
                    return;
                }
            }
            let k: K = node_line.keys[j];
            if key == k {
                self.value_buffer[i][j] = value;
                return;
            } else if key < k {
                j = (j<<1)+1;
            } else {
                j = (j+1)<<1;
            }
            i = node_line.indices[j-7] as usize;
            if i == 0 {
                node_line.indices[j-7] = len;
                self.key_buffer.push(NodeLine::<K>{keys: [key; 7], indices: [0; 8]});
                self.value_buffer.push([value; 7]);
            }
        }
    }

    fn get(&self, key: K) -> Option<V> {
        if self.key_buffer.len() == 0 {
            return None;
        }
        let mut i: usize = 0;
        loop {
            let node_line: &NodeLine<K> = &self.key_buffer[i];
            let mut j: usize = 0;
            for _ in 0..2 {
                let k: K = node_line.keys[j];
                if key == k {
                    return Some(self.value_buffer[i][j]);
                } else if key < k {
                    j = (j<<1)+1;
                } else {
                    j = (j+1)<<1;
                }
                if node_line.keys[j] == k {
                    return None;
                }
            }
            let k: K = node_line.keys[j];
            if key == k {
                return Some(self.value_buffer[i][j]);
            } else if key < k {
                j = (j<<1)+1;
            } else {
                j = (j+1)<<1;
            }
            i = node_line.indices[j-7] as usize;
            if i == 0 {
                return None;
            }
        }
    }
}

