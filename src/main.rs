mod async_tasks;
mod concurrency;
mod graph;
mod text_stats;

use std::error::Error;
use std::io;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let command = match args.next() {
        Some(value) => value
            .into_string()
            .map_err(|_| invalid_input("command must be valid UTF-8"))?,
        None => "help".to_owned(),
    };

    match command.as_str() {
        "help" | "--help" | "-h" => {
            ensure_no_extra(&mut args)?;
            print_help();
        }
        "stats" => {
            let path = args
                .next()
                .map(PathBuf::from)
                .ok_or_else(|| invalid_input("usage: rust-learning-lab stats <file>"))?;
            ensure_no_extra(&mut args)?;
            let stats = text_stats::analyze_file(&path)?;
            println!("bytes: {}", stats.bytes);
            println!("lines: {}", stats.lines);
            println!("words: {}", stats.words);
            println!("most frequent:");
            for (word, count) in stats.most_frequent(5) {
                println!("  {word}: {count}");
            }
        }
        "graph" => {
            ensure_no_extra(&mut args)?;
            let graph = graph::Graph::demo();
            println!("BFS from A: {}", graph.bfs("A").join(" -> "));
            println!("DFS from A: {}", graph.dfs("A").join(" -> "));
        }
        "parallel" => {
            let limit = parse_arg(args.next(), "limit", 1_000_000_u64)?;
            let workers = parse_arg(args.next(), "workers", 4_usize)?;
            ensure_no_extra(&mut args)?;
            let sum = concurrency::sum_parallel(limit, workers)?;
            println!("sum [0, {limit}) with {workers} workers: {sum}");
        }
        "async-demo" => {
            ensure_no_extra(&mut args)?;
            async_tasks::run().await;
        }
        other => return Err(invalid_input(&format!("unknown command: {other}")).into()),
    }

    Ok(())
}

fn parse_arg<T>(value: Option<std::ffi::OsString>, name: &str, default: T) -> Result<T, io::Error>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
{
    match value {
        Some(value) => value
            .to_str()
            .ok_or_else(|| invalid_input(&format!("{name} must be valid UTF-8")))?
            .parse()
            .map_err(|error| invalid_input(&format!("invalid {name}: {error}"))),
        None => Ok(default),
    }
}

fn invalid_input(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.to_owned())
}

fn ensure_no_extra(args: &mut impl Iterator<Item = std::ffi::OsString>) -> Result<(), io::Error> {
    if args.next().is_some() {
        return Err(invalid_input("too many arguments"));
    }
    Ok(())
}

fn print_help() {
    println!(
        "Rust learning lab\n\
         Commands:\n\
         \tstats <file>              Analyze a UTF-8 text file\n\
         \tgraph                     Run BFS and DFS examples\n\
         \tparallel [limit] [workers] Sum 0..limit using threads\n\
         \tasync-demo                Run two async tasks concurrently"
    );
}
