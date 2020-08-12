use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rebuilder", about, author)]
struct Args {
    /// List of input packages
    #[structopt(min_values = 1, required = true)]
    pkgnames: Vec<String>,

    /// The path to the pacman database, default ( /var/lib/pacman )
    #[structopt(long)]
    dbpath: Option<String>,

    /// Write a dotfile into the given file
    #[structopt(short, long)]
    dotfile: Option<String>,
}

fn main() {
    let args = Args::from_args();
    match rebuilder::run(args.pkgnames, args.dbpath, args.dotfile) {
        Ok(_) => std::process::exit(0),
        Err(_e) => std::process::exit(1),
    }
}
