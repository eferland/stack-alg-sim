use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct OPTStack<T> {
 	pub stack: Vec<Option<Box<Node<T>>>>,
    pub opttrace: OPTTrace<T>
}

#[derive(Debug)]
pub struct OPTTrace<T> {
 	pub trace: Vec<Node<T>>,
    pub bank: HashMap<T, usize>
}

impl<T: Eq + Hash + Copy> OPTTrace<T> {

	pub fn new() -> OPTTrace<T> {
		OPTTrace {
			trace: Vec::<Node<T>>::new(),
            bank: HashMap::new()
		}
	}

    pub fn opt_value(val: &Node<T>, pos: usize) -> Option<usize> {

    }

    pub fn last_instance(&self, val: &Node<T>) -> Option<usize> {
        if self.bank.contains_key(&val.val) {
            let mut pos = self.bank[&val.val];
            while pos<self.trace.len() {
                match self.trace[pos].next {
                    Some(num) => {
                        pos += num-1;
                    }
                    None => {
                        return Some(pos);
                    }
                }
                pos += 1;
            }
        }
        return None;
        // let mut pos = 0;
        // while pos<self.trace.len() {
        //     if self.trace[pos]==*val {
        //         match self.trace[pos].next {
        //             Some(num) => {
        //                 pos += num-1;
        //             }
        //             None => {
        //                 return Some(pos);
        //             }
        //         }
        //     }
        //     pos += 1;
        // }
        // return None;
    }

    pub fn push(&mut self, val: T) {
        let insert = Node::<T>::new(val);
        match self.last_instance(&insert) {
            Some(pos) => {
                let nextval = self.trace.len()-pos;
                self.trace[pos].set_next(nextval);
            }
            None => {
                self.bank.insert(val, self.trace.len());
            }
        }
        self.trace.push(insert);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Node<T> {
    pub val: T,
    pub next: Option<usize>
}

impl<T: Eq + Hash + Copy> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<T: Eq + Hash + Copy> Node<T> {
    pub fn new(input: T) -> Node<T> {
        Node {
            val: input,
            next: None
        }
    }

    pub fn set_next(&mut self, number: usize) {
        self.next = Some(number);
    }
}

impl<T: Eq + Hash + Copy> OPTStack<T> {
	pub fn new(tr: OPTTrace<T>) -> OPTStack<T> {
		OPTStack {
			stack: Vec::<Option<Box<Node<T>>>>::new(),
            opttrace: tr
		}
	}

    pub fn rec_access(&mut self, pos: usize) -> Option<u32> {
        let node = self.opttrace.trace[pos];
		if self.stack.len() == 0 {
			self.stack.push(Some(Box::new(node)));
			return None;
		}

	    if **self.stack[0].as_ref().unwrap() == node {
			return Some(1);
	    }

        // let mut last = self.stack[0].take(); 
	    // for pos in 1..self.stack.len() {
		// 	let temp = self.stack[pos].take();
		// 	self.stack[pos] = last;
		// 	last = temp; 
		// 	if **last.as_ref().unwrap() == val {
		//     	self.stack[0] = last;
		// 		return Some(pos as u32 + 1);
		// 	}
	    // }
		// // a cold miss
		// self.stack.push( last ); // add to the end of the vector
	    // self.stack[0] = Some(Box::new(val));
	    return None;
        //self.stack[0..pos].sort_by ??

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        let mut first = Node::<u32>::new(5);
        first.set_next(1);
        let mut second = Node::<u32>::new(5);
        second.set_next(2);
        assert_eq!(first, second);
    }

    #[test]
    fn equality2() {
        let mut first = Node::<u32>::new(5);
        first.set_next(1);
        let second = Node::<u32>::new(5);
        assert_eq!(first, second);
    }

    #[test]
    fn ineq() {
        let mut first = Node::<u32>::new(5);
        first.set_next(1);
        let second = Node::<u32>::new(6);
        assert_ne!(first, second);
    }

    #[test]
    fn ineq2() {
        let mut first = Node::<u32>::new(5);
        first.set_next(1);
        let mut second = Node::<u32>::new(6);
        second.set_next(1);
        assert_ne!(first, second);
    }

    #[test]
    fn push_check() {
        let mut trace = OPTTrace::<u32>::new();

        trace.push(3);
        trace.push(5);
        trace.push(2);
        trace.push(5);
        assert_eq!(trace.trace[1].next, Some(2));
    }

    #[test]
    fn push_check2() {
        let mut trace = OPTTrace::<&str>::new();

        trace.push("hello");
        trace.push("world");
        trace.push("yurr");
        trace.push("world");
        assert_eq!(trace.trace[0].next, None);
    }
}
