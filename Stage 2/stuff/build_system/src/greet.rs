extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(GreetFn)]
pub fn derive_greet_fn(_item : TokenStream) -> TokenStream {
    "fn greet() { println!(\"Hello There!\"); }".parse().unwrap()
}