pub mod read_settings;
pub mod write_settings;
pub mod config_dirs;

// Re-export commonly used types/functions for convenience
pub use read_settings::load_settings;
pub use write_settings::save_settings;
pub use write_settings::Settings;
pub use config_dirs::{project_config_dir, user_cache_dir, ensure_dirs_exist};
