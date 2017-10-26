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
use matrices::*;

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
/// By using the Floyd-Warshall algorithm, this is computed in **O(V^3)** runtime.
pub fn floyd_warshall<G>(g: G) -> PathMatrix
where
    G: Data<EdgeWeight = usize, NodeWeight = usize>
        + GraphBase<NodeId = NodeIndex>
        + NodeCount
        + IntoNodeIdentifiers<NodeId = NodeIndex>
        + IntoNodeReferences
        + IntoEdgeReferences
        + GraphProp,
{
    // We currently only support directed graphs.
    assert!(!g.is_directed());

    let mut m = PathMatrix::new(g.node_count());

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

                // These are the two options in this round to reach from node 1 to node 2.
                let v1 = m.get(n1, n2);
                let v2 = m.get(n1, k).saturating_add(m.get(k, n2));


                // Whichever of these is minimal, can be used to reach from node 1 to node 2.
                if v2 < v1 {
                    // Update the matrix to the minimum of these two.
                    m.set(n1, n2, v2);

                    let mut v: Vec<usize> = Vec::new();

                    if n1 <= k {
                        v.extend(m.get_path_iter(n1, k));
                    } else {
                        v.extend(m.get_path_iter(n1, k).rev());
                    }
                    v.push(*kw);
                    if k <= n2 {
                        v.extend(m.get_path_iter(k, n2));
                    } else {
                        v.extend(m.get_path_iter(k, n2).rev());
                    }

                    m.get_path_mut(n1, n2).set_vector(v);
                }
            }
        }
    }

    m
}