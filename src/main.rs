mod frontend;

use frontend::cli::run_cli;
use frontend::tui::run_tui;

fn main() {
    let raw_args = std::env::args().skip(1).collect::<Vec<_>>();

    if raw_args.is_empty() {
        run_tui();
        return;
    }

    run_cli();
}
