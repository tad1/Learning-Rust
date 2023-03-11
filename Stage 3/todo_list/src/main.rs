
use std::{io::{self, Write, Read}, process::Command, path::Path, fs::File, error::Error};
use serde::{Serialize, Deserialize};
use tui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, 
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode}
};
use clap::Parser;

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

// - Make that a library!
// Split to TUI, Logic, and API

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
use tdl::{Item, ToDoList};






//TODO: add deserializer

#[derive(Debug, Parser)]
struct Cli {
    #[arg(default_value = "visual")]
    command : String
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
        for element in &($list.items) {
            if i == $index { print!(">")} else {print!(" ")}
            println!("{}", element);
            i += 1;
        }
    };
}

// UI, handle adding new element in nice way

fn main() -> Result<(), io::Error>{
    let args = Cli::parse();

    println!("{}", args.command);
    return Ok(());
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
                            if index != list.items.len() - 1 { index += 1; }
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
                                        let item: &mut Item = &mut list.items[index];
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
