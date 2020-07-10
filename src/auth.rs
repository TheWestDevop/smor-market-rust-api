
use rocket::{Outcome, request::{self,Request,FromRequest}, http::Status};
use hmac::{Hmac, NewMac};
use jwt::*;
use sha2::Sha256;
use std::collections::BTreeMap;
use rocket_contrib::json::{JsonValue};





#[derive(Debug)]
pub struct  ApiKey (pub String);

pub fn verify_token(token:&str) -> Result<String,Error>{
    // type Error  = !;
    let key: Hmac<Sha256> = Hmac::new_varkey(b"mysecret").unwrap();
    let token_claims: BTreeMap<String, String> = token.verify_with_key(&key)?;
    // Err(Error::NoClaimsComponent);
    // println!("token sub is {} and company is {}",token_claims["sub"].to_string(),token_claims["company"].to_string() );
    Ok(token_claims["sub"].to_string())
}

pub fn generate_token(username:&str,iat:&str) -> JsonValue {
    let key: Hmac<Sha256> = Hmac::new_varkey(b"mysecret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", username);
    claims.insert("iat",iat);
    claims.insert("company", "smor_group");

    let token = claims.sign_with_key(&key).unwrap();
    if token.is_empty() {
        return json!({
            "status":"error",
            "message":"an error occurred kindly try again"
        });
       }else{
        return json!({
            "status":"success",
            "token":token
        });
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
type Error = ();

fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()>{
  
    let keys: Vec<_> = request.headers().get("authorization").collect();


    // print!("request header ---> {:?}",keys);

    // let nkey = ;
   
     if keys.is_empty() || keys.len() != 1  || keys[0].trim().is_empty() { 
       return  Outcome::Failure((Status::Unauthorized,()));
     }else {
     match verify_token(keys[0]) {
        Ok(claim) => Outcome::Success(ApiKey(claim)),
        Err(_) => Outcome::Failure((Status::NonAuthoritativeInformation,())),
        // _ => Outcome::Failure((Status::NonAuthoritativeInformation,())),
    } 
  }
    // return  keys;
 }
}





