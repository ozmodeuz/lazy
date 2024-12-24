use clap::{Command, Arg};

fn init() {}

fn app() -> Command {
    Command::new("sysenv").about("A tool for managing system environment configurations.")
        .subcommand(Command::new("init")
            .about("")
            .aliases(["i", "initialize", "setup", "s"]))
        .subcommand(Command::new("add")
            .about("")
            .aliases(["a", "install"]))
        .subcommand(Command::new("remove")
            .about("")
            .aliases(["rm", "delete", "d", "uninstall", "un"]))
        .subcommand(Command::new("search")
            .about("")
            .aliases(["s", "find", "f", "query", "q"]))
        .subcommand(Command::new("update")
            .about("")
            .aliases(["u", "upgrade"])) 
        .subcommand(Command::new("clean")
            .about("")
            .aliases(["c", "gc"]))
        .subcommand(Command::new("show")
            .about("")
            .aliases(["s", "info"]))
        .subcommand(Command::new("input")
            .about(""))
        .subcommand(Command::new("help").about(""))
        .subcommand(Command::new("version").about(""))
}

fn main() {
    let app = app();
}