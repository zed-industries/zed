use super::*;
use crate::test_both_dbs;
use language::proto;
use text::Buffer;

test_both_dbs!(test_buffers, test_buffers_postgres, test_buffers_sqlite);

async fn test_buffers(db: &Arc<Database>) {
    let buffer_id = db.create_buffer().await.unwrap();

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

    let buffer_data = db.get_buffer(buffer_id).await.unwrap();

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
