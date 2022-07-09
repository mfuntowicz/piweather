use tracing::Level;

pub fn setup_logging(level: Level) {
    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();
}