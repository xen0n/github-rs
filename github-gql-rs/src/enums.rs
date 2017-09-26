pub enum CommentAuthorAssociation {
    /// Author has been invited to collaborate on the repository.
    Collaborator,
    /// Author has previously committed to the repository.
    Contributor,
    /// Author has not previously committed to GitHub.
    FirstTimer,
    /// Author has not previously committed to the repository.
    FirstTimeContributor,
    /// Author is a member of the organization that owns the repository.
    Member,
    /// Author has no association with the repository.
    None,
    /// Author is the owner of the repository.
    Owner,
}

pub enum CommentCannotUpdateReason {
    /// You must be the author or have write access to this repository to update this comment.
    InsufficientAccess,
    /// Unable to create comment because issue is locked.
    Locked,
    /// You must be logged in to update this comment.
    LoginRequired,
    /// Repository is under maintenance.
    Maintenance,
    /// At least one email address must be verified to update this comment.
    VerifiedEmailRequired,
}

pub enum DefaultRepositoryPermissionField {
    /// Members have read, write, and admin access to org repos by default
    Admin,
    /// Members have read access to org repos by default
    Read,
    /// Members have read and write access to org repos by default
    Write,
}

pub enum DeploymentState {
    /// The pending deployment was not updated after 30 minutes.
    Abandoned,
    /// The deployment is currently active.
    Active,
    /// An inactive transient deployment.
    Destroyed,
    /// The deployment experienced an error.
    Error,
    /// The deployment has failed.
    Failure,
    /// The deployment is inactive.
    Inactive,
    /// The deployment is pending.
    Pending,
}

pub enum DeploymentStatusState {
    /// The deployment experienced an error.
    Error,
    /// The deployment has failed.
    Failure,
    /// The deployment is inactive.
    Inactive,
    /// The deployment is pending.
    Pending,
    /// The deployment was successful.
    Success,
}

pub enum GistOrderField {
    /// Order gists by creation time
    CreatedAt,
    /// Order gists by push time
    PushedAt,
    /// Order gists by update time
    UpdatedAt,
}

pub enum GistPrivacy {
    /// Gists that are public and secret
    All,
    /// Public
    Public,
    /// Secret
    Secret,
}

pub enum GitSignatureState {
    /// Invalid email used for signing.
    BadEmail,
    /// Signing key expired.
    ExpiredKey,
    /// Internal error - the GPG verification service misbehaved.
    GpgverifyError,
    /// Internal error - the GPG verification service is unavailable at the moment.
    GpgverifyUnavailable,
    /// Invalid signature.
    Invalid,
    /// Malformed signature.
    MalformedSig,
    /// The usage flags for the key that signed this don't allow signing.
    NotSigningKey,
    /// Email used for signing not known to GitHub.
    NoUser,
    /// Key used for signing not known to GitHub.
    UnknownKey,
    /// Unknown signature type.
    UnknownSigType,
    /// Unsigned.
    Unsigned,
    /// Email used for signing unverified on GitHub.
    UnverifiedEmail,
    /// Valid signature and verified by GitHub.
    Valid,
}

pub enum IssueOrderField {
    /// Order issues by comment count
    Comments,
    /// Order issues by creation time
    CreatedAt,
    /// Order issues by update time
    UpdatedAt,
}

pub enum IssuePubSubTopic {
    /// The channel ID for marking an issue as read.
    Markasread,
    /// The channel ID for observing issue updates.
    Updated,
}

pub enum IssueState {
    /// An issue that has been closed
    Closed,
    /// An issue that is still open
    Open,
}

pub enum LanguageOrderField {
    /// Order languages by the size of all files containing the language
    Size,
}

pub enum MergeableState {
    /// The pull request cannot be merged due to merge conflicts.
    Conflicting,
    /// The pull request can be merged.
    Mergeable,
    /// The mergeability of the pull request is still being calculated.
    Unknown,
}

pub enum MilestoneState {
    /// A milestone that has been closed.
    Closed,
    /// A milestone that is still open.
    Open,
}

pub enum OrderDirection {
    /// Specifies an ascending order for a given orderBy argument.
    Asc,
    /// Specifies a descending order for a given orderBy argument.
    Desc,
}

pub enum OrganizationInvitationRole {
    /// The user is invited to be an admin of the organization.
    Admin,
    /// The user is invited to be a billing manager of the organization.
    BillingManager,
    /// The user is invited to be a direct member of the organization.
    DirectMember,
    /// The user's previous role will be reinstated.
    Reinstate,
}

pub enum OrganizationInvitationType {
    /// The invitation was to an email address.
    Email,
    /// The invitation was to an existing user.
    User,
}

pub enum ProjectCardState {
    /// The card has content only.
    ContentOnly,
    /// The card has a note only.
    NoteOnly,
    /// The card is redacted.
    Redacted,
}

pub enum ProjectOrderField {
    /// Order projects by creation time
    CreatedAt,
    /// Order projects by name
    Name,
    /// Order projects by update time
    UpdatedAt,
}

pub enum ProjectState {
    /// The project is closed.
    Closed,
    /// The project is open.
    Open,
}

pub enum PullRequestPubSubTopic {
    /// The channel ID for observing head ref updates.
    HeadRef,
    /// The channel ID for marking an pull request as read.
    Markasread,
    /// The channel ID for observing pull request updates.
    Updated,
}

pub enum PullRequestReviewEvent {
    /// Submit feedback and approve merging these changes.
    Approve,
    /// Submit general feedback without explicit approval.
    Comment,
    /// Dismiss review so it now longer effects merging.
    Dismiss,
    /// Submit feedback that must be addressed before merging.
    RequestChanges,
}

pub enum PullRequestReviewState {
    /// A review allowing the pull request to merge.
    Approved,
    /// A review blocking the pull request from merging.
    ChangesRequested,
    /// An informational review.
    Commented,
    /// A review that has been dismissed.
    Dismissed,
    /// A review that has not yet been submitted.
    Pending,
}

pub enum PullRequestState {
    /// A pull request that has been closed without being merged.
    Closed,
    /// A pull request that has been closed by being merged.
    Merged,
    /// A pull request that is still open.
    Open,
}

pub enum ReactionContent {
    /// Represents the 😕 emoji.
    Confused,
    /// Represents the ❤️ emoji.
    Heart,
    /// Represents the 🎉 emoji.
    Hooray,
    /// Represents the 😄 emoji.
    Laugh,
    /// Represents the 👎 emoji.
    ThumbsDown,
    /// Represents the 👍 emoji.
    ThumbsUp,
}

pub enum ReactionOrderField {
    /// Allows ordering a list of reactions by when they were created.
    CreatedAt,
}

pub enum RepositoryAffiliation {
    /// Repositories that the user has been added to as a collaborator.
    Collaborator,
    /// Repositories that the user has access to through being a member of an
    /// organization. This includes every repository on every team that the user,
    /// is on.
    OrganizationMember,
    /// Repositories that are owned by the authenticated user.
    Owner,
}

pub enum RepositoryCollaboratorAffiliation {
    /// All collaborators of the repository.
    All,
    /// All outside collaborators of an organization-owned repository.
    Outside,
}

pub enum RepositoryLockReason {
    /// The repository is locked due to a billing related reason.
    Billing,
    /// The repository is locked due to a migration.
    Migrating,
    /// The repository is locked due to a move.
    Moving,
    /// The repository is locked due to a rename.
    Rename,
}

pub enum RepositoryOrderField {
    /// Order repositories by creation time
    CreatedAt,
    /// Order repositories by name
    Name,
    /// Order repositories by push time
    PushedAt,
    /// Order repositories by number of stargazers
    Stargazers,
    /// Order repositories by update time
    UpdatedAt,
}

pub enum RepositoryPermission {
    /// Can read, clone, push, and add collaborators
    Admin,
    /// Can read and clone
    Read,
    /// Can read, clone and push
    Write,
}

pub enum RepositoryPrivacy {
    /// Private
    Private,
    /// Public
    Public,
}

pub enum SearchType {
    /// Returns results matching issues in repositories.
    Issue,
    /// Returns results matching repositories.
    Repository,
    /// Returns results matching users on GitHub.
    User,
}

pub enum StarOrderField {
    /// Allows ordering a list of stars by when they were created.
    StarredAt,
}

pub enum StatusState {
    /// Status is errored.
    Error,
    /// Status is expected.
    Expected,
    /// Status is failing.
    Failure,
    /// Status is pending.
    Pending,
    /// Status is successful.
    Success,
}

pub enum SubscriptionState {
    /// The User is never notified.
    Ignored,
    /// The User is notified of all conversations.
    Subscribed,
    /// The User is only notified when particpating or @mentioned.
    Unsubscribed,
}

pub enum TeamMemberRole {
    /// A team maintainer has permission to add and remove team members.
    Maintainer,
    /// A team member has no administrative permissions on the team.
    Member,
}

pub enum TeamMembershipType {
    /// Includes immediate and child team members for the team.
    All,
    /// Includes only child team members for the team.
    ChildTeam,
    /// Includes only immediate members of the team.
    Immediate,
}

pub enum TeamOrderField {
    /// Allows ordering a list of teams by name.
    Name,
}

pub enum TeamPrivacy {
    /// A secret team can only be seen by its members.
    Secret,
    /// A visible team can be seen and @mentioned by every member of the organization.
    Visible,
}

pub enum TeamRepositoryOrderField {
    /// Order repositories by creation time
    CreatedAt,
    /// Order repositories by name
    Name,
    /// Order repositories by permission
    Permission,
    /// Order repositories by push time
    PushedAt,
    /// Order repositories by number of stargazers
    Stargazers,
    /// Order repositories by update time
    UpdatedAt,
}

pub enum TeamRole {
    /// User has admin rights on the team.
    Admin,
    /// User is a member of the team.
    Member,
}

pub enum TopicSuggestionDeclineReason {
    /// The suggested topic is not relevant to the repository.
    NotRelevant,
    /// The viewer does not like the suggested topic.
    PersonalPreference,
    /// The suggested topic is too general for the repository.
    TooGeneral,
    /// The suggested topic is too specific for the repository
    /// (e.g. #ruby-on-rails-version-4-2-1).
    TooSpecific
}
