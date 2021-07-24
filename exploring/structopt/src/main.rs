use structopt::StructOpt;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

#[derive(Debug, StructOpt)]
#[structopt(name = "structopt example", about = "using structop")]
struct Args {
    /// target hosts
    #[structopt(short = "n", long)]
    hosts: Vec<String>,
    /// target datacenters
    #[structopt(short = "s", long)]
    sites: Vec<String>,
    /// log output
    #[structopt(short, long)]
    log: Option<String>,
    /// CLI command
    #[structopt(short = "c", long)]
    command: Option<String>,
}
fn main() {
    println!("Structopt example");
    let args = Args::from_args();
    println!("{:?}", args);

    match args.command {
        Some(cmd) => println!("Command: {}", cmd),
        None => println!("No command."),
    }
}
