
use serde::{Serialize, Deserialize};
use std::{io::{self, Write, Read}, process::Command, path::Path, fs::File, error::Error};


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

#[derive(Serialize,Deserialize,Debug)]
pub struct Item{
    pub title: String,
    pub completed : bool
} 

#[derive(Serialize,Deserialize,Debug)]
pub struct ToDoList{
    pub items: Vec<Item>,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "- [{}] {}", 
        if self.completed {"X"} else {" "},
    self.title)
    }
}

impl ToDoList {


    pub fn clear(&mut self){
        self.items.clear();
    }
    
    pub fn load(&mut self, path : &Path) -> Result<(), io::Error>{
        self.clear();
        let mut file = File::open(path.join("todo.json"))?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).expect("Can't read from file");
        let list : ToDoList = serde_json::from_str(&buffer).expect("Can't deserialize file");
        self.items = list.items;            
        Ok(())
    }
    pub fn save(&self, path : &Path){
        let save = serde_json::to_string(&self).expect("Can't serialize data!");
        let mut buffer = File::create(path.join("todo.json")).expect("Can't create file!");
        buffer.write(save.as_bytes()).expect("Can't write to buffer");
    }

    pub fn prepend(&mut self, index : usize, element : Item){
        assert_todo_in_range!(self, index);
        self.items.insert(index, element);
    }

    pub fn append(&mut self, index : usize, element : Item) {
        if index + 1 > self.items.len() {
            self.items.push(element);
        } else {
            self.items.insert(index + 1, element);
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn delete(&mut self, index : usize) -> Option<Item> {
        assert_todo_in_range!(self, index, None);
        Some(self.items.remove(index))
    }

    pub fn toggle(&mut self, index : usize) {
        assert_todo_in_range!(self, index);
        self.items[index].completed = !self.items[index].completed;
    }

    pub fn edit(&mut self, index : usize, value : &String) {
        
    }

    pub fn new() -> ToDoList{
        ToDoList { items: vec![] }
    }
}

