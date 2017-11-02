//! This crate contains an implementation of the Floyd-Warshall algorithm to solve the all-pairs-shortest-paths problem in undirected graphs.

#![deny(missing_docs)]
#![feature(conservative_impl_trait)]

extern crate petgraph;

#[cfg(test)]
extern crate rand;

#[cfg(test)]
#[macro_use]
extern crate text_io;

#[cfg(test)]
mod tests;

mod matrices;
pub use matrices::*;

use petgraph::graph::NodeIndex;
use petgraph::visit::NodeRef;
use petgraph::visit::Data;
use petgraph::visit::GraphBase;
use petgraph::visit::NodeCount;
use petgraph::visit::IntoNodeIdentifiers;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::IntoEdgeReferences;
use petgraph::visit::EdgeRef;
use petgraph::visit::GraphProp;

/// This function computes a distance matrix containing the shortest paths between every two nodes in the graph.
/// By using the Floyd-Warshall algorithm, this is computed in **O(V^(3))** runtime.
pub fn floyd_warshall<G>(g: G) -> PathMatrix<G::NodeWeight>
where
    G: Data
        + GraphBase<NodeId = NodeIndex>
        + NodeCount
        + IntoNodeIdentifiers<NodeId = NodeIndex>
        + IntoNodeReferences
        + IntoEdgeReferences
        + GraphProp,
    G::NodeWeight: Clone,
    G::EdgeWeight: Clone + Into<usize>,
{
    // We currently only support directed graphs.
    assert!(!g.is_directed());

    let mut m = PathMatrix::new(g.node_count());

    // Each node has a distance of 0 to itself.
    // Note, that this sets the distance of every node to itself to 0, due to the matrix representation.
    m.set_path_len(0, 0, 0);

    // Update the matrix to represent the actual edges in the graph.
    for e in g.edge_references() {
        let n1 = e.source().index();
        let n2 = e.target().index();
        let w: G::EdgeWeight = e.weight().clone();
        let w: usize = w.into();
        m.set_path_len(n1, n2, w);
    }

    // k is the "intermediate" node which is currently considered.
    for k in g.node_references() {
        let kw = k.weight();
        let k = k.id().index();

        // For every pair (n1, n2) of two disjunct nodes in the graph check, if the path over k is shorter than the previously found one.
        for n1 in g.node_identifiers() {
            let n1 = n1.index();

            for n2 in g.node_identifiers() {
                let n2 = n2.index();

                // No need to do this for identical nodes.
                if n1 == n2 {
                    continue;
                }

                // No need to do this for both triangles in the matrix.
                if n1 > n2 {
                    continue;
                }

                // No need to do this for k == n1 or k == n2
                if n1 == k || n2 == k {
                    continue;
                }

                // These are the two options in this round to reach from node 1 to node 2:
                // - v1, which is (if it exists) the saved path from n1 to n2, which is eiter a direct edge or a path using any intermediate nodes less than k.
                let mut v1 = None;
                if m.does_path_exist(n1, n2) {
                    v1 = Some(m.get_path_len(n1, n2));
                }

                // - v2, which is the path from node 1 to node k to node 2 (if such a path exists, which means, that k is reachable from n1 and n2 is reachable from k).
                let v2_exists = m.does_path_exist(n1, k);
                let v2_exists = v2_exists && m.does_path_exist(k, n2);

                let mut v2 = None;
                if v2_exists {
                    let part1 = m.get_path_len(n1, k);
                    let part2 = m.get_path_len(k, n2);

                    // .saturating_add is a relict of a time, when a path was usize::MAX as a sign for "there is no path here".
                    // But as any other .add doesn't make any more sense, it will stay.
                    v2 = Some(part1.saturating_add(part2));
                }

                // Whichever of these is minimal, can be used to reach from node 1 to node 2.
                if v2.is_some() && (v1.is_none() || v2.unwrap() < v1.unwrap()) {
                    let v2 = v2.unwrap();

                    // Update the matrix to the minimum of these two.
                    m.set_path_len(n1, n2, v2);

                    // TODO: reuse vector here.
                    let mut v: Vec<G::NodeWeight> = Vec::new();

                    // Reverse path, if n1 < k or k < n2 not fulfilled:
                    if n1 <= k {
                        v.extend(m.get_path_iter(n1, k).cloned());
                    } else {
                        v.extend(m.get_path_iter(n1, k).rev().cloned());
                    }

                    // Push k in the middle of the path here.
                    v.push(kw.clone());

                    if k <= n2 {
                        v.extend(m.get_path_iter(k, n2).cloned());
                    } else {
                        v.extend(m.get_path_iter(k, n2).rev().cloned());
                    }

                    // Save the path as new optimal path from node 1 to node 2.
                    m.get_path_mut(n1, n2).set_vector(v);
                }
            }
        }
    }

    m
}