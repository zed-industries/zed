use super::*;
use crate::test_both_dbs;
use language::proto;
use text::Buffer;

test_both_dbs!(
    test_channel_buffers,
    test_channel_buffers_postgres,
    test_channel_buffers_sqlite
);

async fn test_channel_buffers(db: &Arc<Database>) {
    // Prep database test info
    let a_id = db
        .create_user(
            "user_a@example.com",
            false,
            NewUserParams {
                github_login: "user_a".into(),
                github_user_id: 101,
                invite_count: 0,
            },
        )
        .await
        .unwrap()
        .user_id;
    let b_id = db
        .create_user(
            "user_b@example.com",
            false,
            NewUserParams {
                github_login: "user_b".into(),
                github_user_id: 102,
                invite_count: 0,
            },
        )
        .await
        .unwrap()
        .user_id;

    // This user will not be a part of the channel
    let c_id = db
        .create_user(
            "user_c@example.com",
            false,
            NewUserParams {
                github_login: "user_c".into(),
                github_user_id: 102,
                invite_count: 0,
            },
        )
        .await
        .unwrap()
        .user_id;

    let owner_id = db.create_server("production").await.unwrap().0 as u32;

    let zed_id = db.create_root_channel("zed", "1", a_id).await.unwrap();

    db.invite_channel_member(zed_id, b_id, a_id, false)
        .await
        .unwrap();

    db.respond_to_channel_invite(zed_id, b_id, true)
        .await
        .unwrap();

    let connection_id_a = ConnectionId { owner_id, id: 1 };
    let buffer_response_a = db
        .join_channel_buffer(zed_id, a_id, connection_id_a)
        .await
        .unwrap();
    let buffer_id = BufferId::from_proto(buffer_response_a.buffer_id);

    let mut buffer_a = Buffer::new(0, 0, "".to_string());
    let mut operations = Vec::new();
    operations.push(buffer_a.edit([(0..0, "hello world")]));
    operations.push(buffer_a.edit([(5..5, ", cruel")]));
    operations.push(buffer_a.edit([(0..5, "goodbye")]));
    operations.push(buffer_a.undo().unwrap().1);
    assert_eq!(buffer_a.text(), "hello, cruel world");

    let operations = operations
        .into_iter()
        .map(|op| proto::serialize_operation(&language::Operation::Buffer(op)))
        .collect::<Vec<_>>();

    db.update_channel_buffer(buffer_id, &operations)
        .await
        .unwrap();

    let connection_id_b = ConnectionId { owner_id, id: 2 };
    let buffer_response_b = db
        .join_channel_buffer(zed_id, b_id, connection_id_b)
        .await
        .unwrap();

    let mut buffer_b = Buffer::new(0, 0, buffer_response_b.base_text);
    buffer_b
        .apply_ops(buffer_response_b.operations.into_iter().map(|operation| {
            let operation = proto::deserialize_operation(operation).unwrap();
            if let language::Operation::Buffer(operation) = operation {
                operation
            } else {
                unreachable!()
            }
        }))
        .unwrap();

    assert_eq!(buffer_b.text(), "hello, cruel world");

    // Ensure that C fails to open the buffer
    assert!(db
        .join_channel_buffer(zed_id, c_id, ConnectionId { owner_id, id: 3 })
        .await
        .is_err());

    //Ensure that both collaborators have shown up
    assert_eq!(
        buffer_response_b.collaborators,
        &[
            rpc::proto::Collaborator {
                user_id: a_id.to_proto(),
                peer_id: Some(rpc::proto::PeerId { id: 1, owner_id }),
                replica_id: 0,
            },
            rpc::proto::Collaborator {
                user_id: b_id.to_proto(),
                peer_id: Some(rpc::proto::PeerId { id: 2, owner_id }),
                replica_id: 1,
            }
        ]
    );

    let collaborators = db
        .leave_channel_buffer(zed_id, connection_id_b)
        .await
        .unwrap();

    assert_eq!(collaborators, &[connection_id_a],);

    db.connection_lost(connection_id_a).await.unwrap();
    // assert!()
    // Test buffer epoch incrementing?
}
