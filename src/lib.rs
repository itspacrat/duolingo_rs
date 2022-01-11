use reqwest::{header::HeaderMap, Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, from_value, Value};
use std::{
    collections::HashMap,
    error::Error,
    fs::{read_to_string, File},
    path::Path,
};

/// `login()` returns a mutated client with set duolingo login headers and cookies.
///
/// it takes in a `username: &String`, `password: &String`, and a login `endpoint: &str`.
pub async fn login(
    username: String,
    password: String,
    endpoint: &str,
) -> Result<Client, Box<dyn Error>> {
    // DEFINE DEFAULT HEADER VALUES.
    let content_type = String::from("application/json");
    let accept = String::from("text/plain");
    let accept_encoding = String::from("identity");
    let user_agent = String::from("duoalert_oxide");

    let mut login_json = HashMap::new();
    let mut login_headers = HeaderMap::new();

    // ADD LOGIN HEADERS TO NEW CLIENT.
    println!("inserting login body...");
    login_json.insert("login", username);
    login_json.insert("password", password);
    println!("done.\n");

    println!("inserting login headers...");
    login_headers.insert("Content-Type", (&content_type).parse()?);
    login_headers.insert("Accept", (&accept).parse()?);
    login_headers.insert("Accept-Encoding", (&accept_encoding).parse()?);
    login_headers.insert("User-Agent", (&user_agent).parse()?);
    println!("done.\n");

    let client = Client::builder()
        .default_headers(login_headers.clone())
        .cookie_store(true)
        .build()?;

    println!("Posting auth request...");
    let resp: Response = client.post(endpoint).json(&login_json).send().await?;
    println!("done.\n");

    let response_headers = resp.headers();

    // form Auth header with values
    login_headers.insert(
        "Authorization",
        (format!("Bearer {}", response_headers["jwt"].to_str()?)).parse()?,
    );

    Ok(client.clone())
}

/// fetches duolingo data for a vector of usernames
/// , `&Vec<String>`, with a given reqwest `Client`
///
/// #### example:
/// ```
/// use duolingo_rs::fetch;
/// use reqwest::Client;
///
/// fn main() {
///     let login_client = login(my_username,my_password,login_endpoint);
///     let userlist: Vec<String>
///
///     //returns a hashmap with a username and a streak
///     let new_data: HashMap<String,u16> = fetch(&userlist,login_client).await?;
/// }
/// ```
pub async fn fetch(
    users: &Vec<String>,
    client: Client,
) -> Result<HashMap<String, u16>, Box<dyn Error>> {
    //maps users as a KVP (user: String, and streak: u16)
    let mut user_map: HashMap<String, u16> = HashMap::new();

    // loop through users in config and fetch profile responses (SLOW :<)
    for user in users {
        println!("    fetching data for user {}", &user);

        let main_fetch_url = format!("https://duolingo.com/users/{}", &user);
        let resp: String = client
            .get(main_fetch_url)
            //.headers(headers)
            .send()
            .await?
            .text()
            .await?;
        println!("    done.\n");

        // convert the json resp into a Value for easy map insertion
        let user_val_r: Value = serde_json::from_str(&resp)?;
        // grab the little fucker that we went through this process
        // for. 0/10 not worth the hassle, duolingo. i hope you push
        // a break to prod or something.
        let user_val: String = (user_val_r["site_streak"].clone()).to_string();

        user_map.insert(user.clone(), user_val.parse()?);
    }

    /*let streak_data: StreakData = StreakData {
        users: Some(user_map),
    };*/

    Ok(user_map)
}

/// test if a streak is greater than, equal to,
/// or less than the previous streak.
pub fn check(old_path: &str, new_data: Value) -> Result<(), Box<dyn Error>> {
    let previous_r: HashMap<String, u16> = from_str(&(read_to_string(old_path)?))?; // > Value
    let new_r: HashMap<String, u16> = from_value(new_data)?;

    /* DEBUG STATEMENTS */
    println!("OLD\n{:#?}\n", &previous_r);
    println!("NEW\n{:#?}\n", &new_r);

    for (old_key, old_streak) in previous_r.clone() {
        for (new_key, new_streak) in new_r.clone() {
            if new_key == old_key {
                if new_streak > old_streak {
                    //extend streak for user
                    println!("streak extension: {} - {}", &new_key, &new_streak)
                } else if new_streak < old_streak {
                    // if new data is less (lost streak)
                    println!("streak loss: {} - {}", &new_key, &old_streak)
                } else {
                    // if they are neither greater nor less than eachother
                    println!("no change: {}",&new_key)
                }
            } else {
                // Don't do shit because you're making an apple to oranges comparison
            }
        }
    }

    Ok(())
}
