use crate::SlotGraph;

#[test]
fn simple() {
    let mut sg = SlotGraph::new();
    let n1 = sg.insert_node(());
    let n2 = sg.insert_node(());
    sg.insert_edge(n1, n2).unwrap();
    assert!(sg.iter_in(n1).any(|&n| n == n2));
}
