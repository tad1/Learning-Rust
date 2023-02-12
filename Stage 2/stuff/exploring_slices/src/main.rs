//There's a concept of slices in rust.

// We can have string slices the &str, but there's more!
//Up next: Multithreading and macros

// I found this:
// https://doc.rust-lang.org/rust-by-example/primitives/array.html

// There are arrays and there are slices
// arrays have known length at complie time
// slices don't
// slices are more like how arrays look in C, it's a pointer to data, and lenght of data.
// and like in C you can have slice that overlaps another slice, or have slice inside a slice.

use core::slice;


fn example_array() {
    // so this is an example array
    let arr = [1, 2, 3];
    // this gets elements [1,2)
    let slice = &arr[0..2];
    for el in slice  {
        print!("{}, ", el);
    }
    // this will panic, it's out of scope
    print!("{}", slice[2]);
}

fn more_slices() {
    //this is how you define string slice
    let slice: &str = "&[1,2,3]";

    // this makes we wonder.
    // you can get slices from array
    // you create something like a View for a array

    // but can you modify that?
    // there's only one way to find that out!
    {
        // a test without mutability
        let arr: [u32; 5] = [0; 5];
        let s: &[u32] = &arr[0..2]; // it seems by syntax that it's more like borrow
        let a: &[u32] = &arr[1..3]; // so it complies
        assert_eq!(s[1], a[0]);
    }
    
    {
        //adding mutability
        let mut arr: [u32; 6] = [1; 6];
        let s: &mut [u32] = &mut arr[0..3];
        s[0] = 4;
        
        assert!(arr[0] == 4);
        //a compilation error
        // it appears that once you get mutable slice, it will borrow the whole array
        //arr[3] = 2;
        //assert!(s[1] == 2);
    }
}

struct color {
    r: u8,
    g: u8,
    b: u8
}

fn more_complicated_slices() {
    //but now, can I get slice from random memory?
    // and even more can I thread slice as different data type?

    {
        let arr: [u32; 6] = [u32::MAX, 5, 1, 1, 0, 1];
        let slc = [0u32; 1];
        // a compile error!
        //let slice: &[i64] = &arr[0..5];

        // [0,5)
        let slice: &[u32] = &arr[0..5];
        // and this also give a complie error        
        //let another_slice : &[i32]= &slice; 
    }

    // But can I use slice with structs?
    {
        let colors = [color{r:1 ,g:1, b:0}, color{r:0 ,g:0, b:255}, color{r:100 ,g:100, b:0}];
        let some_colors = &colors[..2];

        for color{r, g, b} in some_colors{
            println!("color: {}, {}, {}", r, g, b);
        }
    }
    
}


fn main() {
    //example_array();
    //more_slices();
    more_complicated_slices();
    println!("Hello, world!");
}
