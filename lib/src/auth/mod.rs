use reqwest::Error;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Property {
    name: String,
    value: String,
    signature: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileResponse {
    id: String,
    name: String,
    properties: Vec<Property>,
}


/// Converts a Mojang-style UUID (32 chars) to standard UUID format
///
/// This is required to be done with a UUID received form the HTTP request before sending it back to the Minecraft client (https://minecraft.wiki/w/Java_Edition_protocol/Encryption#Server)
fn format_uuid(raw: &str) -> String {
    format!(
        "{}-{}-{}-{}-{}",
        &raw[0..8],
        &raw[8..12],
        &raw[12..16],
        &raw[16..20],
        &raw[20..32]
    )
}

/// Sends an HTTP request to Mojang's session server
///
/// Info from: https://minecraft.wiki/w/Java_Edition_protocol/Encryption#Server
///
/// # Arguments
///
/// * `server_hash` - Server hash obtained from the c2s packet
/// * `encode_query_params` - if this is true, the function will properly encode the query parameters instead of just formatting a string with them.
async fn get_profile(username: &str, server_hash: &str, ip: Option<&str>, encode_query_params: bool) -> Result<ProfileResponse, Error> {
    if encode_query_params {
        let mut url = Url::parse("https://sessionserver.mojang.com/session/minecraft/hasJoined").unwrap();
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("username", username);
            query.append_pair("serverId", server_hash);
            if let Some(ip) = ip {
                query.append_pair("ip", ip);
            }
        }
        let resp = reqwest::get(url).await?;
        Ok(resp.json().await?)
    } else {
        let mut url = format!(
            "https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}",
            username, server_hash
        );
        if let Some(ip) = ip {
            url.push_str(&format!("&ip={}", ip));
        }
        let resp = reqwest::get(&url).await?;
        Ok(resp.json().await?)
    }
}