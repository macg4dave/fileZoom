use anyhow::Result;
use crossterm::event::{Event, EventStream};
use futures_util::stream::StreamExt;
use tracing::warn;

/// Asynchronously listens for terminal events and invokes `on_event` for each one.
///
/// This is a thin wrapper around `crossterm::event::EventStream` that forwards
/// events to the provided synchronous handler closure. The handler runs on the
/// task that awaits events; it should be quick and non-blocking to avoid
/// delaying event processing.
///
/// Note: the function currently treats errors from the underlying event
/// stream as best-effort: errors are logged and the listener continues. If you
/// need stronger error semantics (propagating an error to callers or stopping
/// on first failure) open an issue or wrap this helper with custom logic.
///
/// Example (requires an async runtime and `--features async-input`):
///
/// ```ignore
/// fileZoom::input::async_input::event_listener(|ev| {
///     // handle event synchronously inside closure
/// });
/// ```
pub async fn event_listener<F>(mut on_event: F) -> Result<()>
where
    F: FnMut(Event) + Send + 'static,
{
    let mut stream = EventStream::new();

    // Use an idiomatic async stream loop. Log errors but keep listening so
    // transient errors do not terminate the UI unexpectedly.
    while let Some(result) = stream.next().await {
        match result {
            Ok(event) => on_event(event),
            Err(e) => warn!("async input event stream error (continuing): {}", e),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    //! Lightweight tests for the `event_listener` helper.
    //!
    //! These tests are primarily smoke-tests that ensure the function compiles
    //! and the closure is callable. Integration with a real terminal event
    //! stream is environment-specific and therefore not exercised here.

    use super::event_listener;

    // A basic compilation / invocation test: ensure the function is callable
    // and returns `Ok(())` when the runtime drives it for a short time.
    // This test is run with `tokio` if available; otherwise it is ignored.
    #[cfg_attr(not(feature = "tokio"), ignore)]
    #[tokio::test]
    async fn smoke_event_listener_invocable() {
        // Handler that does nothing with the event.
        let handler = |_ev: crossterm::event::Event| {};

        // Spawn the listener for a short time and then cancel it.
        // We don't expect any particular events; this verifies the API.
        let fut = event_listener(handler);

        // Drive the future for a very short time; if it panics this test
        // will catch it. We do not await indefinitely.
        let _ = tokio::time::timeout(std::time::Duration::from_millis(50), fut).await;
    }
}
