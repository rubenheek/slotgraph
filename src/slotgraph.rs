use slotmap::{DefaultKey, Key, SlotMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeKey<K: Key>(K);

struct Node<N> {
    value: N,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgeKey<K: Key>(K);

struct Edge<K: Key, E> {
    from: NodeKey<K>,
    to: NodeKey<K>,
    value: E,
}

/// A graph data structure based on the [`SlotMap`] data structure.
pub struct SlotGraph<K: Key, N, E> {
    nodes: SlotMap<K, Node<N>>,
    edges: SlotMap<K, Edge<K, E>>,
}

impl<N, E> Default for SlotGraph<DefaultKey, N, E> {
    fn default() -> Self {
        Self::with_key()
    }
}

impl<N, E> SlotGraph<DefaultKey, N, E> {
    /// Constructs a new, empty [`SlotGraph`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl<K: Key, N, E> SlotGraph<K, N, E> {
    /// Constructs a new, empty [`SlotGraph`] with a custom [`SlotMap`] key.
    pub fn with_key() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            edges: SlotMap::with_key(),
        }
    }

    /// Insert a new node with the value into the slot graph.
    ///
    /// # Panics
    ///
    /// Panics if the number of nodes in the graph equals 2³² - 2.
    pub fn insert_node(&mut self, value: N) -> NodeKey<K> {
        NodeKey(self.nodes.insert(Node { value }))
    }

    /// Removes a node key from the slot graph, returning the value at the given key if it was not previously removed.
    pub fn remove_node(&mut self, key: NodeKey<K>) -> Option<N> {
        self.nodes.remove(key.0).map(|n| n.value)
    }

    /// Returns a reference to the value corresponding to the node key.
    pub fn get_node(&self, key: NodeKey<K>) -> Option<&N> {
        self.nodes.get(key.0).map(|n| &n.value)
    }

    /// Returns a mutable reference to the value corresponding to the node key.
    pub fn get_node_mut(&mut self, key: NodeKey<K>) -> Option<&mut N> {
        self.nodes.get_mut(key.0).map(|n| &mut n.value)
    }

    pub fn node_len(&self) -> usize {
        self.nodes.len()
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = (NodeKey<K>, &N)> {
        self.nodes.iter().map(|(k, n)| (NodeKey(k), &n.value))
    }

    pub fn iter_nodes_mut(&mut self) -> impl Iterator<Item = (NodeKey<K>, &mut N)> {
        self.nodes
            .iter_mut()
            .map(|(k, n)| (NodeKey(k), &mut n.value))
    }

    pub fn into_node_iter(self) -> impl Iterator<Item = (NodeKey<K>, N)> {
        self.nodes.into_iter().map(|(k, n)| (NodeKey(k), n.value))
    }

    /// Insert a new edge with the given value into the slot graph.
    ///
    /// # Panics
    ///
    /// Panics if the number of edges in the graph equals 2³² - 2.
    pub fn insert_edge(&mut self, from: NodeKey<K>, to: NodeKey<K>, value: E) -> EdgeKey<K> {
        EdgeKey(self.edges.insert(Edge { from, to, value }))
    }

    /// Removes an edge key from the slot graph, returning the value at the given key if it was not previously removed.
    pub fn remove_edge(&mut self, key: EdgeKey<K>) -> Option<E> {
        self.edges.remove(key.0).map(|e| e.value)
    }

    /// Returns a reference to the value corresponding to the edge key.
    pub fn get_edge(&self, key: EdgeKey<K>) -> Option<&E> {
        self.edges.get(key.0).map(|e| &e.value)
    }

    /// Returns a mutable reference to the value corresponding to the edge key.
    pub fn get_edge_mut(&mut self, key: EdgeKey<K>) -> Option<&mut E> {
        self.edges.get_mut(key.0).map(|e| &mut e.value)
    }

    pub fn edge_len(&self) -> usize {
        self.edges.len()
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (EdgeKey<K>, &E)> {
        self.edges.iter().map(|(k, n)| (EdgeKey(k), &n.value))
    }

    pub fn iter_edges_mut(&mut self) -> impl Iterator<Item = (EdgeKey<K>, &mut E)> {
        self.edges
            .iter_mut()
            .map(|(k, e)| (EdgeKey(k), &mut e.value))
    }

    pub fn into_edge_iter(self) -> impl Iterator<Item = (EdgeKey<K>, E)> {
        self.edges.into_iter().map(|(k, e)| (EdgeKey(k), e.value))
    }

    pub fn get_edge_nodes(&self, key: EdgeKey<K>) -> Option<(NodeKey<K>, NodeKey<K>)> {
        self.edges.get(key.0).map(|e| (e.from, e.to))
    }

    pub fn iter_edge_nodes(
        &self,
    ) -> impl Iterator<Item = (EdgeKey<K>, (NodeKey<K>, NodeKey<K>))> + '_ {
        self.edges.iter().map(|(k, e)| (EdgeKey(k), (e.from, e.to)))
    }
}

// TODO: provide trait to auto-implement methods?
// TODO: implement graphs using other slotmap types?
