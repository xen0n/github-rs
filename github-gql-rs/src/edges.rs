use scalars::*;
use objects::*;
use unions::*;
use interfaces::*;

pub struct CommitCommentEdge<A,E,G>
{
    pubconnectionscursor: String,
    pub node: Option<CommitComment<A,E,G>>
}

pub struct CommitEdge<A,E,G>
{
    pub cursor: String,
    pub node: Option<Commit<A,E,G>>,
}

pub struct DeploymentStatusEdge
{
    pub cursor: String,
    pub node: Option<DeploymentStatus>,
}

pub struct GistCommentEdge
{
    pub cursor: String,
    pub node: Option<GistComment>,
}

pub struct ReactionEdge
{
    pub cursor: String,
    pub node: Option<Reaction>,
}

pub struct StargazerEdge
{
    pub cursor: String,
    pub node: User,
    pub starredAt: DateTime,
}
