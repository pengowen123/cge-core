// A simple stack implementaton
// Works by keeping track of the number of values currently on the stack in the `count` field
// Popped values aren't actually deleted, but `count` tricks `pop` into reading earlier elements, so
// the popped values are never read, and are overwritten if enough values are pushed
pub struct Stack<'a> {
    slice: &'a mut [f64],
    len: usize,
    count: usize
}

impl<'a> Stack<'a> {
    pub fn new(slice: &mut [f64]) -> Stack {
        let len = slice.len();

        Stack {
            slice: slice,
            len: len,
            count: 0
        }
    }

    pub fn push(&mut self, val: f64) {
        self.slice[self.count] = val;
        self.count += 1;
    }

    pub fn pop(&mut self, count: usize) -> &[f64] {
        self.count -= count;

        let slice = &self.slice[0..self.count + count];

        &slice[slice.len() - count..]
    }
}
