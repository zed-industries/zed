#[derive(Clone)]
struct FibDescending {
    a: usize,
    b: usize,
}

impl FibDescending {
    fn summing_to(sequence_length: usize) -> FibDescending {
        let mut a = 0;
        let mut b = 1;
        let mut sum = 1;
        while sum <= sequence_length {
            let temp = a;
            a = b;
            b = temp + b;
            sum += b;
        }

        FibDescending { a, b }
    }
}

impl Iterator for FibDescending {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.b == 0 {
            None
        } else {
            let old_b = self.b;
            self.b = self.a;
            self.a = old_b - self.a;
            Some(old_b)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fib_descending_summing_to() {
        assert!(dbg!(FibDescending::summing_to(1000).sum::<usize>()) >= 1000);
    }
}
