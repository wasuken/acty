mod cli;
mod list;
mod log_entry;
mod logger;

fn main() {
    let command = cli::parse_args();
    match command {
        cli::Command::Log { content, tags } => {
            logger::log_action(content, tags);
        }
        cli::Command::List { date, range, tags } => {
            list::list_logs(date, range, tags);
        }
    }
}
