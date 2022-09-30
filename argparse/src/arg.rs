use std::string::String;

pub struct Arg {
    pub(crate) id: String,
    pub(crate) short: Option<char>,
    pub(crate) long: Option<Str>,
    pub(crate) help: Option<char>,

}