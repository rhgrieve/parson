use helpers::{error, parse_opt, ErrorType};
use std::{collections::HashMap, process};
use tabular::Tabular;

pub mod helpers;

#[derive(Debug)]
pub struct Opt {
    name: String,
    short: char,
    long: String,
    value: String,
    expects_value: bool,
}

impl Opt {
    pub fn new(name: &str) -> Opt {
        let short = name.chars().next().unwrap();
        let mut long = String::new();
        if name.len() > 1 {
            long = name.to_string();
        }

        Opt {
            name: name.to_string(),
            short,
            long,
            value: String::new(),
            expects_value: false,
        }
    }

    pub fn short(mut self, value: char) -> Self {
        self.short = value;
        self
    }

    pub fn long(mut self, value: String) -> Self {
        self.long = value;
        self
    }

    pub fn expects_value(mut self) -> Self {
        self.expects_value = true;
        self
    }
}

#[derive(Default, Debug)]
pub struct Argument {
    name: String,
    pub value: String,
}

impl Argument {
    pub fn new(name: &str) -> Argument {
        Argument {
            name: name.to_string(),
            value: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct Command {
    name: String,
    version: String,
    description: String,
    example: String,
    action: fn(Argument, HashMap<String, String>),
    argument: Argument,
    subcommands: HashMap<String, Command>,
    options: Vec<Opt>,
    matches: HashMap<String, String>,
}

impl Command {
    pub fn new(name: &str) -> Command {
        let mut subcommands: HashMap<String, Command> = HashMap::new();
        if !name.eq("help") {
            subcommands.insert(String::from("help"), Command::new("help"));
        }

        Command {
            name: name.to_string(),
            version: String::new(),
            description: String::new(),
            example: String::new(),
            action: |arg, matches| println!("{:?} - {:?}", arg, matches),
            argument: Argument::default(),
            subcommands,
            options: vec![Opt::new("help"), Opt::new("version")],
            matches: HashMap::new(),
        }
    }

    pub fn name(mut self, name: &str) -> Command {
        self.name = name.to_string();
        self
    }

    pub fn version(mut self, version: &str) -> Command {
        self.version = version.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Command {
        self.description = description.to_string();
        self
    }

    pub fn example(mut self, example: &str) -> Command {
        self.example = example.to_string();
        self
    }

    pub fn argument(mut self, argument: Argument) -> Command {
        self.argument = argument;
        self
    }

    pub fn action(mut self, action: fn(Argument, HashMap<String, String>)) -> Command {
        self.action = action;
        self
    }

    pub fn subcommand(mut self, subcommand: Command) -> Command {
        self.subcommands.insert(subcommand.name.clone(), subcommand);
        self
    }

    pub fn option(mut self, option: Opt) -> Command {
        self.options.push(option);
        self
    }

    fn display_help(&self) {
        if !self.name.is_empty() {
            let mut name_line = self.name.clone();
            if !self.version.is_empty() {
                name_line = format!("{name_line} {}", &self.version);
            }
            println!("{name_line}");
        }

        if !self.description.is_empty() {
            println!("\nDESCRIPTION\n  {}", &self.description);
        }

        println!("\nUSAGE");
        let mut usage_string = format!("  $ {}", env!("CARGO_PKG_NAME"));
        if !self.subcommands.is_empty() {
            usage_string = format!("{usage_string} COMMAND");
        }
        if !self.options.is_empty() {
            usage_string = format!("{usage_string} [OPTIONS]")
        }
        println!("{usage_string}");

        if !self.example.is_empty() {
            println!("\nEXAMPLE\n  $ {}", &self.example);
        }

        println!(
            "\nCOMMANDS\n  Use \"{} [COMMAND] --help\" for more information about a command.",
            env!("CARGO_PKG_NAME")
        );

        println!("\nAvailable Commands:");
        let mut sorted_subcommands = self.subcommands.values().collect::<Vec<&Command>>();
        sorted_subcommands.sort_by(|a, b| a.name.cmp(&b.name));

        let tabular_commands = sorted_subcommands
            .into_iter()
            .map(|sc| vec![&sc.name, &sc.description])
            .collect::<Vec<Vec<&String>>>();

        println!("{}", tabular_commands.to_table())
        // .for_each(|sc| println!("  {} {}", sc.name, sc.description))
    }

    fn display_version(&self) {
        println!("{} {}", env!("CARGO_PKG_NAME"), self.version);
    }

    fn handle_default_flags(&self, opt_name: String) {
        if opt_name.eq("help") {
            self.display_help();
            process::exit(0);
        } else if opt_name.eq("version") {
            self.display_version();
            process::exit(0);
        }
    }

    fn parse(mut self) -> Command {
        let mut expecting_opt: Option<&Opt> = None;
        for arg in std::env::args().collect::<Vec<String>>().drain(1..) {
            // Move on to subcommand if we hit a matching command
            if let Some(subcommand) = self.subcommands.remove(&arg) {
                return subcommand.parse();
            }

            // If we were expecting a value from the last option,
            // check if this one is an option, and if not, store
            // it as the value (e.g --name John)
            if let Some(expecting) = expecting_opt {
                if arg.starts_with('-') {
                    error(ErrorType::ExpectingValue(&expecting.name))
                }

                self.matches.insert(expecting.name.clone(), arg);
                expecting_opt = None;
                continue;
            }

            // Handle positional argument, currently 1 per command
            if self.argument.value.is_empty() && !arg.starts_with('-') {
                self.argument.value = arg;
                continue;
            }

            // Find option by either short name or long name depending on
            // option type
            let (opt_name, opt_value) = parse_opt(&arg);
            let mut predicate: Box<dyn Fn(&&Opt) -> bool> =
                Box::new(|opt: &&Opt| opt.long == opt_name);
            if arg.starts_with('-') {
                predicate = Box::new(|opt: &&Opt| opt.short == opt_name.chars().next().unwrap());
            }

            // Handle standalone long and short options
            if arg.starts_with("--") || (arg.starts_with('-') && !opt_value.is_empty()) {
                if let Some(opt) = self.options.iter().find(predicate) {
                    self.handle_default_flags(opt.name.clone());
                    if opt.expects_value && opt_value.is_empty() {
                        expecting_opt = Some(opt);
                    } else if !opt.expects_value && !opt_value.is_empty() {
                        error(ErrorType::UnexpectedValue(&opt.name))
                    }
                    self.matches.insert(opt.name.clone(), opt_value);
                } else {
                    error(ErrorType::InvalidOption(&opt_name));
                }
            } else {
                // Handle chained short options
                for char in opt_name.chars() {
                    if let Some(opt) = self.options.iter().find(|opt| opt.short == char) {
                        self.handle_default_flags(opt.name.clone());
                        if opt.expects_value {
                            error(ErrorType::ExpectingValue(&opt.name))
                        }
                        self.matches.insert(opt.name.clone(), String::new());
                    } else {
                        error(ErrorType::InvalidOption(&opt_name));
                    }
                }
            }
        }
        self
    }

    pub fn run(self) {
        let command = self.parse();
        (command.action)(command.argument, command.matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
