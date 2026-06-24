mod frontend;
mod backend;

use frontend::cli::run_cli;
use frontend::gui::run_gui;

fn main() {
    let raw_args = std::env::args().skip(1).collect::<Vec<_>>();

    if raw_args.is_empty() {
        run_gui().unwrap();
        return;
    }

    run_cli();
}
