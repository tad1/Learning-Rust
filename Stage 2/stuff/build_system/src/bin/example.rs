

fn test_lsp(name: String) -> bool {
    if name == "asdf".to_string(){
       return true
    }
    false
}


#[cfg(test)]
mod test{
    #[test]
    fn random_test() {
        // assert_eq!(1u32,1u8)
    }
}


// Yay! It's working!
// Now, how can I enable LSP on this file also?
fn main(){
    let name = "Lukgla";
    println!("Hello {}!", name);
}