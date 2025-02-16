use parson::{Argument, Command, Opt};

fn main() {
    Command::new("namesay")
        .version("1.0.0")
        .description("Say someone's name")
        .argument(Argument::new("name"))
        .option(Opt::new("shout"))
        .option(Opt::new("last-name").expects_value())
        .example("namesay john --shout --last-name=Doe")
        .action(|arg, options| {
            let mut name = arg.value;
            if let Some(last_name) = options.get("last-name") {
                name = format!("{name} {last_name}");
            }

            let mut greeting = format!("Hello, {name}!");
            if options.contains_key("shout") {
                greeting = greeting.to_uppercase();
            }

            println!("{greeting}");
        })
        .run();
}
