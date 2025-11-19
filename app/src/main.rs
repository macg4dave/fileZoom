fn main() -> anyhow::Result<()> {
    // Initialize logging so that `warn!`/`info!` from modules are visible
    // when a logger is configured via `RUST_LOG` or an env logger.
    env_logger::init();
    fileZoom::runner::run_app()
}
