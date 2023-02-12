// So I never get to known about the attributes
// I mean somethings like this:
// #[test], #[cfg(test)]

// Found this:
// https://doc.rust-lang.org/reference/attributes.html

// So finally I got to known this attribute!

trait Greet {
    fn greet(&self) {
        println!("Hello there!");
    }
}


/* It appears that you can't just derive a trait
attributes are the compile time information
so you need to create a macro that will be evaluated at compile time, and would implement the default trait implementation 
#[derive(Greet)]
struct Person {
    name: String,
    age: u8
}
*/

// Here's answer to my question
// https://doc.rust-lang.org/reference/attributes/derive.html
// https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros

// What! You can write funtions that will be executed at compile time!
// The fuck? You can literary extend the rust syntax with that!
// Like there is a TokenStream thing.
// I need to explore that later.
// NEXT: can I extend rust syntax by using rust?

// There's a problem, I need to add create
// But I use custom build system.
// That means I will need to, again, extend it.

// I done that

// I found this:
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/procedural-macros.html

// It appears that I need to create extern create for that

extern crate greet_derive;
use greet_derive::GreetFn;

#[derive(GreetFn)]
struct Person {
    name: String,
    age: u8
}


fn main(){
    let person = Person{
        name: "Obi-Wan Kenobi".to_owned(),
        age: 31
    };
    person.greet();
}