// After messing up with procedular macros, let's see what we can also achieve with standard macros

// Let start with lecture:
// https://doc.rust-lang.org/rust-by-example/macros.html

// ## Syntax
// It's rust, but parameters have special adnotations called *designators*
// You put $ before argument
// designator comes more from the TokenTrees
// You have blocks, literals, ident (identyficator), and more
// You can even put a TokenTree argument

// Previously I was wondering if there's overloading in rust.
// It appears that there is.
// It's more a pattern matching (like in Haskel for example)

// Here's an interesting example
macro_rules! find_min {
    ($x : expr) => ($x);
    ($x : expr, $($y:expr), +) => {
        std::cmp::min($x, find_min!($($y),+))
    }
}

// Let's create sum of elemenets!
macro_rules! sum {
    ($x : expr) => ($x);
    ($x : expr, $y : expr) => {
        $x + $y
    };
    ($x : expr, $($y: expr), +) => {
        $x + sum!($($y),+)
    }
}

macro_rules! oper {
    (SELECT * FROM $x : ident) => {
        println!("{:?}", $x);
    };
    (SELECT $n : ident FROM $x : ident) => {
        println!("{}: {}", stringify!{$n}, $x.$n);
    };
    (SELECT $n : ident, $($nm : ident),* FROM $x : ident) => {
        oper!(SELECT $n FROM $x);
        oper!(SELECT $($nm),* FROM $x);
    }
}

macro_rules! operation {
    (add $x : expr; $y : expr) => {
        println!("{} + {} = {}", stringify!{$x}, stringify!{$y}, $x + $y)
    };
    (mul $x : expr; $y : expr) => {
        println!("{} * {} = {}", stringify!{$x}, stringify!{$y}, $x * $y)
    };
}

struct Person{
    name: String,
    age : u8,
    lv: u16
}

fn main(){
    println!("{}", find_min!(2));
    println!("{}", find_min!(2, 1));
    println!("{}", find_min!(2, 1, 5*5, -1*2));
    
    println!("Sum macro:");
    println!("{}", sum!(2));
    println!("{}", sum!(2, 1));
    println!("{}", sum!(2, 1, 5*5, -1*2));

    operation!(add 2*2; 2);
    operation!(mul 2; 2+2);

    let person = Person{
        name: "Gandalf".to_owned(),
        age: 99,
        lv: 1003
    };

    oper!(SELECT name, lv FROM person);
}