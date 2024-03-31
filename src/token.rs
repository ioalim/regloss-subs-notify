use std::{env, fs};

use oauth2::{
    reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
};
use serde_json::{json, Value};
use time::OffsetDateTime;
use twitter_v2::authorization::Oauth2Token;

use crate::{error::Error, log::push_to_log};

pub async fn get_token() -> Result<Value, Error> {
    let client = oauth2::basic::BasicClient::new(
        ClientId::new(env::var("RSN_CLIENT_ID")?),
        Some(ClientSecret::new(env::var("RSN_CLIENT_SECRET")?)),
        AuthUrl::new("https://twitter.com/i/oauth2/authorize".to_string()).map_err(Error::new)?,
        Some(
            TokenUrl::new("https://api.twitter.com/2/oauth2/token".to_string())
                .map_err(Error::new)?,
        ),
    )
    .set_redirect_uri(RedirectUrl::new("https://twitter.com".to_string()).map_err(Error::new)?);
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("tweet.read".to_string()))
        .add_scope(Scope::new("tweet.write".to_string()))
        .add_scope(Scope::new("users.read".to_string()))
        .add_scope(Scope::new("offline.access".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();
    println!("Browse to: {}", auth_url.to_string());
    let mut code = String::new();
    let _ = std::io::stdin()
        .read_line(&mut code)
        .expect("Failed to read line");
    let token_result = client
        .exchange_code(AuthorizationCode::new(code.trim().to_string()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .map_err(Error::new)?;
    push_to_log(format!(
        "RefreshToken: {}\n",
        token_result.refresh_token().ok_or_else(|| Error::ErrorString("For some reason the program can't get the refresh_token. Most likely because offline.access scope wasn't passed".to_string()))?.secret()
    ))?;
    let expires = (OffsetDateTime::now_utc()
        + token_result.expires_in().ok_or_else(|| {
            Error::ErrorString(
                "For some reason the program can't get the expires_in. I don't know oaut2 man"
                    .to_string(),
            )
        })?)
    .format(&time::format_description::well_known::Rfc3339)?;
    let access_json = json!({
        "access_token": token_result.access_token().secret(),
        "refresh_token": token_result.refresh_token(),
        "expires": expires,
        "scopes": ["tweet.write"]
    });
    Ok(access_json)
}

pub async fn get_token_without_refresh_flow(refresh_token: String) -> Result<Value, Error> {
    let client = oauth2::basic::BasicClient::new(
        ClientId::new(env::var("RSN_CLIENT_ID")?),
        Some(ClientSecret::new(env::var("RSN_CLIENT_SECRET")?)),
        AuthUrl::new("https://twitter.com/i/oauth2/authorize".to_string()).map_err(Error::new)?,
        Some(
            TokenUrl::new("https://api.twitter.com/2/oauth2/token".to_string())
                .map_err(Error::new)?,
        ),
    )
    .set_redirect_uri(RedirectUrl::new("https://twitter.com".to_string()).map_err(Error::new)?);
    let token_result = client
        .exchange_refresh_token(&RefreshToken::new(refresh_token))
        .request_async(async_http_client)
        .await
        .map_err(Error::new)?;
    push_to_log(format!(
        "RefreshToken: {}\n",
        token_result.refresh_token().ok_or_else(|| Error::ErrorString("For some reason the program can't get the refresh_token. Most likely because offline.access scope wasn't passed".to_string()))?.secret()
    ))?;
    let expires = (OffsetDateTime::now_utc()
        + token_result.expires_in().ok_or_else(|| {
            Error::ErrorString(
                "For some reason the program can't get the expires_in. I don't know oauth2 man"
                    .to_string(),
            )
        })?)
    .format(&time::format_description::well_known::Rfc3339)?;
    let access_json = json!({
        "access_token": token_result.access_token().secret(),
        "refresh_token": token_result.refresh_token(),
        "expires": expires,
        "scopes": ["tweet.write"]
    });
    Ok(access_json)
}

pub async fn get_token_from_file() -> Result<Oauth2Token, Error> {
    match std::fs::File::open(env::var("RSN_OAUTH2TOKEN_FILE")?) {
        Ok(file) => serde_json::from_reader(file).map_err(Error::SerdeJson),
        Err(_) => {
            push_to_log("Token file not found. Getting new token\n".to_string())?;
            let token = get_token().await?;
            fs::write(env::var("RSN_OAUTH2TOKEN_FILE")?, token.to_string())?;
            serde_json::from_value(token).map_err(Error::SerdeJson)
        }
    }
}

pub async fn refresh_if_expire(mut oauth2token: Oauth2Token) -> Result<Oauth2Token, Error> {
    if oauth2token.expires() < OffsetDateTime::now_utc() {
        push_to_log("Token expired. Refreshing token\n".to_string())?;
        let token =
            get_token_without_refresh_flow(oauth2token.refresh_token().unwrap().secret().clone())
                .await?;
        fs::write(env::var("RSN_OAUTH2TOKEN_FILE")?, token.to_string())?;
        oauth2token = serde_json::from_value(token)?;
        Ok(oauth2token)
    } else {
        push_to_log("Token not expired\n".to_string())?;
        Ok(oauth2token)
    }
}
