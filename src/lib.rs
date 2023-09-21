use clap::builder::{Arg, Command};
use clap::{
    crate_authors, crate_description, crate_name, crate_version, value_parser, ArgAction,
    ArgMatches,
};
use std::error::Error;

type HeadrResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> HeadrResult<Config> {
    let matches: ArgMatches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .help_template(
            "\
{before-help}{name} {version}
    {author-with-newline}{about-with-newline}
{usage-heading} {usage}
{all-args} {after-help}
",
        )
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_values(["-"]),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .help(
                    "Print the first K lines instead of the first 10 with the leading '-', print all but the last K lines of each file")
                .num_args(1)
                .default_value("10")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help(
                    "print the first K bytes of each file; with the leading '-', print all but the last K lines of each file")
                .num_args(1)
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set)
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("file")
            .expect("Must contains valid filepaths")
            .cloned()
            .collect(),
        lines: *matches.get_one::<usize>("lines").unwrap(),
        bytes: match matches.get_one::<usize>("bytes") {
            Some(bytes) => Some(*bytes),
            None => None,
        },
    })
}

pub fn run(config: Config) -> HeadrResult<()> {
    println!("{:#?}", config);

    Ok(())
}
