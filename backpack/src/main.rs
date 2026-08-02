use core::hash;
//
//WORKSPACE     1      2     3
//
//          [     ]  [    ] [ ]
//
//
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::hash::DefaultHasher;
#[derive(Serialize, Deserialize, Debug)]

struct Workspace {
    name: String,
    done: VecDeque<String>,
    todo: VecDeque<String>,
    current: VecDeque<String>,
    colour: u64,
}

fn init_workspace 

fn read_save(workspace_name: String) {
    let path = format!("~/.config/backpack/workspaces/{workspace_name}");
    println!("reading from: {path}");
    let content = fs::read_to_string(path).unwrap_or(String("ok"));
    

}

fn main() {
    println!("Hello, world!");
}
