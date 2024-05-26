// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawn threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;
use std::thread::{JoinHandle, Thread};

pub fn sum(v: Vec<i32>) -> i32 {
    let len = v.len();
    let v1 = (&v[0..len / 2]).to_vec();
    let v2 = (&v[len / 2..]).to_vec();
    let r1 = thread::spawn(move || {
        // 返回时指定类型
        v1.iter().sum::<i32>()
    });

    // 定义时指定类型
    let r2: JoinHandle<i32> = thread::spawn(move || {
        v2.iter().sum()
    });

    return r1.join().unwrap() + r2.join().unwrap();
}

pub fn sum2(v: Vec<i32>) -> i32 {
    let len = v.len();
    let (v1, v2) = v.split_at(len/2);
    let v1 = v1.to_vec();
    let v2 = v2.to_vec();
    let r1 = thread::spawn(move || {
        // 返回时指定类型
        v1.iter().sum::<i32>()
    });

    // 定义时指定类型
    let r2: JoinHandle<i32> = thread::spawn(move || {
        v2.iter().sum()
    });

    return r1.join().unwrap() + r2.join().unwrap();

}

// pub fn sum(v: Vec<i32>) -> i32 {
//     let len = v.len();
//     let num = 2;
//     let batch_size = len / 2;
//     let mut handles = vec![];
//     for i in 0..num {
//         let start = i * batch_size;
//         let end = if i == num - 1 { len } else { (i + 1) * batch_size };
//         let handle = thread::spawn(move || {
//             let mut sum = 0;
//             for j in start..end {
//                 sum += v[j];
//             }
//             return sum;
//         });
//         handles.push(handle);
//     }
//     let mut total = 0;
//
//     for h in handles {
//         total += h.join().unwrap();
//     }
//
//     return 0;
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }

    #[test]
    fn ten2() {
        assert_eq!(sum2(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
