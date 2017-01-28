//! Structs related to JSON serialization/deserialization
#![allow(missing_docs)]
extern crate serde_json;
use serde_json::Value;

/// Information related to a user on Github is stored in this struct.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct User {
    // Public
    // These will all have a value of some sort for any
    // user
    pub id: u64,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub site_admin: bool,

    // Optional Public
    // These are optional for a user to fill in and so
    // there might not be a value to have.
    pub login: Option<String>,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<String>,
    pub bio: Option<String>,

    // These values also may or may not be available depending
    // on the user endpoint used. Accessing the username directly
    // or the authenticated user themselves will get these fields.
    pub public_repos: Option<u64>,
    pub public_gists: Option<u64>,
    pub followers: Option<u64>,
    pub following: Option<u64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    // Private User
    // The fields below here are only accesible if looking at your
    // own profile using an AuthToken. Otherwise these fields will
    // have None for users other than yourself.
    pub private_gists: Option<u64>,
    pub total_private_repos: Option<u64>,
    pub owned_private_repos: Option<u64>,
    pub disk_usage: Option<u64>,
    pub collaborators: Option<u64>,
    pub plan: Option<Plan>,
    #[serde(rename="type")]
    pub type_: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Plan {
    pub name: String,
    pub space: u64,
    pub collaborators: u64,
    pub private_repos: u64,
}

/// Used to update an authenticated user.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PatchUser {
    pub name: String,
    pub email: String,
    pub blog: String,
    pub company: String,
    pub location: String,
    pub hireable: bool,
    pub bio: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub verifiable_password_authentication: bool,
    pub github_services_sha: String,
    pub hooks: Vec<String>,
    pub git: Vec<String>,
    pub pages: Vec<String>,
    pub importer: Vec<String>,
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct RateLimit {
    pub resources: Resources,
    pub rate: Rate,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Resources {
    pub core: Core,
    pub search: Search,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Rate {
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Search {
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Core {
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Email {
    pub email: String,
    pub verified: bool,
    pub primary: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RepoCreate {
    pub name: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub private: Option<bool>,
    pub has_issues: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_downloads: Option<bool>,
    pub team_id: Option<i32>,
    pub auto_init: Option<bool>,
    pub gitignore_template: Option<String>,
    pub license_template: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SSHKey {
    pub id: u64,
    pub key: String,
    pub url: Option<String>,
    pub title: Option<String>,
    pub verified: Option<bool>,
    pub created_at: Option<String>,
    pub read_only: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GPGKey {
    pub id: u64,
    pub primary_key_id: Option<u64>,
    pub key_id: String,
    pub public_key: String,
    pub emails: Vec<Email>,
    pub subkeys: Option<Vec<GPGKey>>,
    pub can_sign: bool,
    pub can_encrypt_comms: bool,
    pub can_encrypt_storage: bool,
    pub can_certify: bool,
    pub created_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GPGKeyPost {
    pub armored_public_key: String,
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GitIgnore {
    pub name: String,
    pub source: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Markdown {
    pub text: String,
    pub mode: Option<String>,
    pub context: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Event {
    #[serde(rename="type")]
    pub type_: String,
    pub public: bool,
    pub payload: Vec<String>, // Correct type?
    pub repo: Repo,
    pub actor: Actor,
    pub org: Org,
    pub created_at: String,
    pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Repo {
    pub id: u64,
    pub name: String,
    pub url: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Actor {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub avatar_url: String,
    pub url: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Org {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub avatar_url: String,
    pub url: String,
}

// Git Data Types
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Blob {
    pub content: String,
    pub encoding: String,
    pub url: String,
    pub sha: String,
    pub size: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BlobCreate {
    pub content: String,
    pub encoding: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Commit {
    pub sha: String,
    pub url: String,
    pub author: Author,
    pub committer: Committer,
    pub message: String,
    pub tree: Tree,
    pub parents: Vec<Parent>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Author {
    pub date: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Committer {
    pub date: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Tree {
    pub url: String,
    pub sha: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Parent {
    pub url: String,
    pub sha: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Ref {
    #[serde(rename="ref")]
    pub ref_: String,
    pub url: String,
    pub object: Object,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Object {
    #[serde(rename="type")]
    pub type_: String,
    pub sha: String,
    pub url: String,
}

// Events
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CommitComment {
    pub action: String,
    pub comment: Comment,
    pub repository: Repository,
    pub sender: Sender,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub url: String,
    pub html_url: String,
    pub id: u64,
    pub user: User,
    pub position: Option<u64>,
    pub line: Option<u64>,
    pub path: Option<String>,
    pub commit_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub body: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub owner: Owner,
    pub private: bool,
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    pub forks_url: String,
    pub keys_url: String,
    pub collaborators_url: String,
    pub teams_url: String,
    pub hooks_url: String,
    pub issue_events_url: String,
    pub events_url: String,
    pub assignees_url: String,
    pub branches_url: String,
    pub tags_url: String,
    pub blobs_url: String,
    pub git_tags_url: String,
    pub git_refs_url: String,
    pub trees_url: String,
    pub statuses_url: String,
    pub languages_url: String,
    pub stargazers_url: String,
    pub contributors_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub commits_url: String,
    pub git_commits_url: String,
    pub comments_url: String,
    pub issue_comment_url: String,
    pub contents_url: String,
    pub compare_url: String,
    pub merges_url: String,
    pub archive_url: String,
    pub downloads_url: String,
    pub issues_url: String,
    pub pulls_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub labels_url: String,
    pub releases_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    pub homepage: Option<String>,
    pub size: u64,
    pub stargazers_count: u64,
    pub watchers_count: u64,
    pub language: Option<String>,
    pub has_issues: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: u64,
    pub mirror_url: Option<String>,
    pub open_issues_count: u64,
    pub forks: u64,
    pub open_issues: u64,
    pub watchers: u64,
    pub default_branch: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Owner {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename="type")]
    pub type_: String,
    pub site_admin: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Sender {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename="type")]
    pub type_: String,
    pub site_admin: bool,
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct IssueComment {
    pub action: String,
    pub issue: Issue,
    pub comment: Option<Comment>,
    // Unsure what this entails. Docs are skimpy
    pub changes: Option<Value>,
    #[serde(rename="changes[title][from]")]
    pub changes_title_from: Option<String>,
    #[serde(rename="changes[body][from]")]
    pub changes_body_from: Option<String>,
    pub assignee: Option<Vec<Assignee>>,
    pub label: Option<Label>,
}

// Fill out more
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Issue {
    pub id: u64,
    pub url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub repository_url: String,
    pub events_url: String,
    pub html_url: String,
    pub number: u64,
    pub title: String,
    pub user: User,
    pub repository: Option<Repository>,
    pub sender: Option<Sender>,
    pub labels: Vec<Label>,
    pub state: String,
    pub locked: bool,
    pub assignee: Option<Assignee>,
    pub assignees: Option<Vec<Assignee>>,
    pub milestone: Option<Milestone>,
    pub comments: u64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub body: String,
}

// Fill out later
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Assignee {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename="type")]
    pub type_: String,
    pub site_admin: bool
}

// Fill out later
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Changes {
    pub dummy: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Label {
    pub id: u64,
    pub url: String,
    pub name: String,
    pub color: String,
    pub default: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Milestone {
    pub url: String,
    pub html_url: String,
    pub labels_url: String,
    pub id: u64,
    pub number: u64,
    pub state: String,
    pub title: String,
    pub description: String,
    pub creator: User,
    pub open_issues: u64,
    pub closed_issues: u64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: String,
    pub due_on: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Assignees {
    pub assignees: Vec<String>
}

// PR Reviews
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Review {
    pub id: u64,
    pub user: User,
    pub body: String,
    pub commit_id: String,
    pub state: String,
    pub html_url: String,
    pub pull_request_url: String,
    pub _links: Links
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Links {
    pub html: Href,
    pub pull_request: Href,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Href {
    pub href: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MakeReview {
    pub body: String,
    pub event: String,
    pub comments: Vec<DraftComment>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DraftComment {
    pub path: String,
    pub position: u64,
    pub body: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SubmitReview {
    pub body: String,
    pub event: String,
}

// Collaborator JSON
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Permissions {
    pub permission: String,
    pub user: User,
}
