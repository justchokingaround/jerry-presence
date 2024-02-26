pub struct Stream {
    pub url: String,
    pub subtitle: Option<String>,
    pub title: Option<String>,
    pub player_args: Option<String>,
}

impl Stream {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        url: String,
        subtitle: Option<String>,
        title: Option<String>,
        player_args: Option<String>,
    ) -> Self {
        Self {
            url,
            subtitle,
            title,
            player_args,
        }
    }

}