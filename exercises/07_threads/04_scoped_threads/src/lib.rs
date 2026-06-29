// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let mut s1 = 0;
    let mut s2 = 0;
    thread::scope(|scope| {
        scope.spawn(|| {
            let v1 = &v[0..v.len()/2];
            s1 = v1.iter().sum::<i32>();
        });

        scope.spawn(|| {
            let v1 = &v[v.len()/2..];
            s2 = v1.iter().sum::<i32>();
        });

    });
    s1 + s2
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
