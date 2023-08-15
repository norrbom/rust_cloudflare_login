use serde::Deserialize;
use serde::Serialize;
use jsonwebtoken::DecodingKey;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
   sub: String,
   company: String
}

// pub fn verify(token: &str, public_key: &str) -> Result<Claims, Box<dyn std::error::Error>> {
//     let alg = jsonwebtoken::Algorithm::RS256;
    
//     let claims = jsonwebtoken::decode::<Claims>(token, &DecodingKey::from_secret(public_key), &header)?;
//     Ok(claims.claims)
// }