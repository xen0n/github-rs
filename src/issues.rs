//! Trait definition related to Issues on Github
use json::{Issue, Assignees};
use requests::*;
use github::Client;
use error::*;
use serde_json;

/// Trait dealing with Issues on GitHub
pub trait Issues {
    /// ### Request Type:
    /// `POST`
    /// ### Endpoint:
    /// /repos/:owner/:repo/issues/:number/assignees
    /// ### Description
    /// Returns
    fn add_assignees(&self, owner: &str, repo: &str, number: u64, assignees: Assignees) -> Result<Issue>;

    /// ### Request Type:
    /// `DELETE`
    /// ### Endpoint:
    /// /repos/:owner/:repo/issues/:number/assignees
    /// ### Description
    /// Returns
    fn remove_assignees(&self, owner: &str, repo: &str, number: u64, assignees: Assignees) -> Result<()>;

}

impl Issues for Client {
    fn add_assignees(&self, owner: &str, repo: &str, number: u64, assignees: Assignees) -> Result<Issue> {
        let url = format!("https://api.github.com/repos/{}/{}/issues/{}/assignees", owner, repo, number);
        let res = post(&url,
                        self.get_headers().clone(),
                        serde_json::to_string(&assignees)?)?;
        try_serde!(serde_json::from_str(&res))
    }
    fn remove_assignees(&self, owner: &str, repo: &str, number: u64, assignees: Assignees) -> Result<()> {
        let url = format!("https://api.github.com/repos/{}/{}/issues/{}/assignees", owner, repo, number);
        // This returns a status code we want it to return actual data
        // This part is inconsistent with Github docs saying deletes don't
        // return anything
        let res = delete_with_data(&url,
                                   self.get_headers().clone(),
                                   serde_json::to_string(&assignees)?)?;
        Ok(())
    }
}
