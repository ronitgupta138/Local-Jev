use local_jev::vector::VectorContextStore;

#[test]
fn test_vector_cosine_search() {
    let mut store = VectorContextStore::new();

    // Doc 1: Search codebase
    store.add("doc_grep", vec![1.0, 0.0, 0.0, 0.0], "grep tool");
    // Doc 2: Commit git
    store.add("doc_commit", vec![0.0, 1.0, 0.0, 0.0], "git commit");
    // Doc 3: Deploy pipeline
    store.add("doc_deploy", vec![0.0, 0.0, 1.0, 0.0], "deploy tool");

    // Query close to doc 1
    let query = vec![0.9, 0.1, 0.0, 0.0];
    let results = store.search(&query, 2);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0.id, "doc_grep");
    assert!(results[0].1 > 0.9);
}
