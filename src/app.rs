use crate::calc::Calc;
use clap::{App, AppSettings, Arg, SubCommand};

error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }

    errors {
        FooError

        InvalidFoo(t: String) {
            description("invalid foo")
            display("invalid foo name: '{}'", t)
        }

        InvalidDir(dir: std::path::PathBuf, msg: String) {
            description("invalid dir")
            display("Invalid dir: '{:?}' ({})", dir, msg)
        }

        #[doc = "A custom error kind."]
        Custom(msg: String) {
            display("{}", msg)
        }
    }
}

pub trait Command {
    fn run(&mut self, args: &clap::ArgMatches) -> Result<()>;
}

pub fn run() -> Result<()> {
    let matches = App::new("NumbeRS") // crate_name!()
        .version(crate_version!())
        .author("Jens Tröger <jens@team-rhino.de>") // .author(crate_authors!("\n"))
        .about(crate_description!())
        .before_help("Some info I'd like to appear before the help info")
        .after_help(
            "Longer explanation to appear after the options when \
            displaying the help information from --help or -h",
        )
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequired)
        // prime factorization (PF) - Primfaktorzerlegung (pfz)
        .subcommand(
            SubCommand::with_name("pfz")
                .about("Primfaktorzerlegung")
                .alias("pf")
                .arg(Arg::with_name("values").required(true).min_values(1)),
        )
        // greatest common divisor (GCD) - größter gemeinsamer Teiler (ggT)
        .subcommand(
            SubCommand::with_name("ggt")
                .about("größter gemeinsamer Teiler")
                .alias("gcd")
                .arg(Arg::with_name("values").required(true).min_values(1)),
        )
        // least common multiple (LCM) - kleinstes gemeinsames Vielfaches (kgV)
        .subcommand(
            SubCommand::with_name("kgv")
                .about("kleinstes gemeinsames Vielfaches")
                .alias("lcm")
                .arg(Arg::with_name("values").required(true).min_values(1)),
        )
        .get_matches();

    // println!("matches: {:?}", matches);

    let command_result: Result<()> = match matches.subcommand() {
        ("pfz", Some(sub_m)) => Calc::from_args(sub_m).command_pfz(),
        ("ggt", Some(sub_m)) => Calc::from_args(sub_m).command_ggt(),
        ("kgv", Some(sub_m)) => Calc::from_args(sub_m).command_kgv(),
        _ => {
            println!("{}", matches.usage());
            println!(
                "\n\
                See 'numbers help' or 'numbers help <COMMAND>' \
                to read about a specific subcommand."
            );
            Ok(())
        }
    };

    command_result
}
