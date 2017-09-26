use scalars::*;
use enums::*;
use objects::*;

pub trait Actor {
    type AvatarUrl = Uri; // How to deal with size arg?
    type Login = String;
    type ResourcePath = Uri;
    type Url = Uri;
}

pub trait Assignable {}

pub trait Closable {
    type Closed = Boolean;
}

pub trait Comment<A,E> {
    type Author = Option<A>;
    type AuthorAssociation = CommentAuthorAssociation;
    type Body = String;
    type BodyHtml = Html;
    type CreatedAt = DateTime;
    type CreatedViaEmail = Boolean;
    type Editor = Option<E>;
    type Id = Id;
    type LastEditAt = Option<DateTime>;
    type PublishedAt = Option<DateTime>;
    type UpdatedAt = Option<DateTime>;
    type ViewerDidAuthor = Boolean;
}

pub trait Deletable {
    type ViewerCanDelete = Boolean;
}

pub trait GitObject {
    type AbbreviatedOid = String;
    type CommitResourcePath = Uri;
    type CommitUrl = Uri;
    type Id = Id;
    type Oid = GitObjectId;
    type Repository = Repository;
}

pub trait GitSignature {
    type Email = String;
    type IsValid = Boolean;
    type Payload = String;
    type Signature = String;
    type Signer = User;
    type State = GitSignatureState;
}

pub trait Labelable {}

pub trait Lockable {
    type Locked = Boolean;
}

pub trait Node {
    type Id = Id;
}

pub trait ProjectOwner {
    type Id = Id;
    type Project = Option<Project>; // arg number type Int!
    type ProjectResourcePath = Uri;
    type ViewerCanCreateProjects = Boolean;
}

pub trait Reactable {
    // Note deprecated soon
    type DatabaseId = Option<Int>;
    type Id = Id;
    type ReactionGroup = Vec<ReactionGroup>;
    type ViewerCanReact = Boolean;
}

pub trait RepositoryInfo<O>
    where O: RepositoryOwner
{
    type CreatedAt = DateTime;
    type Description = Option<String>;
    type DescriptionHtml = Html;
    type HasIssuesEnabled = Boolean;
    type HasWikiEnabled = Boolean;
    type HomepageUrl = Option<Uri>;
    type IsFork = Boolean;
    type IsLocked = Boolean;
    type IsMirror = Boolean;
    type IsPrivate = Boolean;
    // Deprecation notice
    type License = Option<String>;
    type LicenseInfo = Option<License>;
    type LockReason = Option<RepositoryLockReason>;
    type MirrorUrl = Option<Uri>;
    type Name = String;
    type NameWithOwner = String;
    type Owner = O;
    type PushedAt = Option<DateTime>;
    type ResourcePath = Uri;
    type ShortDescriptionHtml = Html;
    // Deprecation notice
    type UpdatedAt = DateTime;
    type Url = Uri;
}

pub trait RepositoryNode {
    type Repository = Repository;
}

pub trait RepositoryOwner {
    type AvatarUrl = Uri;
    type Id = Id;
    type Login = String;
    type Repository = Option<Repository>; // arg name type String!
    type ResourcePath = Uri;
    type Url = Uri;
}

pub trait Starrable {
    type Id = Id;
    type ViewewHasStarred = Boolean;
}

pub trait Subscribable {
    type Id = Id;
    type ViewerCanSubscribe = Boolean;
    type ViewerSubscription = SubscriptionState;
}

pub trait UniformResourceLocatable {
    type ResourcePath = Uri;
    type Url = Uri;
}

pub trait Updatable {
    type ViewerCanUpdate = Boolean;
}

pub trait UpdatableComment {
    type ViewerCannotUpdateReasons = Vec<CommentCannotUpdateReason>;
}
