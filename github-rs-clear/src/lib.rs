extern crate github_rs;
use github_rs::client::Github;
use github_rs::Json;

pub struct GitHub {
    client: Github,
}

impl GitHub {
    pub fn new<T>(token: T) -> Self where T: AsRef<str> {
        Self {
            client: Github::new(token)
        }
    }

    pub fn get_self(&self) -> Option<Json> {
        let (_, _, val) = self.client
            .get()
            .user()
            .execute()
            .unwrap();
        val
    }

}
