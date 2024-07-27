# Parson

Parson is a simple CLI library made for myself. Inspired a bit by clap and a bit by Commander.js.

If you decide to use it that's cool but it comes with no guarantees.

## Usage

```rust
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
```

### Result
```sh
> namesay John --last-name=Doe --shout
HELLO, JOHN DOE!
```
