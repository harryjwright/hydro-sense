use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_logger() {
    INIT.call_once(|| {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    });
}
