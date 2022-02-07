use reqwest::{header::HeaderMap, Client, Response};
use serde_json::Value;
use std::{collections::HashMap, error::Error};

// now uses staging branch

//
/// `login()` returns a mutated client with set duolingo login headers and cookies.
///
/// it takes in a `username: &String`, `password: &String`, and a login `endpoint: &str`.
/// 
/// #### example:
/// ```
/// use duolingo_rs::{login};
/// use reqwest::Client;
///
/// fn main() {
///
///     //
///     // please FOR THE LOVE OF GOD don't hardcode your
///     // password in any app you make with this :(
///     let my_username: String = String::from("user0");
///     let my_password: String = String::from("unsafePa$5w0rd1234"); // grab this from a var or something
///
///     //
///     // use reqwest's Client to log in and set session cookies
///     let login_client = login(my_username,my_password,login_endpoint)?;
///    
///     // pass off the client anywhere you want now!
/// }
/// ```
pub async fn login(
    username: String,
    password: String,
    endpoint: &str,
) -> Result<Client, Box<dyn Error>> {
    //
    // DEFINE DEFAULT HEADER VALUES.
    let content_type = String::from("application/json");
    let accept = String::from("text/plain");
    let accept_encoding = String::from("identity");
    let user_agent = String::from("duoalert_oxide");

    let mut login_json = HashMap::new();
    let mut login_headers = HeaderMap::new();

    //
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

    //
    // form Auth header with values
    login_headers.insert(
        "Authorization",
        (format!("Bearer {}", response_headers["jwt"].to_str()?)).parse()?,
    );

    Ok(client.clone())
}

//TODO:  impliment fetch_streak (fetch_Streak_map() but for single user)

//
/// fetches duolingo data for a vector of usernames
/// , `&Vec<String>`, with a given reqwest `Client`
///
/// #### example:
/// ```
/// use duolingo_rs::{login,fetch_streak_map};
/// use reqwest::Client;
///
/// fn main() {
///
///     //
///     // please FOR THE LOVE OF GOD don't hardcode 
///     // your password in any app you make with this.
///     let my_username: String = String::from("user0");
///     let my_password: String = 
///         String::from("unsafePa$5w0rd1234"); // grab this from a var or something
///
///     //
///     // use reqwest's Client to log in and set
///     // session cookies
///     let login_client = login(my_username,my_password,login_endpoint)?;
///     
///     //
///     // have your Vec ready!!!
///     let mut userlist: Vec<String> = Vec::new();
///     userlist.push(String::from("user1"));
///     userlist.push(String::from("user2"));
///
///     //return a hashmap with a username and a streak
///     let new_data: HashMap<String,u16> = fetch_streak_map(&userlist,login_client).await?;
/// }
/// ```
pub async fn fetch_streak_map(
    users: Vec<String>,
    client: Client,
) -> Result<HashMap<String, u16>, Box<dyn Error>> {
    //maps users as a KVP (user: String, and streak: u16)
    let mut user_map: HashMap<String, u16> = HashMap::new();
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
    Ok(user_map)
}
