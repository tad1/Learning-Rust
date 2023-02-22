
use std::{io::{self, Write, Read}, process::Command, path::Path, fs::File};
use serde::{Serialize, Deserialize};

// Guide
// - Start with a simple idea, don't think about making that modular (you will clean your code in next iteration)
//      once you have a prototype, you can start thinking on implementation
// If you want to clear screen checkout: https://doc.rust-lang.org/std/process/struct.Command.html


// Modifications:
// - Save the list, in cache, or executed directory
///         for save instead of making it manual, try find existing solution.
///         If you are not famillar with concept of serialization check:
///             find a good resource about overall conecpts in serialization
//      - and add it to your DevTools folder
// - Allow your to navigate thought the list
// Create separate buffer (add link)


// Idea:
// - What if each project would be an tool that would help you in futher projects?
// - For futher modifications: "Why you would use app, insead of text file?"


#[derive(Serialize, Deserialize)]
struct ToDoList {
    list: Vec<Item>
}

impl ToDoList {
    fn load(path : &Path){
        
    }
    fn save(path : &Path){
        
    }
    fn new() -> ToDoList{
        ToDoList { list: vec![] }
    }
}

//TODO: add deserializer

#[derive(Serialize, Deserialize)]
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

    fn clear(){
        if cfg!(target_os = "windows") {
            Command::new("cmd").arg("/C").arg("cls").status().expect("Can't execute 'cls' command");
        } else {
            Command::new("sh").arg("-C").arg("clear").status().expect("Can't execute 'clear' command");
        }
    }

    let mut list : ToDoList = ToDoList::new();
    let mut input = String::new();

    loop {
        clear();
        println!("Your List: ");
        for element in &list.list {
            println!("{}", element);
        }
        print!("\n> ");
        let _ = io::stdout().flush();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        
        let values = input.split_once(" ");
        if let Some(("add", commmand)) = values {
            list.list.push(Item { title: commmand.trim_end().to_string(), completed: false });
        } else if let Some(("toggle", index)) = values{
            let val = index.trim().parse::<usize>();

            if let Err(err) = &val {
                println!("{} is not an integer", index);   
            } else {
                let val = val.unwrap();
                if val < list.list.len() {
                    list.list[val].completed = !list.list[val].completed;
                } else {
                    println!("Out of index");
                }
            }
        } else if let Some(("save", _)) = values {
            let save = serde_json::to_string(&list).expect("Can't serialize data!");
            let mut buffer = File::create("./todo.json").expect("Can't create file!");
            buffer.write(save.as_bytes()).expect("Can't wire to buffer");
        } else if let Some(("load", _)) = values {
            let res = File::open("./todo.json");
            if let Ok(mut file) = res {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer).expect("Can't read from file");
                list = serde_json::from_str(&buffer).unwrap();
            }
        }
        

    }
}
