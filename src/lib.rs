extern crate petgraph;

use petgraph::graph::NodeIndex;
use petgraph::visit::Data;
use petgraph::visit::GraphBase;
use petgraph::visit::NodeCount;
use petgraph::visit::IntoNodeIdentifiers;
use petgraph::visit::IntoEdgeReferences;
use petgraph::visit::EdgeRef;
use petgraph::visit::GraphProp;

#[derive(Debug)]
pub struct DistanceMatrix {
    m: Box<[usize]>,
    n: usize,
}

impl DistanceMatrix {
    pub fn new(n: usize) -> DistanceMatrix {
        use std::usize::MAX;
        let m = vec![MAX; n * n].into();
        DistanceMatrix { m, n }
    }

    fn idx(&self, mut i: usize, mut j: usize) -> usize {
        if i > j {
            ::std::mem::swap(&mut i, &mut j);
        }
        assert!(i <= j);
        i * self.n + j
    }

    pub fn get(&self, i: usize, j: usize) -> usize {
        let idx = self.idx(i, j);
        self.m[idx]
    }

    pub fn set(&mut self, i: usize, j: usize, v: usize) {
        let idx = self.idx(i, j);
        self.m[idx] = v;
    }
}

pub fn floyd_warshall<G>(g: G) -> DistanceMatrix
where
    G: Data<EdgeWeight = usize>
        + GraphBase<NodeId = NodeIndex>
        + NodeCount
        + IntoNodeIdentifiers<NodeId = NodeIndex>
        + IntoEdgeReferences
        + GraphProp,
{
    assert!(!g.is_directed());

    let mut m = DistanceMatrix::new(g.node_count());

    for k in g.node_identifiers() {
        let k = k.index();
        m.set(k, k, 0);
    }

    for e in g.edge_references() {
        let n1 = e.source().index();
        let n2 = e.target().index();
        let w = e.weight();
        m.set(n1, n2, *w);
    }

    for k in g.node_identifiers() {
        let k = k.index();
        for n1 in g.node_identifiers() {
            let n1 = n1.index();
            for n2 in g.node_identifiers() {
                let n2 = n2.index();
                let v1 = m.get(n1, n2);
                let v2 = m.get(n1, k).saturating_add(m.get(k, n2));
                if v2 < v1 {
                    m.set(n1, n2, v2);
                }
            }
        }
    }

    m
}

#[cfg(test)]
mod tests {
    use super::floyd_warshall;

    #[test]
    fn test_no_intermediate() {
        use petgraph::Graph;
        let mut graph = Graph::new_undirected();

        let a = graph.add_node(0);
        let b = graph.add_node(1);
        let c = graph.add_node(2);
        let d = graph.add_node(3);

        graph.extend_with_edges(
            &[
                (a, b, 1),
                (a, c, 1),
                (a, d, 1),
                (b, c, 1),
                (b, d, 1),
                (c, d, 1),
            ],
        );

        let m = floyd_warshall(&graph);
        println!("{:?}", m);

        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    assert_eq!(m.get(i, j), 0);
                } else {
                    assert_eq!(m.get(i, j), 1);
                }
            }
        }
    }

    #[test]
    fn test_intermediate() {
        use petgraph::Graph;
        let mut graph = Graph::new_undirected();

        let a = graph.add_node(0);
        let b = graph.add_node(1);
        let c = graph.add_node(2);

        graph.extend_with_edges(&[(a, b, 1), (b, c, 1), (a, c, 3)]);

        let m = floyd_warshall(&graph);
        println!("{:?}", m);

        assert_eq!(m.get(0, 0), 0);
        assert_eq!(m.get(1, 1), 0);
        assert_eq!(m.get(2, 2), 0);

        assert_eq!(m.get(0, 1), 1);
        assert_eq!(m.get(1, 2), 1);
        assert_eq!(m.get(0, 2), 2);
    }
}