use interfaces::*;
use scalars::*;
use enums::*;
use connections::*;

impl<A> Node for AddedToProjectEvent<A> {}
pub struct AddedToProjectEvent<A> {
    /// Identifies the actor who performed the event.
    pub actor: Option<A>,
    /// Identifies the date and time when the object was created.
    pub createdAt: DateTime,
    // Deprecation notice
    /// Identifies the primary key from the database.
    pub databaseId: Option<Int>,
    pub id: Id,
}

impl<A, As> Node for AssignedEvent<A, As> {}
pub struct AssignedEvent<A, As> {
    /// Identifies the actor who performed the event.
    pub actor: Option<A>,
    ///Identifies the assignable associated with the event.
    pub assignable: As,
    /// Identifies the date and time when the object was created.
    pub createdAt: DateTime,
    pub id: Id,
    /// Identifies the user who was assigned.
    pub user: Option<User>
}

impl<A> Node for BaseRefChangedEvent<A> {}
pub struct BaseRefChangedEvent<A> {
    /// Identifies the actor who performed the event.
    pub actor: Option<A>,
    /// Identifies the date and time when the object was created.
    pub createdAt: DateTime,
    /// Deprecation notice
    /// Exposed database IDs will eventually be removed in favor of global
    /// Relay IDs.
    pub databaseId: Option<Int>,
    /// Identifies the primary key from the database.
    pub id: Id,
}

impl<A,E,G> Node for BaseRefForcePushedEvent<A,E,G> {}
pub struct BaseRefForcePushedEvent<A,E,G> {
    /// Identifies the actor who performed the event.
    pub actor: Option<A>,
    /// Identifies the after commit SHA for the `base_ref_force_pushed` event.
    pub afterCommit: Commit<A,E,G>,
    /// Identifies the before commit SHA for the `base_ref_force_pushed` event.
    pub beforeCommit: Commit<A,E,G>,
    /// Identifies the date and time when the object was created.
    pub createdAt: DateTime,
    pub id: Id,
    pub pullRequest: PullRequest,
    pub ref_: Option<Ref>,
}

pub struct Blame<A,E,G> {
    pub ranges: Vec<BlameRange<A,E,G>>,
}

pub struct BlameRange<A,E,G> {
    pub age: Int,
    pub commit: Commit<A,E,G>,
    pub endingLine: Int,
    pub startingLine: Int,
}

impl GitObject for Blob {}
impl Node for Blob {}
pub struct Blob {
    pub abbreviatedOid: String,
    pub byteSize: Int,
    pub commitResourcePath: Uri,
    pub commitUrl: Uri,
    pub id: Id,
    pub isBinary: Boolean,
    pub isTruncated: Boolean,
    pub oid: GitObjectId,
    pub repository: Repository,
    pub text: Option<String>,
}


impl Actor for Bot {}
impl Node for Bot {}
impl UniformResourceLocatable for Bot {}
pub struct Bot {
    pub avatarUrl: Uri, // args size type Int
    pub createdAt: DateTime,
    // Deprecation notice
    pub databaseId: Option<Int>,
    pub id: Id,
    pub login: String,
    pub resourcePath: Uri,
    pub updatedAt: DateTime,
    pub url: Uri,
}

impl<A,C,E,G> Node for ClosedEvent<A,C,E,G> {}
pub struct ClosedEvent<A,C,E,G> {
    pub actor: A,
    pub closable: C,
    pub commit: Option<Commit<A,E,G>>,
    pub createdAt: DateTime,
    pub id: Id,
}

pub struct CodeOfConduct {
    pub body: Option<String>,
    pub key: String,
    pub name: String,
    pub url: Uri,
}

impl<A> Node for CommentDeletedEvent<A> {}
pub struct CommentDeletedEvent<A> {
    pub actor: A,
    pub createdAt: DateTime,
    pub databaseId: Option<Int>,
    pub id: Id,
}

impl<A,E,G> GitObject for Commit<A,E,G> {}
impl<A,E,G> Node for Commit<A,E,G> {}
impl<A,E,G> Subscribable for Commit<A,E,G> {}
pub struct Commit<A,E,G> {
    // Connections
    pub comments: CommitCommentConnection<A,E,G>,
    // args after type String
    // args before type String
    // args first type Int
    // args last type Int
    pub history: CommitHistoryConnection<A,G>,
    // args after type String
    // args author type CommitAuthor
    // args before type String
    // args first type Int
    // args last type Int
    // args path type String
    // args since type GitTimestamp
    // args until type GitTimestamp

    // Fields
    pub abbreviatedOid: String,
    pub author: Option<GitActor>,
    pub authoredByCommitter: Boolean,
    pub blame: Blame<A,E,G>, // args path type String
    pub commitResourcePath: Uri,
    pub commitUrl: Uri,
    pub committedDate: DateTime,
    pub committedViaWeb: Boolean,
    pub committer: Option<GitActor>,
    pub id: Id,
    pub message: String,
    pub messageBody: String,
    pub messageBodyHtml: Html,
    pub messageHeadline: String,
    pub messageHeadlineHtml: Html,
    pub oid: GitObjectId,
    pub repository: Repository,
    pub resourcePath: Uri,
    pub signature: Option<G>,
    pub status: Option<Status>,
    pub tree: Tree,
    pub treeResourcePath: Uri,
    pub treeUrl: Uri,
    pub url: Uri,
    pub ViewerCanSubscribe: Boolean,
    pub viewerSubscrition: SubscriptionState,
}

impl<A,E,G> Comment<A,E> for CommitComment<A,E,G> {}
impl<A,E,G> Deletable for CommitComment<A,E,G> {}
impl<A,E,G> Node for CommitComment<A,E,G> {}
impl<A,E,G> Reactable for CommitComment<A,E,G> {}
impl<A,E,G> RepositoryNode for CommitComment<A,E,G> {}
impl<A,E,G> Updatable for CommitComment<A,E,G> {}
impl<A,E,G> UpdatableComment for CommitComment<A,E,G> {}
pub struct CommitComment<A,E,G> {
    pub reactions: ReactionConnection,
    // args after type String
    // args before type String
    // args content type ReactionContent
    // args first type Int
    // args last type Int
    // args orderBy type ReactionOrder
    pub author: Option<A>,
    pub authorAssociation: CommentAuthorAssociation,
    pub body: String,
    pub bodyHtml: Html,
    pub commit: Commit<A,E,G>,
    pub createdAt: DateTime,
    pub createdViaEmail: Boolean,
    pub databaseId: Option<Int>,
    pub editor: Option<E>,
    pub id: Id,
    pub lastEditAt: Option<DateTime>,
    pub path: Option<String>,
    pub position: Option<Int>,
    pub publishedAt: Option<DateTime>,
    pub reactionGroup: Vec<ReactionGroup>,
    pub repository: Repository,
    pub resourcePath: Uri,
    pub updatedAt: DateTime,
    pub url: Uri,
    pub viewerCanDelete: Boolean,
    pub viewerCanReact: Boolean,
    pub viewerCanUpdate: Boolean,
    pub viewerCannotUpdateReasons: Vec<ViewerCannotUpdateReasons>,
    pub viewerDidAuthor: Boolean,
}

impl Node for CommitCommentThread {}
impl RepositoryNode for CommitCommentThread {}
pub struct CommitCommentThread {
    // Connections
    pub comments: CommitCommentConnection,
    // args after type String
    // args before type String
    // args first type Int
    // args last type Int
    // Fields
    pub commit: Commit,
    pub id: Id,
    pub path: Option<String>,
    pub position: Option<Int>,
    pub repository: Repository,
}

impl Node<A> for ConvertedNoteToIssueEvent<A> {}
pub struct ConvertedNoteToIssueEvent<A> {
    pub actor: Option<A>,
    pub createdAt: DateTime,
    // Deprecation Warning
    pub databaseId: Option<Int>,
    pub id: Id,
}

impl Node<A> for CrossReferencedEvent<A> {}
impl UniformResourceLocatable<A> for ConvertedNoteToIssueEvent<A>
pub struct CrossReferencedEvent<A> {
    pub actor: Option<A>,
    pub createdAt: DateTime,
    pub id: Id,
    pub isCrossRepository: Boolean,
    pub referencedAt: DateTime,
    pub resourcePath: Uri,
    pub source: ReferencedSubject,
    pub target: ReferencedSubject,
    pub url: Uri,
    pub willCloseTarget: Boolean,
}

impl Node<A> for DemilestonedEvent<A> {}
pub struct DemilestonedEvent {
    pub actor: Option<A>,
    pub createdAt: DateTime,
    pub id: Id,
    pub mileStoneTitle: String,
    pub subject: MilestoneItem,
}


impl Node<A> for DeployedEvent<A> {}
pub struct DeployedEvent {
    pub actor: Option<A>,
    pub createdAt: DateTime,
    // Deprecated Notice
    pub databaseId: Option<Int>,
    pub deployment: Deployment,
    pub id: Id,
    pub pullRequest: PullRequest,
    pub ref_: Ref,
}

impl Node<A> for Deployment<A> {}
pub struct Deployment {
    // Connections
    pub statuses: DeploymentStatusConnection,
    // args after type String
    // args before type String
    // args first type Int
    // args last type Int
    // Fields
    pub commit: Option<Commit>,
    pub createdAt: DateTime
    pub creator: Option<A>,
    pub databaseId: Option<Int>,
    pub environment: Option<String>,
    pub id: Id,
    pub latestStatus: Option<DeploymentStatus>,
    pub payload: Option<String>,
    pub repository: Repository,
    pub state: Option<DeploymentState>,
}

impl Node<A> for DeploymentStatus<A> {}
pub struct DeploymentStatus {
    pub creator: Option<Actor>,
    pub deployment: Deployment,
    pub description: Option<String>,
    pub environmentUrl: Option<Uri>,
    pub id: Id,
    pub logUrl: Option<Uri>,
    pub state: DeploymentStatusState,
}

impl Node for ExternalIdentity {}
pub struct ExternalIdentity {
    pub guid: String,
    pub id: Id,
    pub organizationInvitation: Option<OrganizationInvitation>,
    pub samlIdentity: Option<ExternalIdentitySamlAttributes>,
    pub scimIdentity: Option<ExternalIdentityScimAttributes>,
    pub user: Option<User>,
}

pub struct ExternalIdentitySamlAttributes {
    pub nameId: Option<String>
}

pub struct ExternalIdentityScimAttributes {
    pub username: Option<String>
}

impl Node for Gist {}
impl Starrable for Gist {}
pub struct Gist {

}

pub struct GistComment {

}

pub struct GitActor {

}

pub struct GitHubMetadata {

}

pub struct GpgSignature {

}

pub struct HeadRefDeletedEvent {

}

pub struct HeadRefForcePushedEvent {

}

pub struct HeadRefRestoredEvent {

}

pub struct Issue {

}

pub struct IssueComment {

}

pub struct Label {

}

pub struct LabeledEvent {

}

pub struct Language {

}

pub struct License {

}

pub struct LicenseRule {

}

pub struct LockedEvent {

}

pub struct MentionedEvent {

}

pub struct MergedEvent {

}

pub struct Milestone {

}

pub struct MilestonedEvent {

}

pub struct MovedColumnsInProjectEvent {

}

pub struct Organization {

}

pub struct OrganizationIdentityProvider {

}

pub struct OrganizationInvitation {

}

pub struct PageInfo {

}

pub struct Project {

}

pub struct ProjectCard {

}

pub struct ProjectColumn {

}

pub struct ProtectedBranch {

}

pub struct PullRequest {

}

pub struct PullRequestCommit {

}

pub struct PullRequestReview {

}

pub struct PullRequestReviewComment {

}

pub struct PullRequestReviewThread {

}

pub struct PushAllowance {

}

pub struct RateLimit {

}

pub struct Reaction {

}

pub struct ReactionGroup {

}

pub struct Ref {

}

pub struct ReferencedEvent {

}

pub struct Release {

}

pub struct ReleaseAsset {

}

pub struct RemovedFromProjectEvent {

}

pub struct RenamedTitleEvent {

}

pub struct ReopenedEvent {

}

pub struct Repository {

}

pub struct RepositoryInvitation {

}

pub struct RepositoryInvitationRepository {

}

pub struct RepositoryTopic {

}

pub struct ReviewDismissalAllowance {

}

pub struct ReviewDismissedEvent {

}

pub struct ReviewRequest {

}

pub struct ReviewRequestRemovedEvent {

}

pub struct ReviewRequestedEvent {

}

pub struct SmimeSignature {

}

pub struct Status {

}

pub struct StatusContext {

}

pub struct SubscribedEvent {

}

pub struct SuggestedReviewer {

}

pub struct Tag {

}

pub struct Team {

}

pub struct Topic {

}

pub struct Tree {

}

pub struct TreeEntry {

}

pub struct UnassignedEvent {

}

pub struct UnknownSignature {

}

pub struct UnlabeledEvent {

}

pub struct UnlockedEvent {

}

pub struct UnsubscribedEvent {

}

pub struct User {

}
