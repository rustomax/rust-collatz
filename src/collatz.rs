pub struct Collatz {
    curr: u64,
    done: bool,
}

impl Collatz {
    pub fn new(start: u64) -> Collatz {
        Collatz {
            curr: start,
            done: false,
        }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {

        if self.done { return None };

        let result = Some(self.curr);
        match (self.curr, self.curr % 2 == 0) {
            (1, _    ) => { self.done = true },
            (_, true ) => { self.curr = self.curr / 2 },
            (_, false) => { self.curr = self.curr * 3 + 1 },
        }

        result
    }
}

#[test]
fn test_collatz() {

    let v = vec![10, 5, 16, 8, 4, 2, 1];

    let mut c = Collatz::new(10);
    for i in 0..7 {
        assert_eq!(c.next(), Some(v[i]));
    }
    
    assert_eq!(c.next(), None);
}