extern crate reqwest;
extern crate serde_json;

pub struct Client {
    base_url: String
}

pub fn create_client(base_url: String) -> Client {
    Client { base_url }
}

impl Client {
    pub fn get_all(&mut self) -> serde_json::Value{
        let full_url = &mut self.base_url;
        let endpoint = "/api/v2/all";
        full_url.push_str(endpoint);

        Client::make_request(&*full_url)
    }

    fn make_request(url: &str) -> serde_json::Value {
        match reqwest::get(url) {
            Ok(mut response) => {
                if response.status() == reqwest::StatusCode::Ok {
                    match response.text() {
                        Ok(text) => serde_json::from_str(&*text).expect("JSON was not well-formatted"),
                        Err(_) => serde_json::from_str("{}").expect("JSON was not well-formatted"),
                    }
                } else {
                    serde_json::from_str("{}").expect("JSON was not well-formatted")
                }
            }
            Err(_) => serde_json::from_str("{}").expect("JSON was not well-formatted")
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
