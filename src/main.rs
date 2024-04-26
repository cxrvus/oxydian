use clap::{arg, command, ArgAction};

fn main() {
    let matches = command!()
        .version("1.0")
        .author("Benito Rabe")
        .about("Rust Interface for Obsidian")
        .arg(
            arg!("command")
                .action(ArgAction::Set)
                .required(true)
                .value_parser(clap::builder::PossibleValuesParser::new(["query"]))
        )
        .arg(
            arg!("id")
                .action(ArgAction::Set)
                .required(true)
        )
        .arg(
            arg!("note")
                .action(ArgAction::Set)
                .required(false)
        )
        .get_matches();
}
