pub trait Tokenizer: Iterator {
    type K: Copy;

    fn get_source(&self) -> &[Self::K];

    fn advance(&mut self, n: usize);

    fn eof(&self) -> bool {
        self.get_source().is_empty()
    }

    fn eof_at(&self, n: usize) -> bool {
        n >= self.get_source().len()
    }

    fn peek_at(&self, n: usize) -> Self::K {
        self.get_source()[n]
    }

    fn peek(&self) -> Self::K {
        self.get_source()[0]
    }

    fn maybe_peek_at(&self, n: usize) -> Option<Self::K> {
        self.get_source().get(n).copied()
    }

    fn maybe_peek(&self) -> Option<Self::K> {
        self.get_source().first().copied()
    }

    fn advance_while(&mut self, predicate: impl Fn(Self::K) -> bool) -> usize {
        let mut n = 0;

        while n < self.get_source().len() && predicate(self.peek_at(n)) {
            n += 1;
        }

        self.advance(n);

        n
    }
}
