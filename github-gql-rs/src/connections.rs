use interfaces::*;
use objects::*;
use unions::*;
use scalars::*;
use edges::*;

impl<A,E,G> GitObject for CommitCommentConnection<A,E,G> {}
impl<A,E,G> Node for CommitCommentConnection<A,E,G> {}
impl<A,E,G> Subscribable for CommitCommentConnection<A,E,G> {}
pub struct CommitCommentConnection<A,E,G>
{
    pub edges: Option<Vec<CommitCommentEdge<A,E,G>>>,
    pub nodes: Option<Vec<CommitComment<A,E,G>>>,
    pub pageInfo: PageInfo,
    pub totalCount: Int,
}

pub struct CommitHistoryConnection<A,E,G>
{
    pub edges: Option<Vec<CommitEdge<A,G>>>,
    pub nodes: Option<Vec<Commit<A,E,G>>>,
    pub pageInfo: PageInfo,
}

pub struct DeploymentStatusConnection {
    pub edges: Option<DeploymentStatusEdge>,
    pub nodes: Option<Vec<DeploymentStatusEdge>>
    pub pageInfo: PageInfo,
    pub totalCount: Int,
}

pub struct GistCommentConnection {
    pub edges: Option<GistCommentEdge>,
    pub nodes: Option<Vec<GistComment>>
    pub pageInfo: PageInfo,
    pub totalCount: Int,
}

pub struct ReactionConnection {
    pub edges: Option<Vec<ReactionEdge>>,
    pub nodes: Vec<Reaction>,
    pub pageInfo: PageInfo,
    pub totalCount: Int,
    pub viewerHasReacted: Boolean,
}

pub struct StargazerConnection {
    pub edges: Option<StargazerEdge>,
    pub nodes: Option<Vec<User>>
    pub pageInfo: PageInfo,
    pub totalCount: Int,
}
