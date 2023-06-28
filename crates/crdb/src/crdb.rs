struct RepoId {
    replica: ReplicaId,
    op_id: OpId,
}

pub struct Db {
    repositories: TreeMap<RepoId, Repo>,
}

pub struct Repo {}
