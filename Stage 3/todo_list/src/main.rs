
use std::{io::{self, Write, Read}, process::Command, path::Path, fs::File, error::Error};
use serde::{Serialize, Deserialize};
use tui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, 
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode}
};

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
// 

// API
//      Allow for usign it with args

/// TUI:
///     - Allow your to navigate thought the list
//      Create separate buffer (add link)
// Alternative:
/// https://docs.rs/crossterm/latest/crossterm/


// Idea:
// - What if each project would be an tool that would help you in futher projects?
// - For futher modifications: "Why you would use app, insead of text file?"


#[derive(Serialize, Deserialize, Debug)]
struct ToDoList {
    list: Vec<Item>
}


macro_rules! assert_todo_in_range {
    ($list:ident, $index:ident) => {
        if($index > $list.len() || $list.len() == 0) {
            println!("Out of range!");
            return;
        }
    };
    ($list:ident, $index:ident,$ret:ident) => {
        if($index > $list.len() || $list.len() == 0) {
            println!("Out of range!");
            return $ret;
        }
    };
}

impl ToDoList {


    fn clear(&mut self){
        self.list.clear();
    }
    
    fn load(&mut self, path : &Path) -> Result<(), io::Error>{
        self.clear();
        let mut file = File::open(path.join("todo.json"))?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).expect("Can't read from file");
        let list : ToDoList = serde_json::from_str(&buffer).expect("Can't deserialize file");
        self.list = list.list;            
        Ok(())
    }
    fn save(&self, path : &Path){
        let save = serde_json::to_string(&self).expect("Can't serialize data!");
        let mut buffer = File::create(path.join("todo.json")).expect("Can't create file!");
        buffer.write(save.as_bytes()).expect("Can't write to buffer");
    }

    fn prepend(&mut self, index : usize, element : Item){
        assert_todo_in_range!(self, index);
        self.list.insert(index, element);
    }

    fn append(&mut self, index : usize, element : Item) {
        if index + 1 > self.list.len() {
            self.list.push(element);
        } else {
            self.list.insert(index + 1, element);
        }
    }

    fn len(&self) -> usize {
        self.list.len()
    }

    fn delete(&mut self, index : usize) -> Option<Item> {
        assert_todo_in_range!(self, index, None);
        Some(self.list.remove(index))
    }

    fn toggle(&mut self, index : usize) {
        assert_todo_in_range!(self, index);
        self.list[index].completed = !self.list[index].completed;
    }

    fn new() -> ToDoList{
        ToDoList { list: vec![] }
    }
}

//TODO: add deserializer

#[derive(Serialize, Deserialize, Debug)]
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

enum Commands{
    Exit,
    Add(String),
    Delete(usize),
    Toggle(usize),
    Navigate
}

macro_rules! print_list {
    ($list:ident, $index:ident) => {
        clear();
        println!("Your List: ");
        let mut i = 0;
        for element in &($list.list) {
            if i == $index { print!(">")} else {print!(" ")}
            println!("{}", element);
            i += 1;
        }
    };
}

// UI, handle adding new element in nice way

fn main() -> Result<(), io::Error>{
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    fn clear(){
        if cfg!(target_os = "windows") {
            Command::new("cmd").arg("/C").arg("cls").status().expect("Can't execute 'cls' command");
        } else {
            Command::new("sh").arg("-C").arg("clear").status().expect("Can't execute 'clear' command");
        }
    }

    let mut list : ToDoList = ToDoList::new();
    let mut input = String::new();
    let mut index : usize = 0; //Why not to save that also?

    print_list!(list, index);
    execute!(io::stdout(), EnterAlternateScreen)?;
    let path = Path::new(".");
    list.load(&path)?;
    loop {
        print!("\n> ");
        let _ = io::stdout().flush();
        input.clear();
        // Need introduce modes
        match read()? {
            Event::FocusGained => {
                
            },
            Event::FocusLost => {
                
            },
            Event::Key(event) => {
                if event.kind == KeyEventKind::Press  {
                    match event.code {
                        KeyCode::Up | KeyCode::Char('k') => {
                            //If with CTRL, move element
                            if event.modifiers == KeyModifiers::CONTROL {
                                //assert, and move
                            }
                            if index != 0 { index -= 1; }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if index != list.list.len() - 1 { index += 1; }
                        }
                        KeyCode::Char(' ') => {
                            list.toggle(index);
                            list.save(path);
                        }
                        KeyCode::Char('a') => {
                            //Append
                            list.append(index, Item { title: String::from(""), completed: false });
                            index = if list.len() == 1 {0} else {index + 1};
                            print_list!(list, index);
                            {
                                //Enter insert mode
                                loop {
                                    let event = read()?;
                                    if let Event::Key(code) = event {
                                        if code.kind != KeyEventKind::Press {
                                            continue;
                                        }
                                        let item: &mut Item = &mut list.list[index];
                                        if code.code == KeyCode::Esc || code.code == KeyCode::Enter {
                                            break;
                                        } else if let KeyCode::Char(char) = code.code {
                                            item.title.push(char);
                                        } else if KeyCode::Backspace == code.code {
                                            item.title.pop();
                                        }
                                        print_list!(list, index);
                                    }
                                }

                            }
                            list.save(path);
                        }
                        KeyCode::Delete | KeyCode::Char('x') => {
                            // Delete selected
                            list.delete(index);
                            if index >= list.len(){
                                index = if index > 0 {index - 1} else {0};
                            }
                            list.save(path);
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    }
                }
            },
            Event::Mouse(event) => {

            },
            Event::Paste(data) => {
                
            },
            Event::Resize(width, height) => {
                
            },
        }

        print_list!(list, index);
    }
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
