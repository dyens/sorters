#![feature(test)]

extern crate test;
extern crate rand;
use rand::prelude::*;
use std::cmp::Ord;

const N: usize = 1000;


fn gen_vector(len: usize) -> Vec<u32> {
    let mut rng = thread_rng();
    let mut v = Vec::with_capacity(len);
    for _ in 0..len {
        v.push(rng.gen_range(0, 100))
    }
    v
}

fn rust_sort(v: &mut Vec<u32>) {
    v.sort();
}

fn bubble_sort(v: &mut Vec<u32>) {
    let len = v.len();
    let mut tmp: u32;
    for i in 0..len {
        for j in i..len {
            if v[i] > v[j] {
                tmp = v[i];
                v[i] = v[j];
                v[j] = tmp;
            }
        }
    }
}

fn insert_sort(v: &mut Vec<u32>) {
    let len = v.len();
    let mut tmp: u32;
    for i in 1..len {
        for j in 1..(i + 1)  {
            let index1 = i - j + 1;
            let index2 = i - j;
            if v[index1] < v[index2]  {
                tmp = v[index2];
                v[index2] = v[index1];
                v[index1] = tmp;
            }

        }
    }
}


fn gnome_sort(v: &mut Vec<u32>) {
    let mut i: usize = 1;
    let mut tmp: u32;
    while i < v.len() {
        if i==0 || v[i - 1] <= v[i] {
            i = i + 1;
        } else {
            tmp = v[i];
            v[i] = v[i - 1];
            v[i - 1] = tmp;
            i = i - 1;

        }
    }
}

fn _merge_sort(v: &mut Vec<u32>, lo: usize, hi: usize) {
    if hi <= lo {return;}
    let mid: usize = lo + (hi - lo) / 2;
    _merge_sort(v, lo, mid);
    _merge_sort(v, mid + 1, hi);

    let mut buf = vec![0; v.len()];
    for k in lo..(hi + 1) {
        buf[k] = v[k];
    }

    let mut i: usize = lo;
    let mut j: usize = mid + 1;


    for k in lo..(hi + 1) {
        if i > mid {
            v[k] = buf[j];
            j += 1;
        } else if j > hi {
            v[k] = buf[i];
            i += 1;
        } else if buf[j] < buf[i] {
            v[k] = buf[j];
            j += 1;
        } else {
            v[k] = buf[i];
            i += 1;
        }
    }
}

fn merge_sort(v: &mut Vec<u32>) {
    let hi = v.len() - 1;
    _merge_sort(v, 0, hi);
}


struct Node<T> where T: Ord + Clone {
    val: T,
    count: usize,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl <T> Node<T> where T: Ord + Clone{

    fn new(new_val: T) -> Node<T> {
        Node {
            val: new_val,
            count: 1,
            left: None,
            right: None
        }
    }

    fn insert(&mut self, new_val: T) {
        if self.val == new_val {self.count += 1; return}
        let target_node = if new_val < self.val { &mut self.left} else {&mut self.right};
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_val),
            &mut None => {
                let new_node = Node::new(new_val);
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }

    fn travel(&self, to: &mut Vec<T> ) {
        match self.left {
            Some(ref left) => left.travel(to),
            None => {}

        }
        for _ in 0..self.count {
            to.push(self.val.clone());
        }
        match self.right {
            Some(ref right) => right.travel(to),
            None => {}
        }

    }
}

fn tree_sort(v: &mut Vec<u32>) {
    let mut tree = Node::new(v[0]);
    for i in &v[1..v.len()] {
        tree.insert(*i);
    }
    unsafe {
        v.set_len(0);
    }
    tree.travel(v);


}


fn main() {
//    let mut v = gen_vector(N);
//    println!("{:?}", v);
//    rust_sort(&mut v);
//    println!("{:?}", v);
//
//    let mut v = gen_vector(N);
//    println!("{:?}", v);
//    bubble_sort(&mut v);
//    println!("{:?}", v);

//    let mut v = gen_vector(N);
//    println!("{:?}", v);
//    insert_sort(&mut v);
//    println!("{:?}", v);

//    let mut v = gen_vector(N);
//    println!("{:?}", v);
//    gnome_sort(&mut v);
//    println!("{:?}", v);

//    let mut v = gen_vector(N);
//    println!("{:?}", v);
//    merge_sort(&mut v);
//    println!("{:?}", v);

//    let mut v = gen_vector(N);
//    println!("{:?}", v);
//    tree_sort(&mut v);
//    println!("{:?}", v);
}


fn bench_sort<F>(f: F) where
    F: for<'a> Fn(&'a mut Vec<u32>)
{
    let mut v = gen_vector(N);
    f(&mut v);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_bubble_sort() {
        let mut v1 = gen_vector(N);
        let mut v2 = v1.clone();
        rust_sort(&mut v1);
        bubble_sort(&mut v2);
        assert!(v1==v2);
    }


    #[test]
    fn test_insert_sort() {
        let mut v1 = gen_vector(N);
        let mut v2 = v1.clone();
        rust_sort(&mut v1);
        insert_sort(&mut v2);
        assert!(v1==v2);
    }

    #[test]
    fn test_gnome_sort() {
        let mut v1 = gen_vector(N);
        let mut v2 = v1.clone();
        rust_sort(&mut v1);
        gnome_sort(&mut v2);
        assert!(v1==v2);
    }

    #[test]
    fn test_merge_sort() {
        let mut v1 = gen_vector(N);
        let mut v2 = v1.clone();
        rust_sort(&mut v1);
        merge_sort(&mut v2);
        assert!(v1==v2);
    }

    #[test]
    fn test_tree_sort() {
        let mut v1 = gen_vector(N);
        let mut v2 = v1.clone();
        rust_sort(&mut v1);
        tree_sort(&mut v2);
        assert!(v1==v2);
    }



    #[bench]
    fn bench_rust_sort(b: &mut Bencher) {
        b.iter(|| bench_sort(rust_sort));
    }

    #[bench]
    fn bench_bubble_sort(b: &mut Bencher) {
        b.iter(|| bench_sort(bubble_sort));
    }

    #[bench]
    fn bench_insertion_sort(b: &mut Bencher) {
        b.iter(|| bench_sort(insert_sort));
    }

    #[bench]
    fn bench_gnome_sort(b: &mut Bencher) {
        b.iter(|| bench_sort(gnome_sort));
    }

    #[bench]
    fn bench_merge_sort(b: &mut Bencher) {
        b.iter(|| bench_sort(merge_sort));
    }

    #[bench]
    fn bench_tree_sort(b: &mut Bencher) {
        b.iter(|| bench_sort(tree_sort));
    }


}
