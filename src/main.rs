mod utils;
use anstyle::Style;
use clap::{arg, builder::Styles, command, parser, value_parser, Arg, ArgAction};
use reqwest::Url;
use std::path::PathBuf;
use trauma::{
    download::Download,
    downloader::DownloaderBuilder,
    Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Clap CLI command
    let get = command!()
        .version("0.1.0")
        .author("Oscar N. <github.com/StelmanJones>")
        .about("No BS download tool. Does one thing and one thing only.")
        .arg(
            arg!(<URL> "Download URL.")
                .value_parser(value_parser!(PathBuf))
                .action(ArgAction::Append)
                .required(true),
        )
        .arg(
            arg!(-o --out [OUT] "Set output path.")
                .value_parser(value_parser!(PathBuf))
                .default_value("./"),
        )
        .arg(
            Arg::new("retries")
                .short('r')
                .long("retries")
                .value_parser(clap::value_parser!(u32))
                .default_value("3")
                .help("Sets retry attempts before failing."),
        )
        .arg(
            Arg::new("concurrent")
                .short('c')
                .long("concurrent")
                .value_parser(value_parser!(usize))
                .default_value("32")
                .help("Sets max number of concurrent downloads."),
        )
        .arg(Arg::new("summary") 
            .short('s')
            .long("summary")
            .help("Shows summary after all downloads has finished")
            .default_value("false")
            .action(ArgAction::SetTrue)
        )
        .color(clap::ColorChoice::Auto)
        .styles(
            Styles::styled()
                .header(
                    anstyle::Style::new()
                        .bold()
                        .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
                )
                .literal(
                    Style::new()
                        .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightBlue))),
                ),
        )
        .get_matches();

    // Parsing arguments

    let mut downloads: Vec<Download> = Vec::new();
    let urls = get.get_many::<PathBuf>("URL").unwrap_or_default();

    for url in urls {
        downloads.push(Download::new(
            &Url::try_from(url.to_str().expect("Parse error")).expect("Parse error."),
            url.file_name()
                .expect("Parse error")
                .to_str()
                .expect("String coercion error."),
        ));
        // Create downloading client
    }
    let downloader = DownloaderBuilder::new()
        .directory(get
            .get_one::<PathBuf>("out")
            .expect("Could not find an output path even though it has a default value. How is that even possible?").to_path_buf()
            )
        .retries(get
            .get_one::<u32>("retries")
            .expect("Error parsing retries value.").to_owned()
            )
        .concurrent_downloads(get
            .get_one::<usize>("concurrent")
            .expect("Error parsing concurrent downloads.").to_owned()

        )
            .style_options(utils::get_styles())
        
        .build();
    let summaries = downloader.download(&downloads).await;

    if get.get_one::<bool>("summary").expect("Error parsing summary flag.").to_owned() == true {
        utils::display_summary(&summaries)
    }  
    Ok(())
}
