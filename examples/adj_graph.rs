use slotgraph::{
    slotgraph::{EdgeKey, NodeKey},
    SlotGraph,
};
use slotmap::{DefaultKey, Key, SecondaryMap};

/// A graph data structure based on the [`SlotGraph`] data structure.
/// It supports efficient lookup of adjacency information by storing node and edge keys adjacent to a node.
struct AdjGraph<K: Key, N, E> {
    sg: SlotGraph<K, N, E>,
    adj_in: SecondaryMap<NodeKey<K>, Vec<(EdgeKey<K>, NodeKey<K>)>>,
    adj_out: SecondaryMap<NodeKey<K>, Vec<(EdgeKey<K>, NodeKey<K>)>>,
}

impl<K: Key, N, E> AdjGraph<K, N, E> {
    /// Constructs a new, empty [`AdjGraph`] with a custom [`SlotMap`] key.
    fn with_key() -> Self {
        Self {
            sg: SlotGraph::with_key(),
            adj_in: SecondaryMap::new(),
            adj_out: SecondaryMap::new(),
        }
    }

    /// Insert a new node with the value into the slot graph.
    ///
    /// # Panics
    ///
    /// Panics if the number of nodes  in the graph equals 2³² - 2.
    fn insert_node(&mut self, value: N) -> NodeKey<K> {
        let key = self.sg.insert_node(value);
        self.adj_in.insert(key, Vec::new());
        self.adj_out.insert(key, Vec::new());
        key
    }

    /// Insert a new edge with the given value into the slot graph.
    ///
    /// # Panics
    ///
    /// Panics if the number of edges in the graph equals 2³² - 2.
    fn insert_edge(&mut self, from: NodeKey<K>, to: NodeKey<K>, value: E) -> EdgeKey<K> {
        let key = self.sg.insert_edge(from, to, value);
        self.adj_out.get_mut(from).unwrap().push((key, to));
        self.adj_in.get_mut(to).unwrap().push((key, from));
        key
    }

    /// An iterator of the edge keys pointing from the given node key.
    fn out_edges(&self, from: NodeKey<K>) -> Option<impl Iterator<Item = EdgeKey<K>> + '_> {
        let adj_out = self.adj_out.get(from)?;
        let ek_iter = adj_out.iter().map(|&(ek, _nk)| ek);
        Some(ek_iter)
    }

    /// An iterator of the node keys pointed to from the given node key.
    fn out_nodes(&self, from: NodeKey<K>) -> Option<impl Iterator<Item = NodeKey<K>> + '_> {
        let adj_out = self.adj_out.get(from)?;
        let nk_iter = adj_out.iter().map(|&(_ek, nk)| nk);
        Some(nk_iter)
    }

    /// An iterator of the edge keys pointing to the given node key.
    fn in_edges(&self, to: NodeKey<K>) -> Option<impl Iterator<Item = EdgeKey<K>> + '_> {
        let adj_in = self.adj_in.get(to)?;
        let ek_iter = adj_in.iter().map(|&(ek, _nk)| ek);
        Some(ek_iter)
    }

    /// An iterator of the node keys pointing to the given node key.
    fn in_nodes(&self, to: NodeKey<K>) -> Option<impl Iterator<Item = NodeKey<K>> + '_> {
        let adj_in = self.adj_in.get(to)?;
        let nk_iter = adj_in.iter().map(|&(_ek, nk)| nk);
        Some(nk_iter)
    }
}

fn main() {
    let mut ag = AdjGraph::<DefaultKey, &str, &str>::with_key();
    let n1 = ag.insert_node("n1");
    let n2 = ag.insert_node("n2");
    let e1 = ag.insert_edge(n1, n2, "e1");

    assert_eq!(ag.out_edges(n1).unwrap().next(), Some(e1));
    assert_eq!(ag.out_edges(n2).unwrap().next(), None);

    assert_eq!(ag.out_nodes(n1).unwrap().next(), Some(n2));
    assert_eq!(ag.out_nodes(n2).unwrap().next(), None);

    assert_eq!(ag.in_edges(n1).unwrap().next(), None);
    assert_eq!(ag.in_edges(n2).unwrap().next(), Some(e1));

    assert_eq!(ag.in_nodes(n1).unwrap().next(), None);
    assert_eq!(ag.in_nodes(n2).unwrap().next(), Some(n1));
}
