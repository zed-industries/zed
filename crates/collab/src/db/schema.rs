pub mod project {
    use sea_query::Iden;

    #[derive(Iden)]
    pub enum Definition {
        #[iden = "projects"]
        Table,
        Id,
        RoomId,
        HostUserId,
        HostConnectionId,
    }
}

pub mod worktree {
    use sea_query::Iden;

    #[derive(Iden)]
    pub enum Definition {
        #[iden = "worktrees"]
        Table,
        Id,
        ProjectId,
        AbsPath,
        RootName,
        Visible,
        ScanId,
        IsComplete,
    }
}

pub mod room_participant {
    use sea_query::Iden;

    #[derive(Iden)]
    pub enum Definition {
        #[iden = "room_participants"]
        Table,
        RoomId,
        UserId,
        AnsweringConnectionId,
    }
}
