//! This crate contains an implementation of the Floyd-Warshall algorithm to solve the all-pairs-shortest-paths problem in undirected graphs.

#![deny(missing_docs)]

extern crate petgraph;

#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod tests;

use std::fmt;

use petgraph::graph::NodeIndex;
use petgraph::visit::Data;
use petgraph::visit::GraphBase;
use petgraph::visit::NodeCount;
use petgraph::visit::IntoNodeIdentifiers;
use petgraph::visit::IntoEdgeReferences;
use petgraph::visit::EdgeRef;
use petgraph::visit::GraphProp;

/// This matrix is a solution to the APSP problem, calculated by the Floyd-Warshall algorithm. It contains the length of the shortest path for every pair of nodes in a given graph.
pub struct DistanceMatrix {
    m: Box<[usize]>,
    n: usize,
}

impl DistanceMatrix {
    /// Creates a new ```DistanceMatrix``` with the given dimension (n * n).
    pub fn new(n: usize) -> DistanceMatrix {
        use std::usize::MAX;
        let m = vec![MAX; n * n].into();
        DistanceMatrix { m, n }
    }

    /// This method computes the "inner index" into the ```Vec``` by using the given X-Y-coordinates into the matrix.
    fn idx(&self, mut i: usize, mut j: usize) -> usize {
        // We only fill one half of the matrix.
        if i > j {
            ::std::mem::swap(&mut i, &mut j);
        }
        assert!(i <= j);

        i + self.n * j
    }

    /// This method returns the value at the given position.
    pub fn get(&self, i: usize, j: usize) -> usize {
        let idx = self.idx(i, j);
        self.m[idx]
    }

    /// This method updates the value at the given position.
    pub fn set(&mut self, i: usize, j: usize, v: usize) {
        let idx = self.idx(i, j);
        self.m[idx] = v;
    }
}

impl fmt::Debug for DistanceMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::result::Result;

        for j in 0..self.n {
            let from = j * self.n;
            let to = j * self.n + j + 1;
            writeln!(f, "{:?}", &self.m[from..to])?
        }

        Result::Ok(())
    }
}

/// This function computes a distance matrix containing the shortest paths between every two nodes in the graph.
/// By using the Floyd-Warshall algorithm, this is computed in **O(V^3)** runtime.
pub fn floyd_warshall<G>(g: G) -> DistanceMatrix
where
    G: Data<EdgeWeight = usize>
        + GraphBase<NodeId = NodeIndex>
        + NodeCount
        + IntoNodeIdentifiers<NodeId = NodeIndex>
        + IntoEdgeReferences
        + GraphProp,
{
    // We currently only support directed graphs.
    assert!(!g.is_directed());

    let mut m = DistanceMatrix::new(g.node_count());

    // Each node has a distance of 0 to itself.
    for k in g.node_identifiers() {
        let k = k.index();
        m.set(k, k, 0);
    }

    // Update the matrix to represent the actual edges in the graph.
    for e in g.edge_references() {
        let n1 = e.source().index();
        let n2 = e.target().index();
        let w = e.weight();
        m.set(n1, n2, *w);
    }

    // k is the "intermediate" node which is currently considered.
    for k in g.node_identifiers() {
        let k = k.index();

        // For every pair (n1, n2) of two disjunct nodes in the graph check, if the path over k is shorter than the previously found one.
        for n1 in g.node_identifiers() {
            let n1 = n1.index();

            for n2 in g.node_identifiers() {
                let n2 = n2.index();

                // No need to do this for identical nodes.
                if n1 == n2 {
                    continue;
                }

                // These are the two options in this round to reach from node 1 to node 2.
                let v1 = m.get(n1, n2);
                let v2 = m.get(n1, k).saturating_add(m.get(k, n2));


                // Whichever of these is minimal, can be used to reach from node 1 to node 2.
                if v2 < v1 {
                    // Update the matrix to the minimum of these two.
                    m.set(n1, n2, v2);
                }
            }
        }
    }

    m
}