use clap::Parser;

/// Small CLI wrapper for fileZoom. Keep it minimal: allow starting
/// directory override and an option to disable mouse capture at startup.
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Start the program in this directory instead of the current directory
    #[arg(short, long, value_name = "DIR")]
    dir: Option<std::path::PathBuf>,

    /// Disable mouse capture on startup (can be toggled in settings later)
    #[arg(long)]
    no_mouse: bool,

    /// Start with this theme (e.g. `default` or `dark`). When omitted the
    /// persisted setting (or default) is used. Allowed values: `default`, `dark`.
    #[arg(long, value_name = "NAME", value_parser = ["default", "dark"])]
    theme: Option<String>,

    /// Show hidden files at startup (overrides persisted setting).
    #[arg(long = "show-hidden")]
    show_hidden: bool,

    /// Increase verbosity (-v, -vv, -vvv). This sets the logging level;
    /// more `v` increases verbosity (0 = default, 1 = info, 2 = debug, 3+ = trace).
    #[arg(short, long = "verbose", action = clap::ArgAction::Count)]
    verbosity: u8,
}

fn main() -> anyhow::Result<()> {
    // Parse CLI args early so we can affect process state (cwd, etc.).
    let cli = Cli::parse();

    // Configure logging early. If the user provided `-v` flags we set a
    // reasonable RUST_LOG default (unless the environment already set it),
    // then initialize the logger.
    if std::env::var_os("RUST_LOG").is_none() {
        let lvl = match cli.verbosity {
            0 => "info",
            1 => "info",
            2 => "debug",
            _ => "trace",
        };
        std::env::set_var("RUST_LOG", lvl);
    }
    env_logger::init();

    // Create a shutdown channel and register a Ctrl-C handler that sends
    // a shutdown notification. The main runner will own the `TerminalGuard`
    // and will restore the terminal when the shutdown signal is received.
    let (shutdown_tx, shutdown_rx) = std::sync::mpsc::channel::<()>();
    let tx_clone = shutdown_tx.clone();
    ctrlc::set_handler(move || {
        let _ = tx_clone.send(());
    })?;

    // If async input support is enabled, spawn a small thread that runs
    // an EventStream and forwards events into a channel. Install the
    // receiver so `input::read_event()` will check it before falling back
    // to the synchronous `crossterm::event::read()` path.
    #[cfg(feature = "async-input")]
    {
        use std::sync::mpsc::channel as mpsc_channel;
        use std::thread;

        let (async_tx, async_rx) = mpsc_channel::<crossterm::event::Event>();
        // install the receiver so `read_event()` can poll it
        fileZoom::input::install_async_event_receiver(async_rx);

        // Spawn a thread to run the async EventStream producer. We use a
        // simple executor via `futures::executor::block_on` here so we do
        // not add a full async runtime dependency; this thread will live
        // for the lifetime of the process when the feature is enabled.
        thread::spawn(move || {
            let fut = async move {
                if let Err(e) = fileZoom::input::async_input::event_listener(|ev| {
                    let _ = async_tx.send(ev);
                })
                .await
                {
                    tracing::error!("async event listener failed: {:#}", e);
                }
            };
            // Block on the future for this thread.
            futures::executor::block_on(fut);
        });
    }

    // Initialize the terminal and hand ownership to the runner so the
    // runner (in main thread) can restore the terminal cleanly on shutdown.
    let terminal = fileZoom::runner::terminal::init_terminal()?;

    // Construct start options from CLI and hand them to the runner. The
    // runner will apply CLI-provided overrides after loading persisted
    // settings so CLI values take precedence.
    let start_opts = fileZoom::app::StartOptions {
        start_dir: cli.dir,
        mouse_enabled: if cli.no_mouse { Some(false) } else { None },
        theme: cli.theme,
        show_hidden: if cli.show_hidden { Some(true) } else { None },
        verbosity: if cli.verbosity > 0 { Some(cli.verbosity) } else { None },
    };

    fileZoom::runner::run_app(terminal, shutdown_rx, start_opts)
}
