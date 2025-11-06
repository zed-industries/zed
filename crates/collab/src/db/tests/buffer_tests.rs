use super::*;
use crate::test_both_dbs;
use language::proto::{self, serialize_version};
use text::{Buffer, ReplicaId};

test_both_dbs!(
    test_channel_buffers,
    test_channel_buffers_postgres,
    test_channel_buffers_sqlite
);

async fn test_channel_buffers(db: &Arc<Database>) {
    let a_id = db
        .create_user(
            "user_a@example.com",
            None,
            false,
            NewUserParams {
                github_login: "user_a".into(),
                github_user_id: 101,
            },
        )
        .await
        .unwrap()
        .user_id;
    let b_id = db
        .create_user(
            "user_b@example.com",
            None,
            false,
            NewUserParams {
                github_login: "user_b".into(),
                github_user_id: 102,
            },
        )
        .await
        .unwrap()
        .user_id;

    // This user will not be a part of the channel
    let c_id = db
        .create_user(
            "user_c@example.com",
            None,
            false,
            NewUserParams {
                github_login: "user_c".into(),
                github_user_id: 103,
            },
        )
        .await
        .unwrap()
        .user_id;

    let owner_id = db.create_server("production").await.unwrap().0 as u32;

    let zed_id = db.create_root_channel("zed", a_id).await.unwrap();

    db.invite_channel_member(zed_id, b_id, a_id, ChannelRole::Member)
        .await
        .unwrap();

    db.respond_to_channel_invite(zed_id, b_id, true)
        .await
        .unwrap();

    let connection_id_a = ConnectionId { owner_id, id: 1 };
    let _ = db
        .join_channel_buffer(zed_id, a_id, connection_id_a)
        .await
        .unwrap();

    let mut buffer_a = Buffer::new(
        ReplicaId::new(0),
        text::BufferId::new(1).unwrap(),
        "".to_string(),
    );
    let operations = vec![
        buffer_a.edit([(0..0, "hello world")]),
        buffer_a.edit([(5..5, ", cruel")]),
        buffer_a.edit([(0..5, "goodbye")]),
        buffer_a.undo().unwrap().1,
    ];
    assert_eq!(buffer_a.text(), "hello, cruel world");

    let operations = operations
        .into_iter()
        .map(|op| proto::serialize_operation(&language::Operation::Buffer(op)))
        .collect::<Vec<_>>();

    db.update_channel_buffer(zed_id, a_id, &operations)
        .await
        .unwrap();

    let connection_id_b = ConnectionId { owner_id, id: 2 };
    let buffer_response_b = db
        .join_channel_buffer(zed_id, b_id, connection_id_b)
        .await
        .unwrap();

    let mut buffer_b = Buffer::new(
        ReplicaId::new(0),
        text::BufferId::new(1).unwrap(),
        buffer_response_b.base_text,
    );
    buffer_b.apply_ops(buffer_response_b.operations.into_iter().map(|operation| {
        let operation = proto::deserialize_operation(operation).unwrap();
        if let language::Operation::Buffer(operation) = operation {
            operation
        } else {
            unreachable!()
        }
    }));

    assert_eq!(buffer_b.text(), "hello, cruel world");

    // Ensure that C fails to open the buffer
    assert!(
        db.join_channel_buffer(zed_id, c_id, ConnectionId { owner_id, id: 3 })
            .await
            .is_err()
    );

    // Ensure that both collaborators have shown up
    assert_eq!(
        buffer_response_b.collaborators,
        &[
            rpc::proto::Collaborator {
                user_id: a_id.to_proto(),
                peer_id: Some(rpc::proto::PeerId { id: 1, owner_id }),
                replica_id: ReplicaId::FIRST_COLLAB_ID.as_u16() as u32,
                is_host: false,
                committer_name: None,
                committer_email: None,
            },
            rpc::proto::Collaborator {
                user_id: b_id.to_proto(),
                peer_id: Some(rpc::proto::PeerId { id: 2, owner_id }),
                replica_id: ReplicaId::FIRST_COLLAB_ID.as_u16() as u32 + 1,
                is_host: false,
                committer_name: None,
                committer_email: None,
            }
        ]
    );

    // Ensure that get_channel_buffer_collaborators works
    let zed_collaborats = db.get_channel_buffer_collaborators(zed_id).await.unwrap();
    assert_eq!(zed_collaborats, &[a_id, b_id]);

    let left_buffer = db
        .leave_channel_buffer(zed_id, connection_id_b)
        .await
        .unwrap();

    assert_eq!(left_buffer.connections, &[connection_id_a],);

    let cargo_id = db.create_root_channel("cargo", a_id).await.unwrap();
    let _ = db
        .join_channel_buffer(cargo_id, a_id, connection_id_a)
        .await
        .unwrap();

    db.leave_channel_buffers(connection_id_a).await.unwrap();

    let zed_collaborators = db.get_channel_buffer_collaborators(zed_id).await.unwrap();
    let cargo_collaborators = db.get_channel_buffer_collaborators(cargo_id).await.unwrap();
    assert_eq!(zed_collaborators, &[]);
    assert_eq!(cargo_collaborators, &[]);

    // When everyone has left the channel, the operations are collapsed into
    // a new base text.
    let buffer_response_b = db
        .join_channel_buffer(zed_id, b_id, connection_id_b)
        .await
        .unwrap();
    assert_eq!(buffer_response_b.base_text, "hello, cruel world");
    assert_eq!(buffer_response_b.operations, &[]);
}

test_both_dbs!(
    test_channel_buffers_last_operations,
    test_channel_buffers_last_operations_postgres,
    test_channel_buffers_last_operations_sqlite
);

async fn test_channel_buffers_last_operations(db: &Database) {
    let user_id = db
        .create_user(
            "user_a@example.com",
            None,
            false,
            NewUserParams {
                github_login: "user_a".into(),
                github_user_id: 101,
            },
        )
        .await
        .unwrap()
        .user_id;
    let observer_id = db
        .create_user(
            "user_b@example.com",
            None,
            false,
            NewUserParams {
                github_login: "user_b".into(),
                github_user_id: 102,
            },
        )
        .await
        .unwrap()
        .user_id;
    let owner_id = db.create_server("production").await.unwrap().0 as u32;
    let connection_id = ConnectionId {
        owner_id,
        id: user_id.0 as u32,
    };

    let mut buffers = Vec::new();
    let mut text_buffers = Vec::new();
    for i in 0..3 {
        let channel = db
            .create_root_channel(&format!("channel-{i}"), user_id)
            .await
            .unwrap();

        db.invite_channel_member(channel, observer_id, user_id, ChannelRole::Member)
            .await
            .unwrap();
        db.respond_to_channel_invite(channel, observer_id, true)
            .await
            .unwrap();

        let res = db
            .join_channel_buffer(channel, user_id, connection_id)
            .await
            .unwrap();

        buffers.push(
            db.transaction(|tx| async move { db.get_channel_buffer(channel, &tx).await })
                .await
                .unwrap(),
        );

        text_buffers.push(Buffer::new(
            ReplicaId::new(res.replica_id as u16),
            text::BufferId::new(1).unwrap(),
            "".to_string(),
        ));
    }

    update_buffer(
        buffers[0].channel_id,
        user_id,
        db,
        vec![
            text_buffers[0].edit([(0..0, "a")]),
            text_buffers[0].edit([(0..0, "b")]),
            text_buffers[0].edit([(0..0, "c")]),
        ],
    )
    .await;

    update_buffer(
        buffers[1].channel_id,
        user_id,
        db,
        vec![
            text_buffers[1].edit([(0..0, "d")]),
            text_buffers[1].edit([(1..1, "e")]),
            text_buffers[1].edit([(2..2, "f")]),
        ],
    )
    .await;

    // cause buffer 1's epoch to increment.
    db.leave_channel_buffer(buffers[1].channel_id, connection_id)
        .await
        .unwrap();
    db.join_channel_buffer(buffers[1].channel_id, user_id, connection_id)
        .await
        .unwrap();
    let replica_id = text_buffers[1].replica_id();
    text_buffers[1] = Buffer::new(
        replica_id,
        text::BufferId::new(1).unwrap(),
        "def".to_string(),
    );
    update_buffer(
        buffers[1].channel_id,
        user_id,
        db,
        vec![
            text_buffers[1].edit([(0..0, "g")]),
            text_buffers[1].edit([(0..0, "h")]),
        ],
    )
    .await;

    update_buffer(
        buffers[2].channel_id,
        user_id,
        db,
        vec![text_buffers[2].edit([(0..0, "i")])],
    )
    .await;

    let channels_for_user = db.get_channels_for_user(user_id).await.unwrap();

    pretty_assertions::assert_eq!(
        channels_for_user.latest_buffer_versions,
        [
            rpc::proto::ChannelBufferVersion {
                channel_id: buffers[0].channel_id.to_proto(),
                epoch: 0,
                version: serialize_version(&text_buffers[0].version())
                    .into_iter()
                    .filter(
                        |vector| vector.replica_id == text_buffers[0].replica_id().as_u16() as u32
                    )
                    .collect::<Vec<_>>(),
            },
            rpc::proto::ChannelBufferVersion {
                channel_id: buffers[1].channel_id.to_proto(),
                epoch: 1,
                version: serialize_version(&text_buffers[1].version())
                    .into_iter()
                    .filter(
                        |vector| vector.replica_id == text_buffers[1].replica_id().as_u16() as u32
                    )
                    .collect::<Vec<_>>(),
            },
            rpc::proto::ChannelBufferVersion {
                channel_id: buffers[2].channel_id.to_proto(),
                epoch: 0,
                version: serialize_version(&text_buffers[2].version())
                    .into_iter()
                    .filter(
                        |vector| vector.replica_id == text_buffers[2].replica_id().as_u16() as u32
                    )
                    .collect::<Vec<_>>(),
            },
        ]
    );
}

async fn update_buffer(
    channel_id: ChannelId,
    user_id: UserId,
    db: &Database,
    operations: Vec<text::Operation>,
) {
    let operations = operations
        .into_iter()
        .map(|op| proto::serialize_operation(&language::Operation::Buffer(op)))
        .collect::<Vec<_>>();
    db.update_channel_buffer(channel_id, user_id, &operations)
        .await
        .unwrap();
}
