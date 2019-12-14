use clap::{App, AppSettings, Arg, ArgMatches};

fn is_six_digit(string: String) -> Result<(), String> {
    let num = string.parse::<i32>().map_err(|err| err.to_string())?;
    if 100000 <= num && num <= 999999 {
        Ok(())
    } else {
        Err(String::from("The number wasn't a 6-digit number"))
    }
}

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Advent of Code Day 4")
        .author("Eran Cohen")
        .about("Calculates the number of 6-digit passwords that exist in the given range")
        .arg(
            Arg::with_name("min")
                .help("The minimum 6-digit number in the range of possible passwords")
                .takes_value(true)
                .required(true)
                .validator(is_six_digit)
                .display_order(1),
        )
        .arg(
            Arg::with_name("max")
                .help("The maximum 6-digit number in the range of possible passwords")
                .takes_value(true)
                .required(true)
                .validator(is_six_digit),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
}

#[derive(Debug)]
struct Opt {
    min: i32,
    max: i32,
}

impl From<ArgMatches<'_>> for Opt {
    fn from(matches: ArgMatches) -> Self {
        Self {
            min: matches.value_of("min").unwrap().parse().unwrap(),
            max: matches.value_of("max").unwrap().parse().unwrap(),
        }
    }
}

fn main() {
    // get the options
    let app = app();
    let opt: Opt = app.get_matches().into();

    // max sure the options are valid
    if opt.min > opt.max {
        println!("Invalid value for <max>: The number was smaller the <min>");
        std::process::exit(1)
    }

    println!("{:?}", opt);
}
