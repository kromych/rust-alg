use std::fmt::Debug;
use std::thread;

use crate::*;

pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        println!("{:?}", v);
        let mut sorted = true;
        for j in 0..(v.len() - 1) - p {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }
}

pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    println!("{:?}", v);

    if v.len() <= 1 {
        return v;
    }

    let mut res = Vec::with_capacity(v.len());

    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();

    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val)
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

pub fn pivot<T: PartialOrd + Debug>(v: &mut [T]) -> usize {
    let mut p = rand::rand(v.len());

    v.swap(p, 0);
    p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p + 1, i);
            v.swap(p, p + 1);

            p += 1;
        }
    }

    p
}

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);

    quick_sort(a);
    quick_sort(&mut b[1..]);
}

struct RawSend<T>(*mut [T]);
unsafe impl<T> Send for RawSend<T> {    
}

pub fn threaded_quick_sort<T: 'static + PartialOrd + Debug + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);
    let raw_a: *mut [T] = a as *mut [T];
    let raw_s = RawSend(raw_a);

    unsafe {
        let handle = thread::spawn(move || {
            threaded_quick_sort(&mut *raw_s.0);
        });

        threaded_quick_sort(&mut b[1..]);

        handle.join().ok();
    }
}

pub fn quick_sort_threadpool<T: PartialOrd + Debug + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);

    rayon::join(
        || quick_sort_threadpool(a),
        || quick_sort_threadpool(&mut b[1..]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

        bubble_sort(&mut v);

        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }

    #[test]
    fn test_merge_sort() {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

        v = merge_sort(v);

        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }

    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 19, 8, 11, 13, 3];

        let p = pivot(&mut v);

        for x in 0..v.len() - 1 {
            assert!((v[x] < v[p]) == (x < p))
        }

        //assert_eq!(v, vec![1, 3, 4, 6, 19, 8, 11, 13])
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

        quick_sort(&mut v);

        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }

    #[test]
    fn test_threaded_quick_sort() {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

        threaded_quick_sort(&mut v);

        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }

    #[test]
    fn test_threadpooled_quick_sort() {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

        quick_sort_threadpool(&mut v);

        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
}
