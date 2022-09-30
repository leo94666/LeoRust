use argparse;
use argparse::arg::Arg;
use argparse::command::Command;

fn main() {
    println!("======================================");
    println!();


    let command = Command::new("Leo Rust Demo")
        .author("Leo Development Team")
        .version("3.2.1")
        .about("一款用于命令行解析的高性能的Rust框架！")
        .add(Arg::new("域名").short_name('h'))
        .get_matches();

    println!();
    println!("======================================");
}
