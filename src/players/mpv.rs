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
    fn play(&self, media: crate::stream::Stream) -> Result<std::process::Child, Box<dyn std::error::Error>> {
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

#[cfg(test)]
mod test {
    use crate::{players::Player, stream::Stream};
    #[test]
    fn mpv_execution_for_stream() {
        /* Ignore in CI. */
        let stream = Stream::new(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
            None,
            Some("One Piece: Episode 1".to_string()),
            Some("--fs".to_string()),
        );

        let mpv = super::Mpv::new();

        let mut child = mpv.play(stream).unwrap();
        assert_eq!(
            child
                .wait()
                .expect("Failed to spawn child process for mpv.")
                .code(),
            Some(0)
        )
    }
}