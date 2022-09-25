use slotgraph::SlotGraph;

#[test]
fn edge_iter() {
    let mut sg = SlotGraph::new();
    let n1 = sg.insert_node("n1");
    let n2 = sg.insert_node("n2");
    let e1 = sg.insert_edge(n1, n2, "e1");
    let mut edge_iter = sg.iter_edges();
    assert_eq!(edge_iter.next(), Some((e1, &"e1")));
    assert_eq!(edge_iter.next(), None);
}
