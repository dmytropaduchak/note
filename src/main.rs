use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about = "A CLI app to create secure notion notes", long_about = None, disable_version_flag = true)]
struct Args {
    #[arg(short, long, action = clap::ArgAction::SetTrue, help = "Print version")]
    version: bool,
    /// Name of the person to greet
    #[arg(short, long, default_value_t = 10)]
    list: u32,
    // conf:
    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}

// #[derive(Parser, Debug)]
// #[command(name)]
// #[command(about, version)]
// struct Args {
//     #[command(subcommand)]
//     command: Commands,
// }

fn main() {
    let args = Args::parse();

    if args.version {
        println!(env!("CARGO_PKG_VERSION"));
        return;
    }

    // ma
    println!("Hello {}!", args.list);
    // for _ in 0..args.count {

    // }
}
