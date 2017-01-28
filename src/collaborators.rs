use github::Client;
use json::Permissions;
use hyper::status::StatusCode;
use requests::*;
use error::*;
use serde_json;

/// Trait used to define access to endpoints grouped under `Collaborators` in the Github API
/// specification
pub trait Collaborators {
    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /repos/:owner/:repo/collaborators/:username/
    /// ### Description
    /// Returns a boolean if the user is a collaborator
    fn is_collaborator(&self, owner: &str, repo: &str, user: &str) -> Result<bool>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /repos/:owner/:repo/collaborators/:username/permission
    /// ### Description
    /// Returns a boolean if the user is a collaborator
    fn get_permissions(&self, owner: &str, repo: &str, user: &str) -> Result<Permissions>;
}

impl Collaborators for Client {
    fn is_collaborator(&self, owner: &str, repo: &str, user: &str) -> Result<bool> {
        let url = format!("https://api.github.com/repos/{}/{}/collaborators/{}",
                          owner, repo, user);
        let data = get_status_code(&url, self.get_headers().clone())?;
        Ok(StatusCode::NoContent == data)
    }

    fn get_permissions(&self, owner: &str, repo: &str, user: &str) -> Result<Permissions> {
        let url = format!("https://api.github.com/repos/{}/{}/collaborators/{}/permission",
                          owner, repo, user);
        let res = get(&url, self.get_headers().clone())?;
        try_serde!(serde_json::from_str(&res))
    }
}
