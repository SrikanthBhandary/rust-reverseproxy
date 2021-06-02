use jwt_simple::Error;
use jwt_simple::prelude::*;
use hyper::{Body, Request};
use regex::Regex;

pub struct JwtHelper{}

impl JwtHelper{
    pub fn generate_jwt_token(&self, key: HS256Key) -> Result<String, Error>{
        let claims = Claims::create(Duration::from_hours(2));
        key.authenticate(claims)
    }

    pub fn read_secret_key_from_bytes(&self, secret_key: &[u8]) -> HS256Key{
        HS256Key::from_bytes(secret_key)
    }

    pub fn verify_token(&self,key: HS256Key, token:&str) -> Result<JWTClaims<NoCustomClaims>, Error>{
        key.verify_token::<NoCustomClaims>(token, None)
    }

    pub fn validate_request(&self, req: &Request<Body>, secret_key: &str) -> bool {
        if req.headers().contains_key("Cookie") {
            let cookies: String = format!("{:?}", req.headers()["Cookie"]).replace("\"", "").replace(";", "");
            let re = Regex::new(r"ppu-jwt-token=(?P<token>(\S*))").unwrap();
            return match re.captures(&cookies) {
                None => false,
                Some(value) => {
                    let key = self.read_secret_key_from_bytes(secret_key.as_bytes());
                    let token = String::from(&value["token"]);
                    let claims = self.verify_token(key, &token);
                    match claims {
                        Ok(_v) => true,
                        Err(_e) => false
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_token() {
        let helper =JwtHelper{};
        let x = helper.read_secret_key_from_bytes("Srikanth1234*".as_bytes());
        println!( "{:?}",x);

    }

    #[test]
    fn generate_token() {
        let helper =JwtHelper{};
        let x = helper.read_secret_key_from_bytes("Srikanth1234*".as_bytes());
        println!("Token {:?}", helper.generate_jwt_token(x));
    }
}
