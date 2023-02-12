use std::fmt::Display;



struct Container<T> {
    value: T
}
// It seems that it would be impossible to implement all wrapping for any type
// impl From<u32> for Container<u32> {
//     fn from(value: u32) -> Self {
//         Container { value: value }
//     }
// }

// Found solution
impl<T> From<T> for Container<T> {
    fn from(value: T) -> Self {
        Container { value: value }
    }
}

// ToString is automatically implemented!
impl<T: Display> Display for Container<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Container {{{}}}", self.value)
    }
}

// So I started reading.
// And it appears that rust optimalize Generics, so we don't have to pay on runtime.

fn main(){
    let val = Container::from(123);
    println!("{}", val);
    println!("{}", val.to_string());
}