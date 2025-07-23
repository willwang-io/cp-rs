struct Fenwick {
    n: usize,
    bit: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick { n, bit: vec![0; n + 1] }
    }

    fn update(&mut self, idx: usize, val: i64) -> () {
        let mut i = idx + 1;
        while i <= self.n {
            self.bit[i] += val;
            i += i & (!i + 1);
        }
    }

    fn query(&self, idx: usize) -> i64 {
        let mut s = 0i64;
        let mut i = idx + 1;
        while i > 0 {
            s += self.bit[i];
            i -= i & (!i + 1);
        }
        s
    }

    fn range_query(&self, l: usize, r: usize) -> i64 {
        if l > 0 {
            self.query(r) - self.query(l - 1)
        } else {
            self.query(r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Fenwick;

    #[test]
    fn new_initializes_with_zeroes() {
        let n = 10;
        let fenwick = Fenwick::new(n);
        for i in 0..n {
            assert_eq!(fenwick.query(i), 0);
        }
    }

    #[test]
    fn multiple_updates_accumulate() {
        let mut fenwick = Fenwick::new(5);
        fenwick.update(1, 2);
        fenwick.update(1, 3);
        fenwick.update(4, 5);

        assert_eq!(5, fenwick.query(1));
        assert_eq!(5, fenwick.query(3));
        assert_eq!(10, fenwick.query(4));
    }

    #[test]
    fn range_query_full_and_partial() {
        let mut fenw = Fenwick::new(6);
        for i in 0..6 {
            fenw.update(i, (i + 1) as i64);
        }
        assert_eq!(fenw.range_query(0, 5), 21);
        assert_eq!(fenw.range_query(2, 4), 3 + 4 + 5);
        assert_eq!(fenw.range_query(3, 3), 4);
    }
}