use std::mem;

fn main(){
    loop{
        let a : Box<u32> = Box::new(0);
        mem::forget(a);
    }
}