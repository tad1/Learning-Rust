
extern crate greet_derive;
use greet_derive::funny_syntax;
use greet_derive::ignore_me;

trait Greet {
    fn greet(&self);
}

#[funny_syntax]
struct Ping {
    name : String,
    age: u8
}


#[ignore_me]
fn KIKK(_: _) {
    {asdf}
    aaaaa!();
}

fn main(){
    println!("Hello world!");
    let a = 2 + 1;
    let b = Pong{
        name: "tad1".to_string(),
        age: 21
    };
}