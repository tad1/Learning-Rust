extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree, Ident, Span};

#[macro_use]
extern crate quote;
extern crate syn;
extern crate reqwest;
extern crate geolocation;
extern crate json;

fn impl_greet_fn(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote!{
        impl Greet for #name {
            fn greet(&self){
                println!("Hello There! It's me, {}", self.name);
            }
        }
    }
}

#[proc_macro_attribute]
pub fn ignore_me(attr: TokenStream, _item : TokenStream) -> TokenStream {
    TokenStream::new()
}


#[proc_macro_attribute]
pub fn funny_syntax(attr: TokenStream, _item : TokenStream) -> TokenStream {
    let iter = _item.into_iter();
    let mut res = TokenStream::new();
    for item in iter {
        match item {
            proc_macro::TokenTree::Group(element) => {
                println!("Group: {}", element);
                res.extend([TokenTree::Group(element)].into_iter());

            },
            proc_macro::TokenTree::Ident(element) => {
                println!("Ident: {}", element);
                if "Ping".to_owned() == element.to_string() {
                    println!("Changed: {} to Pong", element);
                    res.extend([TokenTree::Ident(Ident::new("Pong", element.span()))])                    
                } else {
                    res.extend([TokenTree::Ident(element)].into_iter());
                }

            },
            proc_macro::TokenTree::Punct(element) => {
                println!("Punct: {}", element);
                res.extend([TokenTree::Punct(element)].into_iter());

            },
            proc_macro::TokenTree::Literal(element) => {
                println!("Literal: {}", element);
                res.extend([TokenTree::Literal(element)].into_iter());

            },
        }
    }
    res
}

#[proc_macro_derive(GreetFn)]
pub fn derive_greet_fn(_item : TokenStream) -> TokenStream {
    let s = _item.to_string();
    println!("{}", s);
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_greet_fn(&ast);

    // Interesting! I can do things at compile time!
    println!("It's a compile time messeage!");

    // Can I get weather on compile time?
    // There's only one way to find out!
    let resp = reqwest::blocking::get("https://httpbin.org/ip").unwrap()
        .json::<std::collections::HashMap<String, String>>().unwrap();
    let ip = resp.get("origin").unwrap();
    println!("Compiled at: {:#?}", ip);
    let city = geolocation::find(ip).unwrap();
    println!("You are in {} at {}", city.city, city.country);
    let req = format!("https://api.weather.com/v3/aggcommon/v3alertsHeadlines;v3-wx-observations-current;v3-location-point?apiKey=e1f10a1e78da46f5b10a1e78da96f525&geocodes={},{}&language=en-US&units=e&format=json", city.latitude, city.longitude).to_owned();
    let resp = reqwest::blocking::get(req).unwrap();
    let result = resp.text().unwrap();
    let json = json::parse(&result).unwrap();
    println!("It's {}, and the temperature currently is {} °F", json[0]["v3-wx-observations-current"]["cloudCoverPhrase"], json[0]["v3-wx-observations-current"]["temperature"]);
    println!("Have a nice day!");

    // Fucking beautiful!
    gen.parse().unwrap()
}