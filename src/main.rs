pub mod players;
pub mod stream;

use clap::Parser;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use players::{mpv::Mpv, Player};
use stream::Stream;
use regex::Regex;
use structopt::lazy_static::lazy_static;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    id: Option<String>,
    #[arg(long)]
    title: Option<String>,
    #[arg(long)]
    episode_number: Option<u32>,
    #[arg(long)]
    video_url: String,
    #[arg(long)]
    subtitle_url: Option<String>,
    #[arg(long)]
    player_args: Option<String>,
}
const SMALL_IMAGE: &str = "https://images-ext-1.discordapp.net/external/dUSRf56flwFeOMFjafsUhIMMS_1Xs-ptjeDHo6TWn6c/%3Fquality%3Dlossless%26size%3D48/https/cdn.discordapp.com/emojis/1138835294506975262.png";
const PATTERN: &str = r#"(\(Paused\)\s)?AV:\s([0-9:]*) / ([0-9:]*) \(([0-9]*)%\)"#;
const KITSU_API_ENDPOINT: &str = "https://kitsu.io/api/";

lazy_static! {
    static ref FILE_PATH: String = if cfg!(windows) {
        std::env::var("LocalAppData").unwrap() + "\\Temp\\jerry_position"
    } else {
        String::from("/tmp/jerry_position")
    };
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let id = match cli.id {
        Some(id) => id,
        None => "1003450375732482138".to_string(),
    };

    let mut client = DiscordIpcClient::new(&id)?;

    let anime_title = cli.title.clone().unwrap_or("No Title".to_string());

    client.connect()?;
    let details = match (&cli.title, &cli.episode_number) {
        (Some(title), Some(episode_number)) => format!("{} - Episode {}", title, episode_number),
        (Some(title), None) => title.clone(),
        (None, _) => String::from("No Title"),
    };

    let mpv = Mpv::new();
    let stream = Stream::new(cli.video_url, cli.subtitle_url, cli.title, cli.player_args);

    let mut child = mpv.play(stream).unwrap();

    let large_image = get_large_image(&anime_title).await?;

    let re: regex::Regex = Regex::new(PATTERN).unwrap();

    while child.try_wait()?.is_none() {
        let content = std::fs::read_to_string(&*FILE_PATH)?;
        let captures = re.captures_iter(content.as_str()).last().ok_or("Could not match the regex pattern.");

        let position = match captures {
            Ok(captures) => {
                let (_paused, av_first, av_second, _percentage) = (
                    captures.get(1).map_or("", |m| m.as_str()),
                    captures.get(2).map_or("", |m| m.as_str()),
                    captures.get(3).map_or("", |m| m.as_str()),
                    captures.get(4).map_or("", |m| m.as_str()),
                );
                format!("{}/{}", av_first, av_second)
            }
            Err(_) => String::from(""),
        };

        let episode_text = format!("Episode {}", cli.episode_number.unwrap_or(0));
        let activity = activity::Activity::new()
            .details(details.as_str())
            .state(position.as_str())
            .assets(
                activity::Assets::new()
                    .large_image(&large_image)
                    .large_text(&anime_title)
                    .small_image(SMALL_IMAGE)
                    .small_text(episode_text.as_str()),
            )
            .buttons(vec![
                activity::Button::new("Github", "https://github.com/justchokingaround/jerry"),
                activity::Button::new("Discord", "https://discord.gg/4P2DaJFxbm"),
            ]);

        client.set_activity(activity)?;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    client.close()?;

    Ok(())
}


async fn get_large_image(title: &String) -> Result<String, Box<dyn std::error::Error>> {
    let title = title.replace(" ", "-").to_lowercase();
    let url = format!("{}edge/anime?filter[text]={}", KITSU_API_ENDPOINT, title);
    let text_response = reqwest::get(url).await?.text().await?;
    let resp: serde_json::Value = serde_json::from_str(&text_response)?;
    Ok(resp["data"][0]["attributes"]["posterImage"]["original"].as_str().ok_or("No image found.")?.to_string())
}
