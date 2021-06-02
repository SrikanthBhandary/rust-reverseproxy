use std::fs;
use serde_json::{Value, Map};
use std::error::Error;

pub struct ConfigSerde;

impl ConfigSerde {
    pub fn read_config(path: &str) -> Result<Map<String, Value>, Box<Error>> {
        let config = fs::read_to_string(path)?;
        let parsed: Value = serde_json::from_str(&config)?;
        let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
        ;
        Ok(obj)
    }

    pub fn detect_url(config_obj: Map<String, Value>, path: String) -> String {
        let mut n = path.to_owned();
        let mut url= String::from("");
        loop {
            if config_obj.contains_key(&n) {
                url  = String::from(&config_obj[&n].to_string());
                return url;
            }
            let tmp: Vec<&str> = n.split("/").collect();
            if tmp.len() == 1 {
                break;
            }
            n = tmp[0..tmp.len() - 1].join("/");
        }
        url
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_positive_results_with_single_route_exist() {
        let obj = ConfigSerde::read_config("/Users/srikanthbhandary/IdeaProjects/reverse-proxy/src/config.json");
        let mut k: Map<String, Value> = Map::new();
        match obj {
            Ok(v) => {
                k = v;
            }
            Err(_e) => println!("Error")
        }
        let url = ConfigSerde::detect_url(k, String::from("/linkedin/1")).replace("\"", "");
        assert_eq!("https://www.sfsf.com", url);
    }

    #[test]
    fn test_positive_results_when_no_route_exist() {
        let obj = ConfigSerde::read_config("/Users/srikanthbhandary/IdeaProjects/reverse-proxy/src/config.json");
        let mut k: Map<String, Value> = Map::new();
        match obj {
            Ok(v) => {
                k = v;
            }
            Err(_e) => println!("Error")
        }
        let url = ConfigSerde::detect_url(k, String::from("/test/1")).replace("\"", "");
        assert_eq!("", url);
    }

    #[test]
    fn test_positive_results_when_multiple_route_exist() {
        let obj = ConfigSerde::read_config("/Users/srikanthbhandary/IdeaProjects/reverse-proxy/src/config.json");
        let mut k: Map<String, Value> = Map::new();
        match obj {
            Ok(v) => {
                k = v;
            }
            Err(_e) => println!("Error")
        }
        let url = ConfigSerde::detect_url(k, String::from("/facebook/1/2")).replace("\"", "");
        assert_eq!("https://www.facebook.com", url);
    }
}


