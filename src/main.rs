mod sort;
mod rand;
mod list;

use sort::*;

fn sort_some() {
    {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

        bubble_sort(&mut v);

        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
    {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

        v = merge_sort(v);

        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
    {
        let mut v = vec![4, 6, 1, 19, 8, 11, 13, 3];

        let p = pivot(&mut v);

        for x in 0..v.len() - 1 {
            assert!((v[x] < v[p]) == (x < p))
        }

        //assert_eq!(v, vec![1, 3, 4, 6, 19, 8, 11, 13])
    }
    {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

        quick_sort(&mut v);

        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
    {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

        threaded_quick_sort(&mut v);

        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
    {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

        quick_sort_threadpool(&mut v);

        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }    
}

fn main() {
    sort_some();
}
