use objects::*;
use interfaces::*;

pub enum IssueOrPullRequest {
    Issue(Issue),
    PullRequest(PullRequest),
}

pub enum IssueTimelineItem<A,As,C,E,G>
{
    AssignedEvent(AssignedEvent<A,As>),
    ClosedEvent(ClosedEvent<A,C,E,G>),
    Commit(Commit<A,E,G>),
    CrossReferencedEvent(CrossReferencedEvent),
    DemilestonedEvent(DemilestonedEvent),
    IssueComment(IssueComment),
    LabeledEvent(LabeledEvent),
    LockedEvent(LockedEvent),
    MilestonedEvent(MilestonedEvent),
    ReferencedEvent(ReferencedEvent),
    RenamedTitleEvent(RenamedTitleEvent),
    ReopenedEvent(ReopenedEvent),
    SubscribedEvent(SubscribedEvent),
    UnassignedEvent(UnassignedEvent),
    UnlabeledEvent(UnlabeledEvent),
    UnlockedEvent(UnlockedEvent),
    UnsubscribedEvent(UnsubscribedEvent),
}

pub enum MilestoneItem {
    Issue(Issue),
    PullRequest(PullRequest),
}

pub enum ProjectCardItem {
    Issue(Issue),
    PullRequest(PullRequest),
}

pub enum PullRequestTimelineItem<A,As,C,E,G>
{
    AssignedEvent(AssignedEvent<A,As>),
    BaseRefForcePushedEvent(BaseRefForcePushedEvent<A,E,G>),
    ClosedEvent(ClosedEvent<A,C,E,G>),
    Commit(Commit<A,E,G>),
    CommitCommentThread(CommitCommentThread),
    CrossReferencedEvent(CrossReferencedEvent),
    DemilestonedEvent(DemilestonedEvent),
    DeployedEvent(DeployedEvent),
    HeadRefDeletedEvent(HeadRefDeletedEvent),
    HeadRefForcePushedEvent(HeadRefForcePushedEvent),
    HeadRefRestoredEvent(HeadRefRestoredEvent),
    IssueComment(IssueComment),
    LabeledEvent(LabeledEvent),
    LockedEvent(LockedEvent),
    MergedEvent(MergedEvent),
    MilestonedEvent(MilestonedEvent),
    PullRequestReview(PullRequestReview),
    PullRequestReviewComment(PullRequestReviewComment),
    PullRequestReviewThread(PullRequestReviewThread),
    ReferencedEvent(ReferencedEvent),
    RenamedTitleEvent(RenamedTitleEvent),
    ReopenedEvent(ReopenedEvent),
    ReviewDismissedEvent(ReviewDismissedEvent),
    ReviewRequestRemovedEvent(ReviewRequestRemovedEvent),
    ReviewRequestedEvent(ReviewRequestedEvent),
    SubscribedEvent(SubscribedEvent),
    UnassignedEvent(UnassignedEvent),
    UnlabeledEvent(UnlabeledEvent),
    UnlockedEvent(UnlockedEvent),
    UnsubscribedEvent(UnsubscribedEvent),
}

pub enum PushAllowanceActor {
    Team(Team),
    User(User),
}

pub enum ReferencedSubject {
    Issue(Issue),
    PullRequest(PullRequest),
}

pub enum RenamedTitleSubject {
    Issue(Issue),
    PullRequest(PullRequest),
}

pub enum ReviewDismissalAllowanceActor {
    Team(Team),
    User(User),
}

pub enum SearchResultItem {
    Issue(Issue),
    Organization(Organization),
    PullRequest(PullRequest),
    Repository(Repository),
    User(User),
}
