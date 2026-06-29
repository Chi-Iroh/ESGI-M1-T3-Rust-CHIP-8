mod app;
mod cli;
mod engine;
mod interpreter;

use app::App;
use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    let mut app = match App::new_from_file(args.input) {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Error loading program from file: {}", e);
            return;
        }
    };

    app.draw_current_registers();
    match app.run() {
        Ok(_) => println!("Program executed successfully."),
        Err(e) => eprintln!("Error during execution: {}", e),
    }
    app.draw_current_registers();
}
