#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Env(std::env::VarError),
    SerdeJson(serde_json::Error),
    TwitterV2(twitter_v2::Error),
    Time(time::error::Error),
    OAuth2Conf(oauth2::ConfigurationError),
    TimeFormat(time::error::Format),
    //OAuth2Req(oauth2::RequestTokenError<oauth2::reqwest::Error<reqwest::error::Error>, StandardErrorResponse<BasicErrorResponseType>>),
    Error(Box<dyn std::error::Error + Send + Sync>),
    ErrorString(String),
}

impl Error {
    pub fn new(e: impl std::error::Error + Send + Sync + 'static) -> Self {
        Error::Error(Box::new(e))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO Error: {}", e),
            Error::Env(e) => write!(f, "Env Error: {}", e),
            Error::SerdeJson(e) => write!(f, "SerdeJson Error: {}", e),
            Error::TwitterV2(e) => write!(f, "TwitterV2 Error: {}", e),
            Error::Time(e) => write!(f, "Time Error: {}", e),
            Error::OAuth2Conf(e) => write!(f, "OAuth2 Configuration Error: {}", e),
            Error::TimeFormat(e) => write!(f, "Time Format Error: {}", e),
            Error::Error(e) => write!(f, "Error: {}", e),
            Error::ErrorString(e) => write!(f, "Error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::Env(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJson(e)
    }
}

impl From<twitter_v2::Error> for Error {
    fn from(e: twitter_v2::Error) -> Self {
        Error::TwitterV2(e)
    }
}

impl From<time::error::Error> for Error {
    fn from(e: time::error::Error) -> Self {
        Error::Time(e)
    }
}

impl From<oauth2::ConfigurationError> for Error {
    fn from(e: oauth2::ConfigurationError) -> Self {
        Error::OAuth2Conf(e)
    }
}

impl From<time::error::Format> for Error {
    fn from(e: time::error::Format) -> Self {
        Error::TimeFormat(e)
    }
}


