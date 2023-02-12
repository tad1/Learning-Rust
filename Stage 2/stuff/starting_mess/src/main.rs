
use std::ops::Add;

// Nice, I can add any value using this
fn add<T: Add<T, Output = T>>(a: T, b: T) -> T{
    a+b
}

//I guess Add is like trait, can I implement that for any structure?
// So it's a bad implementation of infinite large numbers
// we treat every u32 as bites of large number, if we need to expand precision, we add more bytes.
struct InfinitePrecision {
    // for simplicity we treat this vector like:
    // LSB -> MSB.
    integer: Vec<u32>,
    // and this one:
    // MSB -> LSB
    fraction: Vec<u32>
}

impl Add for InfinitePrecision {
    type Output = InfinitePrecision;

    fn add(self, rhs: Self) -> Self::Output {
        
        //how can I check for overflows?
        //ok rust have something what's called checked_add

        let mut int_res: Vec<u32> = vec![];

        // but I can cast it to a bigger precision
        let mut carry: u32 = 0;
        let int_size = if self.integer.len() > rhs.integer.len() { self.integer.len()} else {rhs.integer.len()};

        for i in 0..int_size {
            //NOTE: it will work now only on equal lenght Vectors
            let a:u64 = self.integer[i] as u64 + rhs.integer[i] as u64 + carry as u64;

            if a > u32::MAX.into() {
                carry = 1;
            } else {
                carry = 0;
            }

            int_res.push(a as u32);
        }
        //NOTE: I would need to push another fragment of bits if carry == 1

        InfinitePrecision{
            integer: int_res,
            fraction: vec![]
        }
    }
}

fn main() {
    println!("Hello, world! {}", add(3,3));
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let a = InfinitePrecision{
            integer: vec![u32::MAX,1],
            fraction: vec![10]
        };

        let b = InfinitePrecision{
            integer: vec![102,1],
            fraction: vec![10]
        };

        let c = add(a,b);
        println!("c[0] = {}, c[1] = {}",c.integer[0], c.integer[1]);
        // Nice! it works. But I won't implement everything. I don't need it.
    }
}