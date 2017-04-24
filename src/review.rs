use github::Client;
use json::{MakeReview, SubmitReview, Review};
use requests::*;
use error::*;
use serde_json;

/// Trait used to define access to endpoints grouped under `Review` in the Github API
/// specification
pub trait Reviews {
    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /repos/:owner/:repo/pulls/:number/reviews
    /// ### Description
    /// Returns 
    fn get_reviews(&self, owner: &str, repo: &str, number: u64) -> Result<Vec<Review>>;

    /// ### Request Type:
    /// `POST`
    /// ### Endpoint:
    /// /repos/:owner/:repo/pulls/:number/reviews
    /// ### Description
    /// Returns 
    fn post_review(&self, owner: &str, repo: &str, number: u64, mk_review: MakeReview) -> Result<Review>;

    /// ### Request Type:
    /// `POST`
    /// ### Endpoint:
    /// /repos/:owner/:repo/pulls/:number/reviews/:id/events
    /// ### Description
    /// Returns 
    fn submit_review(&self, owner: &str, repo: &str, number: u64, id: u64, sub_review: SubmitReview) -> Result<Review>;

    /// ### Request Type:
    /// `DELETE`
    /// ### Endpoint:
    /// /repos/:owner/:repo/pulls/:number/reviews/:id/events
    /// ### Description
    /// Returns 
    fn delete_review(&self, owner: &str, repo: &str, number: u64, id: u64) -> Result<()>;
}

impl Reviews  for Client {
    fn get_reviews(&self, owner: &str, repo: &str, number: u64) -> Result<Vec<Review>> {
        let url = format!("https://api.github.com/repos/{}/{}/pulls/{}/reviews",
                          owner, repo, number);
        let data = get(&url, self.get_headers().clone())?;
        try_serde!(serde_json::from_str(&data))
    }

    fn post_review(&self, owner: &str, repo: &str, number: u64, mk_review: MakeReview) -> Result<Review> {
        let url = format!("https://api.github.com/repos/{}/{}/pulls/{}/reviews",
                          owner, repo, number);
        let res = post(&url,
                        self.get_headers().clone(),
                        serde_json::to_string(&mk_review)?)?;
        try_serde!(serde_json::from_str(&res))
    }

    fn submit_review(&self, owner: &str, repo: &str, number: u64, id: u64, sub_review: SubmitReview) -> Result<Review> {
        let url = format!("https://api.github.com/repos/{}/{}/pulls/{}/reviews/{}/events",
                          owner, repo, number, id);
        let res = post(&url,
                        self.get_headers().clone(),
                        serde_json::to_string(&sub_review)?)?;
        try_serde!(serde_json::from_str(&res))
    }

    fn delete_review(&self, owner: &str, repo: &str, number: u64, id: u64) -> Result<()> {
        let url = format!("https://api.github.com/repos/{}/{}/pulls/{}/reviews/{}/events",
                          owner, repo, number, id);
        let _ = delete(&url,
                        self.get_headers().clone())?;
        Ok(())
    }
}
