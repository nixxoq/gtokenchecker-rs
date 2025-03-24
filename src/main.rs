use api::Checker;
use clap::Parser;

pub mod api;
pub mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Token to check
    #[arg(short, long)]
    pub token: String,

    /// Mask last token part for security purposes
    #[arg(short, long, default_value_t = false)]
    pub mask_token: bool,
}

#[tokio::main]
async fn main() {
    // get_account_creation(935942230634532884, None);
    let args = Args::parse();

    let result = Checker::new(&args.token).check().await;

    match result {
        Ok(token_info) => {
            token_info.show(args.mask_token);
        }
        Err(err) => {
            eprintln!("Error: {}", err.message)
        }
    }
}
