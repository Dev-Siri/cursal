use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct CursalArgs {
    pub url: String,
    /// Only GET, POST, PATCH, PUT, DELETE are supported.
    #[arg(short, long)]
    pub method: Option<String>,
    /// Write the stdout to a file.
    #[arg(short, long)]
    pub output: Option<String>,
    /// Data to send in the body of the request. For pointing to a file, start with a `@`.
    #[arg(short, long)]
    pub data: Option<String>,
}
