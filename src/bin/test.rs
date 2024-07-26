use parson::{Argument, Command, Opt};

fn main() {
    let subcommand = Command::new("build")
        .description("Build whatever you want, whenever you want")
        .action(|_, _| println!("Do a build!"));

    Command::new("Test Program")
        .version("1.0.0")
        .description("A sample app for you and me")
        .argument(Argument::new("path"))
        .option(Opt::new("caps"))
        .option(Opt::new("name").expects_value())
        .example("parson john --caps")
        .action(|arg, options| {
            let caps = options.contains_key("caps");
            let value = if caps {
                arg.value.to_uppercase()
            } else {
                arg.value
            };

            let mut value_to_print = value;
            if let Some(name) = options.get("name") {
                value_to_print = format!("{value_to_print} - {name}")
            }

            println!("{value_to_print}")
        })
        .subcommand(subcommand)
        .run();
}
