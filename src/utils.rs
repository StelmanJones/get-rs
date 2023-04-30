use comfy_table::{Row, Table};
use owo_colors::OwoColorize;
use trauma::{
    download::{Status, Summary},
    downloader::{ProgressBarOpts, StyleOptions},
};
pub const BAR_TEMPLATE: & 'static str = "{bar:70.green/black} {bytes:>11.green}/{total_bytes:<11.green} {bytes_per_sec:>13.red} eta {eta:.blue}";

pub fn get_styles() -> StyleOptions {
    StyleOptions::new(
        ProgressBarOpts::new(
            Some("{pos:>}/{len} ({percent}%) eta {eta_precise:.blue}".to_owned()),
            Some("  ".to_owned()),
            true,
            true,
        ),
        ProgressBarOpts::new(
            Some(BAR_TEMPLATE.to_owned()),
            Some(ProgressBarOpts::CHARS_LINE.to_owned()),
            true,
            false,
        ),
    )
}

pub fn display_summary(summaries: &[Summary]) {
    let header = Row::from(vec!["File", "Status"]);
    let mut table = Table::new();
    table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);
    table
        .load_preset(comfy_table::presets::NOTHING)
        .set_header(header);
    summaries.iter().for_each(|s| {
        let mut error = String::new();
        let status = match s.status() {
            Status::Success => String::from("OK").green().to_string(),
            Status::Fail(s) => {
                error = s.to_string();
                error.truncate(50);
                if error.len() <= 50 {
                    error.push_str("...");
                }
                String::from("ERROR").red().to_string()
            }
            Status::NotStarted => String::from("🔜"),
            Status::Skipped(s) => {
                error = s.to_string();
                String::from("⏭️")
            }
        };
        table.add_row(vec![&s.download().filename, &status]);
    });
    println!("{table}");
}
