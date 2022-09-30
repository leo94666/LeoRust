pub mod arg;
pub mod command;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}



#[cfg(test)]
mod tests {
    use crate::command::Command;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn command(){
        let command = Command::new("Leo Rust Demo")
            .author("Leo Development Team")
            .version("3.2.1")
            .about("一款用于命令行解析的高性能的Rust框架！")
            .get_matches();


    }



}
