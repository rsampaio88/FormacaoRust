
/*
    LIFO:    Last In First Out

    -add element to the top of stack
    -return the last element
    -remove the last element
    -size of stack
    - clear stack

    the stack is filed 
 */


struct stack {
    items: Vec<T>,
}

impl<T> stack<T> {

    fn new() -> stack {
        stack {items: Vec::new()}
    }

    fn add(&mut stack, item: T) {
        stack.items.push(item);
    }

    fn remove(&mut stack, item: T) -> Option<&T> {
        stack.items.pop()
    }

    fn clear(&mut stack)
}

#[cfg(teste)]

fn main() {}
    todo!()
}
