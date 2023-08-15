mod cloudflare;
use crate::cloudflare::keys;

struct User {
    email: String,
    groups: String,
}

impl User {
    fn new(email: String, groups: String) -> User {
        User { email, groups }
    }
}

fn main() {
    let user = User::new(String::from("foo@bar"), String::from("foo@bar"));
    println!("email is {}, groups are {}", user.email, user.groups);
    match keys::keys(&String::from("https://company.cloudflareaccess.com/cdn-cgi/access/certs")) {
        Ok(text)  =>  for jwk_key in text.keys {
            println!("Key: {}", jwk_key.n);
            println!("Alg: {}", jwk_key.alg);
        }
        Err(e) => println!("Error: {}", e),
    }
}
