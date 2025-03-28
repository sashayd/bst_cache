pub trait BSTTrait<K: Copy+Ord, V: Copy> {
    fn new() -> Self;
    fn insert(&mut self, key: K, value: V);
    fn get(&self, key: K) -> Option<V>;
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;
    use std::collections::HashMap;
    use crate::bst::BST;

    fn test<T: BSTTrait<i32, f32>>() {
        let n: usize = 10_000;
        let mut rng = rand::rng();
        let mut bst: T = T::new();
        let mut dct: HashMap<i32, f32> = HashMap::new();
        let keys: Vec<i32> = (0..n).map(|_| rng.random::<i32>()).collect();
        let more_keys: Vec<i32> = (0..n).map(|_| rng.random::<i32>()).collect();
        let values: Vec<f32> = (0..n).map(|_| rng.random::<f32>()).collect();
        let more_values: Vec<f32> = (0..(n/2)).map(|_| rng.random::<f32>()).collect();
        for (k, v) in keys.iter().zip(values.iter()) {
            bst.insert(*k, *v);
            dct.insert(*k, *v);
        }
        for k in keys.iter().chain(more_keys.iter()) {
            assert!(dct.get(k) == bst.get(*k).as_ref());
        }
        for (k, v) in keys.iter().zip(more_values.iter()) {
            bst.insert(*k, *v);
            dct.insert(*k, *v);
        }
        for k in keys.iter().chain(more_keys.iter()) {
            assert!(dct.get(k) == bst.get(*k).as_ref());
        }
    }

    #[test]
    fn test_bst() {
        test::<BST<i32, f32>>();
    }
}
