use super::*;
use crate::test_both_dbs;
use language::proto;
use text::Buffer;

test_both_dbs!(test_buffers, test_buffers_postgres, test_buffers_sqlite);

async fn test_buffers(db: &Arc<Database>) {
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

    let zed_id = db.create_root_channel("zed", "1", a_id).await.unwrap();

    db.invite_channel_member(zed_id, b_id, a_id, false)
        .await
        .unwrap();

    db.respond_to_channel_invite(zed_id, b_id, true)
        .await
        .unwrap();

    // TODO: Join buffer
    let buffer_id = db.get_or_create_buffer_for_channel(zed_id);

    let mut buffer = Buffer::new(0, 0, "".to_string());
    let mut operations = Vec::new();
    operations.push(buffer.edit([(0..0, "hello world")]));
    operations.push(buffer.edit([(5..5, ", cruel")]));
    operations.push(buffer.edit([(0..5, "goodbye")]));
    operations.push(buffer.undo().unwrap().1);
    assert_eq!(buffer.text(), "hello, cruel world");

    let operations = operations
        .into_iter()
        .map(|op| proto::serialize_operation(&language::Operation::Buffer(op)))
        .collect::<Vec<_>>();

    db.update_buffer(buffer_id, &operations).await.unwrap();

    let buffer_data = db.open_buffer(buffer_id).await.unwrap();

    let mut buffer_2 = Buffer::new(0, 0, buffer_data.base_text);
    buffer_2
        .apply_ops(buffer_data.operations.into_iter().map(|operation| {
            let operation = proto::deserialize_operation(operation).unwrap();
            if let language::Operation::Buffer(operation) = operation {
                operation
            } else {
                unreachable!()
            }
        }))
        .unwrap();

    assert_eq!(buffer_2.text(), "hello, cruel world");
}
