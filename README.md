# floyd-warshall

This is a rust crate which aims to solve the all-pairs-shortest-paths (APSP) problem efficiently. It uses [petgraph](https://crates.io/crates/petgraph) for graph storage and the Floyd-Warshall algorithm to solve the given problem in **O(V^(3))** runtime.

For examples, please have a look at the test cases. It consists of two simple tests (one fully connected graph and one graph, where it's shorter to use an intermediate node) and a random graph test, to manually verify the shortest paths for larger, random graphs.

The ultimate goal is to use the algorithm by [Thorup (1999)](https://dl.acm.org/citation.cfm?id=316548) to solve the same problem in **O(VE)** runtime.

Contributions are welcome!

## TODO-List

- Use mocking
- More unit tests
- cleaner API
- more efficient path saving
- include [algorithm by Thorup](https://dl.acm.org/citation.cfm?id=316548)

# License

This work is licensed under terms of the MIT License. See LICENSE.
