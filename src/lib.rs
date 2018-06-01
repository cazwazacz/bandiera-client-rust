extern crate reqwest;
extern crate serde_json;

pub struct Client {
    base_url: String
}

pub fn create_client(base_url: String) -> Client {
    let base_url_string: String = base_url.to_owned();
    Client { base_url: base_url_string }
}

impl Client {
    pub fn get_all(&mut self) -> serde_json::Value {
        let endpoint = "/api/v2/all";

        let full_url = format!("{}{}", &mut self.base_url, endpoint);

        Client::make_request(&*full_url)
    }

    pub fn get_feature(&mut self, group: &str, feature: &str) -> bool {
        let endpoint = format!("/api/v2/groups/{}/features/{}", group, feature);

        let full_url = format!("{}{}", &mut self.base_url, endpoint);

        let response = serde_json::Value::as_bool(&Client::make_request(&*full_url));

        response.unwrap()
    }

    fn make_request(url: &str) -> serde_json::Value {
        let json: serde_json::Value = match reqwest::get(url) {
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
        };

        let response = json["response"].to_owned();

        response
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
