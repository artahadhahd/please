use clap::Parser;

#[derive(Parser)]
#[command(author = "artahadhahd", about = "build system", long_about = None)]
pub struct Cli {
    #[arg(short, long, num_args(0..))]
    pub paths: Vec<String>,
}

impl Cli {
    pub fn get_result(&self) -> Vec<String> {
        if self.paths.is_empty() {
            vec![".".into()]
        } else {
            self.paths.to_owned()
        }
    }
}
