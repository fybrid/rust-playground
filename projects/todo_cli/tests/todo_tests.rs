use todo_cli::TodoList;

#[test]
fn adds_and_completes_items() {
    let mut list = TodoList::default();
    let id = list.add("learn ownership");

    assert_eq!(list.pending().len(), 1);
    assert!(list.complete(id));
    assert!(list.pending().is_empty());
}

#[test]
fn serializes_and_deserializes_list() {
    let mut list = TodoList::default();
    list.add("learn serde");

    let json = list.to_json().expect("serialize");
    let restored = TodoList::from_json(&json).expect("deserialize");

    assert_eq!(restored, list);
}
