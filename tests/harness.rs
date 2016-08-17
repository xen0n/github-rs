extern crate github_rs;
use github_rs::github::*;

pub fn gen_client() -> Client {
        use std::io::BufReader;
        use std::io::prelude::*;
        use std::fs::File;
        let file = File::open("tests/auth_token").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();

        // error handle better
        let _ = reader.read_line(&mut buffer);
        Client::new(String::from(buffer.trim())).unwrap()
}

#[cfg(test)]
mod users {
    use super::gen_client;
    use github_rs::github::*;
    #[test]
    fn test_get_user() {
        let gh = gen_client();
        // We want the test to fail if it didn't get anything
        let user = gh.get_user().unwrap();

        // If the id is the same then we have succesfully queried for the github-rs user
        // that is used for these tests and the User struct has come back
        assert_eq!(user.id,20033736)
    }
}
