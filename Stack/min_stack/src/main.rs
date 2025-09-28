struct MinStack {
    pub vc: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        MinStack {
            vc: Vec::<i32>::new()
        }
    }
    
    fn push(&mut self, val: i32) {
        self.vc.push(val);
    }
    
    fn pop(&mut self) {
        self.vc.pop();
    }
    
    fn top(&self) -> i32 {
        *self.vc.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.vc.iter().min().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

fn main() {
    let mut obj = MinStack::new();
    println!("{:?}", obj.push(-2));
    println!("{:?}", obj.push(0));
    println!("{:?}", obj.push(-3));
    println!("{:?}", obj.get_min()); // return -3
    println!("{:?}", obj.pop());
    println!("{:?}", obj.top());    // return 0
    println!("{:?}", obj.get_min()); // return -2

    println!("{:?}", obj.vc);
}
