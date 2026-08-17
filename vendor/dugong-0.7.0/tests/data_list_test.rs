use dugong::data::list::{List, Node};
use serde_json::json;

#[test]
fn list_dequeue_returns_none_with_an_empty_list() {
    let list: List<serde_json::Value> = List::new();
    assert!(list.dequeue().is_none());
}

#[test]
fn list_dequeue_unlinks_and_returns_the_first_entry() {
    let list: List<serde_json::Value> = List::new();
    let obj = Node::new(json!({}));
    list.enqueue(obj.clone());
    assert!(std::rc::Rc::ptr_eq(&list.dequeue().unwrap(), &obj));
}

#[test]
fn list_dequeue_unlinks_and_returns_multiple_entries_in_fifo_order() {
    let list: List<serde_json::Value> = List::new();
    let obj1 = Node::new(json!({ "id": 1 }));
    let obj2 = Node::new(json!({ "id": 2 }));
    list.enqueue(obj1.clone());
    list.enqueue(obj2.clone());

    assert!(std::rc::Rc::ptr_eq(&list.dequeue().unwrap(), &obj1));
    assert!(std::rc::Rc::ptr_eq(&list.dequeue().unwrap(), &obj2));
    assert!(list.dequeue().is_none());
}

#[test]
fn list_dequeue_unlinks_and_relinks_an_entry_if_it_is_re_enqueued() {
    let list: List<serde_json::Value> = List::new();
    let obj1 = Node::new(json!({ "id": 1 }));
    let obj2 = Node::new(json!({ "id": 2 }));
    list.enqueue(obj1.clone());
    list.enqueue(obj2.clone());
    list.enqueue(obj1.clone());

    assert!(std::rc::Rc::ptr_eq(&list.dequeue().unwrap(), &obj2));
    assert!(std::rc::Rc::ptr_eq(&list.dequeue().unwrap(), &obj1));
    assert!(list.dequeue().is_none());
}

#[test]
fn list_dequeue_unlinks_and_relinks_an_entry_if_it_is_enqueued_on_another_list() {
    let list: List<serde_json::Value> = List::new();
    let list2: List<serde_json::Value> = List::new();
    let obj = Node::new(json!({ "id": 1 }));
    list.enqueue(obj.clone());
    list2.enqueue(obj.clone());

    assert!(list.dequeue().is_none());
    assert!(std::rc::Rc::ptr_eq(&list2.dequeue().unwrap(), &obj));
}

#[test]
fn list_can_return_a_string_representation() {
    let list: List<serde_json::Value> = List::new();
    list.enqueue(Node::new(json!({ "entry": 1 })));
    list.enqueue(Node::new(json!({ "entry": 2 })));

    assert_eq!(list.to_string(), r#"[{"entry":1}, {"entry":2}]"#);
}
