pub struct Throttler {
    length: usize,
    counter: usize,
}

impl Throttler {
    pub fn new(length: usize) -> Self {
        Throttler {
            length,
            counter: length,
        }
    }

    pub fn try_run<F>(&mut self, mut f: F)
    where
        F: FnMut() -> (),
    {
        self.counter += 1;
        if self.counter >= self.length {
            f();
            self.counter = 0;
        }
    }
}
