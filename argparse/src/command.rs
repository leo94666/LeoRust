use std::fmt::{Display, Formatter, write};
use std::string::String;
use crate::arg::Arg;
use std::env;

#[derive(Debug, Clone, Default)]
pub struct Command {
    name: String,
    author: String,
    version: String,
    about: String,
}



impl Command {
    pub fn new(name: &str) -> Self {
        fn new_inner(name: &str) -> Command {
            let name = name.to_string();
            Command {
                name,
                ..Default::default()
            }
        }
        new_inner(name)
    }

    pub fn author(mut self, author: &str) -> Self {
        self.author = author.to_string();
        self
    }

    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }


    pub fn about(mut self, about: &str) -> Self {
        self.about = about.to_string();
        self
    }

    pub fn add(mut self, arg: Arg) -> Self {
        self
    }


    
    pub fn get_matches(self) -> Self {
        //self.get_matches_from(&mut env::args_os())
        println!("{} {}", self.name, self.version);
        println!("{}", self.author);
        println!("{}", self.about);

        println!();
        println!("usage:");
        println!("  studio.exe <sub_command>");
        println!();
        println!("options:");
        println!("  -h, --help      {}", "Print Help Information");
        println!("  -v, --version   {}", "Print Version Information");
        println!();

        let args: Vec<String> = env::args().collect();
        println!("{:?}", args);

        self
    }
}