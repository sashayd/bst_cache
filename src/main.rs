mod bst_trait;
mod bst;
mod bst_vec;
mod bst_cache;

use bst_trait::BSTTrait;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::hint::black_box;
use std::time::{Instant, Duration};
use std::collections::BTreeMap;
use crate::bst::BST as BST;
use crate::bst_vec::BST as BST_VEC;
use crate::bst_cache::BST as BST_CACHE;

fn measure_time<T: BSTTrait<i32, f32>>(name: &str, num_entries: Option<usize>, num_repeats: Option<u32>, seed: Option<[u8; 32]>) {
    let n: usize = num_entries.unwrap_or(1_000_000);
    let r: u32 = num_repeats.unwrap_or(1);
    let seed: [u8; 32] = seed.unwrap_or([42; 32]); 
    let mut insert_time: Duration = Duration::new(0, 0);
    let mut get_time_1: Duration = Duration::new(0, 0);
    let mut get_time_2: Duration = Duration::new(0, 0);
    let mut rng = StdRng::from_seed(seed);
    let mut bst: T = T::new();
    for _ in 0..r {
        let keys: Vec<i32> = (0..n).map(|_| rng.random()).collect();
        let more_keys: Vec<i32> = (0..n).map(|_| rng.random()).collect();
        let values: Vec<f32> = (0..n).map(|_| rng.random()).collect();
        let timer = Instant::now();
        for (k, v) in keys.iter().zip(values.iter()) {
            bst.insert(*k, *v);
        }
        insert_time += timer.elapsed();
        let timer1 = Instant::now();
        for k in keys.iter() {
            let result = bst.get(*k);
            black_box(result);
        }
        get_time_1 += timer1.elapsed();
        let timer2 = Instant::now();
        for k in more_keys.iter() {
            let result = bst.get(*k);
            black_box(result);
        }
        get_time_2 += timer2.elapsed();
    }
    insert_time /= r;
    get_time_1 /= r;
    get_time_2 /= r;
    println!("{:?}: Insertion of {:?} random items took {:?}", name, n, insert_time);
    println!("{:?}: Getting of {:?} existing items took {:?}", name, n, get_time_1);
    println!("{:?}: Getting of {:?} random items took {:?}", name, n, get_time_2);
}

fn measure_time_std(name: &str, num_entries: Option<usize>, num_repeats: Option<u32>, seed: Option<[u8; 32]>) {
    let n: usize = num_entries.unwrap_or(1_000_000);
    let r: u32 = num_repeats.unwrap_or(1);
    let seed: [u8; 32] = seed.unwrap_or([42; 32]); 
    let mut insert_time: Duration = Duration::new(0, 0);
    let mut get_time_1: Duration = Duration::new(0, 0);
    let mut get_time_2: Duration = Duration::new(0, 0);
    let mut rng = StdRng::from_seed(seed);
    let mut bst: BTreeMap<i32, f32> = BTreeMap::new();
    for _ in 0..r {
        let keys: Vec<i32> = (0..n).map(|_| rng.random()).collect();
        let more_keys: Vec<i32> = (0..n).map(|_| rng.random()).collect();
        let values: Vec<f32> = (0..n).map(|_| rng.random()).collect();
        let timer = Instant::now();
        for (k, v) in keys.iter().zip(values.iter()) {
            bst.insert(*k, *v);
        }
        insert_time += timer.elapsed();
        let timer1 = Instant::now();
        for k in keys.iter() {
            if let Some(result) = bst.get(black_box(k)) {
                black_box(*result);
            }
            //let result = bst.get(k);
            //black_box(*result);
        }
        get_time_1 += timer1.elapsed();
        let timer2 = Instant::now();
        for k in more_keys.iter() {
            let result = bst.get(k);
            black_box(result);
        }
        get_time_2 += timer2.elapsed();
    }
    insert_time /= r;
    get_time_1 /= r;
    get_time_2 /= r;
    println!("{:?}: Insertion of {:?} random items took {:?}", name, n, insert_time);
    println!("{:?}: Getting of {:?} existing items took {:?}", name, n, get_time_1);
    println!("{:?}: Getting of {:?} random items took {:?}", name, n, get_time_2);
}

fn main() {
    measure_time::<BST<i32, f32>>("bst", None, Some(5), None);
    measure_time::<BST_VEC<i32, f32>>("bst_vec", None, Some(5), None);
    measure_time::<BST_CACHE<i32, f32>>("bst_cache", None, Some(5), None);
    measure_time_std("btree_map", None, Some(5), None);
}

