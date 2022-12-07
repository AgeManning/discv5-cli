

/// Scope Args
#[derive(ClapParser, Debug, Clone)]
#[clap(name = "scope", version, about, long_about = None)]
struct Scope {


    #[clap(subcommand)]
    server
}
