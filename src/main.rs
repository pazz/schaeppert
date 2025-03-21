use clap::Parser;
use nfa::Nfa;
use std::fs::File;
use std::io::{self, Read};
use std::process;
mod coef;
mod flow;
mod graph;
mod ideal;
mod nfa;
mod partitions;
mod semigroup;
mod sheep;
mod solver;
mod strategy;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: Option<String>,

    #[arg(short, long, default_value = "tikz")]
    input_type: String,

    #[arg(short, long, action)]
    test: bool,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    // Get the arguments

    let nfa: Option<Nfa>;
    match args.test {
        true => nfa = Some(nfa::Nfa::get_nfa("((a#b){a,b})#")),
        false => {
            match (args.filename) {
                Some(filename) => match read_file(&filename) {
                    Ok(content) => match args.input_type.as_str() {
                        "tikz" => {
                            nfa = Some(nfa::Nfa::from_tikz(&content));
                        }
                        _ => {
                            eprintln!("Invalid format: {}", args.input_type);
                            eprintln!("Known formats: [tikz]");
                            process::exit(1);
                        }
                    },
                    Err(e) => {
                        eprintln!("Error reading file '{}': '{}'", filename, e);
                        process::exit(1);
                    }
                },
                None => {
                    eprintln!("Provide filename");
                    process::exit(1);
                }
            }
            assert!(nfa.is_some());
        }
    }
    let solution = solver::solve(&nfa.unwrap());
    println!("{}", solution);
}

/// Reads the content of the file
fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
