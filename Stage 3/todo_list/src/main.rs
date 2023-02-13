
use std::io::{self, Write};

// Guide
// - Start with a simple idea, don't think about making that modular (you will clean your code in next iteration)
//      once you have a prototype, you can start thinking on implementation

// Modifications:
// - Save the list, in cache, or executed directory
//      - and add it to your DevTools folder
// - Allow your to navigate thought the list
// Create separate buffer (add link)


// Idea:
// - What if each project would be an tool that would help you in futher projects?
// - For futher modifications: "Why you would use app, insead of text file?"


struct Item {
    title : String,
    completed : bool
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "- [{}] {}", 
            if self.completed {"X"} else {" "},
        self.title)
    }
}

fn main() {
    let mut list : Vec<Item> = vec![];
    let mut input = String::new();
        
    loop {
        println!("Your List: ");
        for element in &list {
            println!("{}", element);
        }
        print!("\n> ");
        let _ = io::stdout().flush();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        
        let values = input.split_once(" ");
        if let Some(("add", commmand)) = values {
            list.push(Item { title: commmand.trim_end().to_string(), completed: false });
        } else if let Some(("toggle", index)) = values{
            let val = index.trim().parse::<usize>();

            if let Err(err) = &val {
                println!("{} is not an integer", index);   
            } else {
                let val = val.unwrap();
                if val < list.len() {
                    list[val].completed = !list[val].completed;
                } else {
                    println!("Out of index");
                }
            }

        }
        

    }
}
