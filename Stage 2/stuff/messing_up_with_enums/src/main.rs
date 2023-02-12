// We known that Option, and Results uses enums.
// Is there's something interesting we can use enums for uncomventional purpose?

// Let's create math from enums!
// Because why fucking not?

// There's a concept about using the set theory, where you can define all numbers as sets
// https://en.wikipedia.org/wiki/Set-theoretic_definition_of_natural_numbers

use std::{ops::Add, fmt::Display};

//It apprers that I need to use Box for recursive enums
enum Number {
    Null,
	Set(Vec<Number>)
}


impl AsMut<Number> for Number {
    fn as_mut(&mut self) -> &mut Number {
        self
    }
}

impl Clone for Number {
    fn clone(&self) -> Self {
        match self {
            Self::Null => Self::Null,
            Self::Set(arg0) => Self::Set(arg0.clone()),
        }
    }

    fn clone_from(&mut self, source: &Self)
    {
        *self = source.clone()
    }
}

impl Into<u32> for Number {
    fn into(self) -> u32 {
		let mut i = 0;
		let mut num = &self.clone();

		while let Number::Set(ref x) = num {
			i += 1;
			num = x.last().unwrap(); 
		}
		i
    }
}

impl Add for Number {
	type Output = Number;
	fn add(self, rhs: Self) -> Self::Output {
		//We can use recursion / iterations for that
		let mut num = &rhs;
		let mut res = self.clone();
		while let Number::Set(ref x) = num {
			
			let v: &mut Vec<Number>;
			let c: Number = res.clone(); 
			// wrap result
			// move vector from number
			match res {
    			Number::Null => {
					res = Number::Set(vec![]);
					continue;
				},
    			Number::Set(ref mut val) => {
					v = val;
				},
			};
			v.push(c);
			
			// unwrap iterator
			num = x.last().unwrap()
		};
		res
	}

}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
    		Number::Null => {
				write!(f, "Ø")
			},
    		Number::Set(x) => {
				// "{"
				_ =write!(f,"{{");
				// for 
				let mut it = x.iter().peekable();
				while let Some(n) = it.next() {
					_ =write!(f, "{}", n);
					if !it.peek().is_none() {
						_ =write!(f, ", ");
					}
				}
				// "}"
				write!(f,"}}")
			},
		}
    }
}



// Look! It's working!
// That's enough for me.
fn main() {
    println!("Hello, world!");
	let zero = Number::Null;
	let one = Number::Set(vec![zero.clone()]);
	let two = one.clone() + one.clone();
	let four = two.clone() + two.clone();
	let five = four.clone() + one.clone();
	println!("0: {}", zero);
	println!("1: {}", one);
	println!("2: {}", two);
	println!("4: {}", four);
	println!("5: {}", five);
	let dec : u32 = five.into();
	assert_eq!(dec, 5);
}
