use std::fmt::{Display, Formatter, write};
use std::string::String;

//Arg结构体
#[derive(Default, Clone)]
pub struct Arg {
    // id
    pub(crate) id: String,
    // 简名
    pub(crate) short_name: char,
    // 全名
    pub(crate) long_name: String,
    // 是否必须
    pub(crate) need: bool,

    pub(crate) desc:String,
    pub(crate) default: String,
}

// Basic API
impl Arg {
    pub fn new(id: &str) -> Self {
        Arg::default().id(id)
    }

    #[must_use]
    pub fn id(mut self, id: &str) -> Self {
        self.id = String::from(id);
        self
    }

    #[must_use]
    pub fn short_name(mut self, s: char) -> Self {
        debug_assert!(s!='-',"short option name cannot be `-`");
        self.short_name = s;
        self
    }

}

impl Display for Arg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use crate::arg::Arg;

    #[test]
    fn it_works() {
        let arg = Arg::new("2222222")
            .short_name('c');

        println!("{}", arg)
    }
}
