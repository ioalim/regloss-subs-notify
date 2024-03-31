use std::{
    env,
    fs::File,
    io::{Read, Write},
};

use rand::distributions::DistString;
use twitter_v2::TwitterApi;

use crate::{
    error::Error,
    log::push_to_log,
    regloss,
    token::{get_token_from_file, refresh_if_expire},
};

pub async fn post() -> Result<(), Error> {
    let oauth2token = get_token_from_file().await?;
    let oauth2token = refresh_if_expire(oauth2token).await?;
    let text_supposed_to = regloss::subscribers_as_str();
    let text_actual;
    if is_same_as_previous_tweet(&text_supposed_to)? {
        text_actual = "Same as before. ".to_string() + &rand_alphanumeric();
    } else {
        text_actual = text_supposed_to.clone();
    }
    let output = TwitterApi::new(oauth2token)
        .post_tweet()
        .text(text_actual.clone())
        .send()
        .await.map_err(|err| {
            push_to_log(format!("Tweet content supposed to be:\n {}\n", text_supposed_to)).unwrap();
            Error::TwitterV2(err)
        })?;
    if !is_same_as_previous_tweet(&text_supposed_to)? {
        save_as_previous_tweet(&text_actual)?;
    }
    push_to_log(format!("{}\n", time::OffsetDateTime::now_utc()))?;
    push_to_log(format!("{:#?}\n", output))
}

fn is_same_as_previous_tweet(content: &str) -> Result<bool, Error> {
    match File::open(env::var("RSN_PREVIOUS_TWEET_FILE")?) {
        Ok(mut file) => {
            let mut old_content = String::new();
            file.read_to_string(&mut old_content).map_err(Error::Io)?;
            Ok(old_content == content)
        }
        Err(_) => Ok(false),
    }
}

fn save_as_previous_tweet(content: &str) -> Result<(), Error> {
    let mut file = File::create(env::var("RSN_PREVIOUS_TWEET_FILE")?)?;
    file.write_all(content.as_bytes()).map_err(Error::Io)
}

fn rand_alphanumeric() -> String {
    rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
}

#[cfg(test)]
mod test {
    use std::{env, fs};
    use crate::{regloss, tweet::rand_alphanumeric};

    #[test]
    fn test_rand_alphanumeric() {
        let result = rand_alphanumeric();
        println!("{:#?}", result);
    }

    #[test]
    fn test_content() {
        let content = regloss::subscribers_as_str();
        fs::write(env::var("RSN_PREVIOUS_TWEET_FILE").unwrap(), content).unwrap();
    }
}
