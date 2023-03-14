
use std::{io::{self, Write, Read}, process::Command, path::{Path, PathBuf}, fs::File, error::Error};
use regex::Regex;
use serde::{Serialize, Deserialize};
use tui::{backend::CrosstermBackend, Terminal};
use crossterm::{
	event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
	execute, 
	terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode}
};
use clap::{Parser, Subcommand, ValueEnum};

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
	/// Set a custom todo file.
	#[arg(short, long, value_name = "FILE")]
	file: Option<PathBuf>,
	#[command(subcommand)]
	command: Option<Commands>
}

#[derive(Subcommand, Debug, PartialEq)]
enum Commands{
	Visual,
	/// Add item with <NAME> to the list 
	Add {
		/// items needed to be added, to the list, separated with space
		names :Vec<String>
	},
	Toggle {
		mode : Option<ToggleMode>,
		#[command(subcommand)]
		reference: ReferenceMode,

	},
	///Print out list to stdout buffer
	Print,
	Remove {
		#[command(subcommand)]
		reference: ReferenceMode,

	},
	Edit {
		index : u16,
		value : String
	},
	Move {
		index : u8,
		offset : i8
	},

	// * What about adding groups?
	// * That could be a nice addition to this project
}

#[derive(Debug, PartialEq, Subcommand, Clone)]
enum ReferenceMode{
	Indexes {
		indexes: Vec<usize>
	},
	Pattern {
		pattern: String
	}
}

#[derive(Clone, Debug, ValueEnum, PartialEq)]
enum ToggleMode {
	On,
	Off,
	Swap
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
		println!("\nx - delete, a - append, p - prepend, SPACE - toggle");
		println!("\tEnter: accept text in item");
		println!("Navigation: up/down or j/k, CTRL to move items");
		println!("ESC to exit");
	};
}

//*NOTE: I'm using regex here!
fn handle_command(list : &mut ToDoList,command : Commands) {
	match command {
    Commands::Visual => {},
    Commands::Add { names } => {
		for name in names{
			list.push(Item { title: name, completed: false });
		}
	},
    Commands::Toggle { mode, reference } => {
		let _mode = mode.unwrap_or(ToggleMode::Swap);
		let value = if _mode == ToggleMode::On {true} else {false};
		
		match reference {
    		ReferenceMode::Indexes { indexes } => {
				for index in indexes {
					//check if regex passes
					if _mode == ToggleMode::Swap {
						list.toggle(index as usize);
					} else {
						list.set(index as usize, value);
					}
				}
			},
    		ReferenceMode::Pattern { pattern } => {
				let re = Regex::new(pattern.as_str()).unwrap();
				for mut item in list.items.iter_mut() {
					if re.is_match(&item.title) {
						// toggle
						if _mode == ToggleMode::Swap {
							item.completed = !item.completed;
						} else {
							item.completed = value;
						}
					}
				}
				
			},
		}
	},
    Commands::Print => {
		for element in &list.items {
			println!("{}", element);
		}
	},
    Commands::Remove { reference } => {
		//Check if pattern is set
		match reference {
    		ReferenceMode::Indexes { indexes } => {
				for index in indexes {
					list.delete(index);
				}
			},
    		ReferenceMode::Pattern { pattern } => {
				let re = Regex::new(pattern.as_str()).unwrap();
				list.items.retain(|item| !re.is_match(&item.title));
			},
}
	},
    Commands::Edit { index, value } => {
		list.edit(index as usize, &value);
	},
    Commands::Move { index, offset } => {
		list.r#move(index as usize, offset);
	},
}
}

// UI, handle adding new element in nice way
fn handle_cli(args : Cli) -> Result<(), io::Error> {
	let path = args.file.unwrap_or(PathBuf::from("./todo.json"));
	let command = args.command.expect("No command given");
	let mut list : ToDoList = ToDoList::new();
	list.load(&path)?;

	handle_command(&mut list, command);
	list.save(&path);
	Ok(())
}

fn clear(){
	if cfg!(target_os = "windows") {
		Command::new("cmd").arg("/C").arg("cls").status().expect("Can't execute 'cls' command");
	} else {
		Command::new("sh").arg("-C").arg("clear").status().expect("Can't execute 'clear' command");
	}
}

fn handle_visual(args : Cli) -> Result<(), io::Error> {

	let mut list : ToDoList = ToDoList::new();
	let mut input = String::new();
	let mut index : usize = 0; //Why not to save that also?
	let path_buf : PathBuf = args.file.unwrap_or(PathBuf::from("./todo.json"));
	let path: &Path = path_buf.as_path();
	execute!(io::stdout(), EnterAlternateScreen)?;
	list.load(&path)?;
	print_list!(list, index);

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
							if event.modifiers == KeyModifiers::CONTROL && index > 0{
								list.r#move(index, 1);
								list.save(path);
							}
							if index != 0 { index -= 1; }
						}
						KeyCode::Down | KeyCode::Char('j') => {
							if event.modifiers == KeyModifiers::CONTROL && index < list.len() - 1{
								list.r#move(index, -1);
								list.save(path);
							}
							if index != list.len() - 1 { index += 1; }
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
			Event::Mouse(_event) => {

			},
			Event::Paste(_data) => {
				
			},
			Event::Resize(_width, _height) => {
				
			},
		}

		print_list!(list, index);
	}
	execute!(io::stdout(), LeaveAlternateScreen)?;

	Ok(())
}

fn main() -> Result<(), io::Error>{
	let args = Cli::parse();

	if args.command == Some(Commands::Visual) {
		return handle_visual(args);
	} else {
		return handle_cli(args);
	}
}
