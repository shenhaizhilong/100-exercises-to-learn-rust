// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

pub fn sum(v: Vec<i32>) -> i32 {
    let len = v.len();
    let mid = len / 2;
    let (v1, v2) = v.split_at(mid);
    let s1 = std::thread::scope(|scope| {
        let h1 = scope.spawn(|| {
            v1.iter().sum::<i32>()
        });
        let h2 = scope.spawn(|| {
            v2.iter().sum::<i32>()
        });
        return h1.join().unwrap() + h2.join().unwrap();
    });
    return s1;
}

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
}
