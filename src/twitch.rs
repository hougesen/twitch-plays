use dotenv::dotenv;

pub fn get_twitch_credentials() -> Result<(String, String), dotenv::Error> {
    dotenv().ok();

    let channel_name = dotenv::var("TWITCH_CHANNEL_NAME")?;

    let access_token = dotenv::var("TWITCH_ACCESS_TOKEN")?;

    Ok((channel_name, access_token))
}
