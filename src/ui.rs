use indicatif::{ProgressBar, ProgressStyle};

use crate::error::ScillaResult;

pub async fn show_spinner<F, T>(message: &str, fut: F) -> ScillaResult<T>
where
    F: std::future::Future<Output = ScillaResult<T>>,
{
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));
    spinner.set_message(message.to_string());

    let result = fut.await;
    spinner.finish_with_message("✅ Done");

    result
}
