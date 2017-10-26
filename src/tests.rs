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

#[test]
fn test_random() {
    use petgraph::Graph;
    use rand;
    use rand::Rng;

    let mut graph = Graph::new_undirected();
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();

    for i in 0..10 {
        vec.push(graph.add_node(i));
    }

    for v1 in &vec {
        for v2 in &vec {
            if v1 != v2 && rng.next_f32() < 0.1 {
                let w = (rng.next_u64() as usize) % 100;
                graph.add_edge(*v1, *v2, w);
            }
        }
    }

    let m = floyd_warshall(&graph);
    println!("{:?}", m);

    // use petgraph::dot::Dot;
    // use std::fs::File;
    // use std::io::prelude::*;

    // let mut file = File::create("random.dot").unwrap();
    // let b = format!("{:?}", Dot::new(&graph));
    // let b = b.as_bytes();
    // file.write_all(b);
}