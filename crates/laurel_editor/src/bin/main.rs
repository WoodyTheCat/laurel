use laurel_editor::Editor;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

pub fn main() -> iced::Result {
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            "warn,iced=info,laurel_editor=trace,async_lsp=info",
        ))
        .with(
            #[cfg(debug_assertions)]
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true),
            #[cfg(not(debug_assertions))]
            tracing_subscriber::fmt::layer()
                .compact()
                .with_thread_ids(true),
        )
        .init();

    iced::application("Laurel", Editor::update, Editor::view)
        .theme(Editor::theme)
        // .scale_factor(Editor::scale_factor)
        .subscription(Editor::subscription)
        // .settings(settings(&config_load))
        .run_with(|| Editor::new(()))
        .inspect_err(|err| tracing::error!(error = ?err, "Iced error"))?;

    Ok(())
}
