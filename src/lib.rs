pub use std::ops::{Range, Index};

#[derive(Clone)]
pub struct Fibs(Vec<f64>);

impl Fibs {
    pub fn new(to_n: usize) -> Self {
        assert!(to_n > 0);

        let mut vec: Vec<usize> = Vec::with_capacity(to_n + 1);
        vec.extend_from_slice(&[1, 1]);
        for _ in 1..to_n {
            vec.push(vec[vec.len() - 2] + vec[vec.len() - 1]);
        }

        Self(vec.into_iter().map(|x| x as f64).collect())
    }

    pub fn max_n(&self) -> usize {
        self.0.len() - 1
    }
}

impl Index<usize> for Fibs {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

pub fn search(range: Range<f64>, eps: f64, f: impl Fn(f64) -> f64) -> f64 {
    assert!(eps > 0.0);

    let n = epsilon_to_n(range.clone(), eps);
    let fibs = Fibs::new(n + 2);
    search_with_n(range, fibs, n, f)
}

pub fn search_with_n(range: Range<f64>, fibs: Fibs, n: usize, f: impl Fn(f64) -> f64) -> f64 {
    assert!(n > 0);
    assert!(fibs.max_n() >= n + 2);

    let Range{ mut start, mut end } = range;
    let mut x1 = start + (end - start) * fibs[n] / fibs[n + 2];
    let mut x2 = start + (end - start) * fibs[n + 1] / fibs[n + 2];
    let mut fx1 = f(x1);
    let mut fx2 = f(x2);
    for n in (1..n).rev() {
        if fx1 < fx2 {
            end = x2;
            x2 = x1;
            fx2 = fx1;
            x1 = start + (end - start) * fibs[n] / fibs[n + 2];
            fx1 = f(x1);
        } else {
            start = x1;
            x1 = x2;
            fx1 = fx2;
            x2 = start + (end - start) * fibs[n + 1] / fibs[n + 2];
            fx2 = f(x2);
        }
    }

    0.5 * (start + end)
}

pub fn epsilon_to_n(range: Range<f64>, eps: f64) -> usize {
    assert!(eps > 0.0);

    ((5.0f64.sqrt() * (range.end - range.start) / eps).ln() 
     / ((1.0 + 5.0f64.sqrt()) / 2.0).ln() 
     - 1.0) as usize
}
