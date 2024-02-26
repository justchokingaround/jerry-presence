use super::Player;

pub struct Mpv {
    pub executable: String,
    pub args: Vec<String>,
}

impl Mpv {
    pub fn new() -> Self {
        Self {
            executable: "mpv".to_string(),
            args: vec![],
        }
    }
}

impl Player for Mpv {
    fn play(
        &self,
        media: crate::stream::Stream,
    ) -> Result<std::process::Child, Box<dyn std::error::Error>> {
        let mut args = self.args.clone();

        args.push(media.url);
        if let Some(subtitle) = media.subtitle {
            args.push(format!("--sub-file={subtitle}"));
        }

        if let Some(title) = media.title {
            args.push(format!("--title={title}"));
            args.push(format!("--force-media-title={title}"));
        }

        if let Some(player_args) = media.player_args {
            for arg in player_args.split(' ') {
                args.push(arg.to_string());
            }
        }

        std::process::Command::new(&self.executable)
            .args(args)
            .spawn()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}
