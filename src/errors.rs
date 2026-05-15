#[derive(Debug)]
pub enum Errors {
    GetPostCall(String),
    GetPostRead(String),
    GetPostEmptyResponse,
    TrayIcon(String),
}

// Display for Errors instead?

impl Errors {
    pub fn error_msg(&self) -> &str {
        match &self {
            Self::GetPostCall(e) => {
                eprintln!("{e}");
                "⚠ Unable to fetch posts"
            }
            Self::GetPostRead(e) => {
                eprintln!("{e}");
                "⚠ Unable to parse posts"
            }
            Self::GetPostEmptyResponse => "⚠ No posts found!",
            Self::TrayIcon(e) => {
                eprintln!("{e}");
                "⚠ Unable to init tray icon"
            }
        }
    }
}
