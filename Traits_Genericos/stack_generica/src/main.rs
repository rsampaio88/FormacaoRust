/*
   LIFO:    Last In First Out

   new : create new stack
   add : add element to the top of stack
   last : return the last element
   remove : remove the last element of stack
   size : return size of stack
   clear : clears stack

   the stack is filed
*/

use std::vec::Vec;

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn remove(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn clear(&mut self) {
        self.items.clear();
    }

    fn size(&self) -> usize {
        self.items.len()
    }

    fn last(&self) -> Option<&T> {
        self.items.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_operations() {
        let mut stack = Stack::new();

        stack.add(8);
        stack.add(0);
        stack.add(23);
        stack.add(90);
        stack.add(7);
        stack.add(54);

        assert_eq!(stack.size(), 6);

        assert_eq!(*stack.last().unwrap(), 54);

        assert_eq!(stack.remove().unwrap(), 54);
        assert_eq!(stack.size(), 5);

        stack.add(40);
        assert_eq!(stack.size(), 6);
        stack.clear();
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_empty_stack() {
        let mut stack: Stack<i32> = Stack::new();

        assert_eq!(stack.size(), 0);
        assert_eq!(stack.remove(), None);
        assert_eq!(stack.last(), None);
    }
}

fn main() {}
