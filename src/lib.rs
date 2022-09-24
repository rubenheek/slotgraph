use slotmap::{DefaultKey, Key, SecondaryMap, SlotMap};
use std::slice::Iter;

#[cfg(test)]
mod tests;

/// A graph data structure based on the `SlotMap` data structure.
/// It provides no guarantees for uniqueness or cyclicity of edges.
/// It is aimed to be a simple building block for other types of graphs.
struct SlotGraph<K: Key, V> {
    nodes: SlotMap<K, V>,
    adj_in: SecondaryMap<K, Vec<K>>,
    adj_out: SecondaryMap<K, Vec<K>>,
}

impl<V> SlotGraph<DefaultKey, V> {
    /// Constructs a new, empty `SlotGraph`.
    pub fn new() -> Self {
        Self::with_key()
    }
}

impl<K: Key, V> SlotGraph<K, V> {
    /// Constructs a new, empty `SlotGraph` with a custom `SlotMap` `Key`.
    pub fn with_key() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            adj_out: SecondaryMap::new(),
            adj_in: SecondaryMap::new(),
        }
    }

    /// Returns a reference to the value corresponding to the node.
    pub fn get(&self, key: K) -> Option<&V> {
        self.nodes.get(key)
    }

    /// Returns a mutuble reference to the value of the node.
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.nodes.get_mut(key)
    }

    /// Insert a new node with the given value.
    /// Returns a key corresponding to the new node.
    pub fn insert_node(&mut self, value: V) -> K {
        let key = self.nodes.insert(value);
        self.adj_in.insert(key, Vec::new());
        self.adj_out.insert(key, Vec::new());
        key
    }

    /// Returns `true` if the graph contains a node with the given key.
    pub fn contains_node(&self, key: K) -> bool {
        self.nodes.contains_key(key)
    }

    /// Insert a new directed edge pointing from `from` to `to`.
    /// Returns an error if a node does not exist, containing the given key of that node.
    pub fn insert_edge(&mut self, from: K, to: K) -> Result<(), K> {
        let adj_out_from = self.adj_out.get_mut(from).ok_or(from)?;
        let adj_in_to = self.adj_in.get_mut(to).ok_or(to)?;
        adj_out_from.push(to);
        adj_in_to.push(from);
        Ok(())
    }

    /// Returns `true` if the graph cotnains an edge from `from` to `to`.
    /// Returns `false` if either of the nodes do not exist.
    pub fn contains_edge(&self, from: K, to: K) -> bool {
        self.adj_out
            .get(from)
            .map(|adj| adj.iter().any(|&n| n == from))
            .unwrap_or(false)
    }

    /// Remove a directed edge from 'from' to 'to', if it exists.
    /// Returns an error if a node does not exist, containing the given key of that node.
    pub fn remove_edge(&mut self, from: K, to: K) -> Result<(), K> {
        let adj_out_from = self.adj_out.get_mut(from).ok_or(from)?;
        let adj_in_to = self.adj_in.get_mut(to).ok_or(to)?;
        if let Some(idx_out) = adj_out_from.iter().position(|&node| node == to) {
            if let Some(idx_in) = adj_in_to.iter().position(|&node| node == from) {
                adj_out_from.swap_remove(idx_out);
                adj_in_to.swap_remove(idx_in);
            }
        }
        Ok(())
    }

    /// Returns an iterator over the nodes `node` points to.
    pub fn iter_in(&self, node: K) -> Option<Iter<K>> {
        self.adj_in.get(node).map(|adj| adj.iter())
    }

    /// Returns an iterator over the nodes pointing to `node`.
    pub fn iter_out(&self, node: K) -> Option<Iter<K>> {
        self.adj_out.get(node).map(|adj| adj.iter())
    }
}
