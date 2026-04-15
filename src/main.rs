mod graph;

use graph::Graph;

fn main() {
    let k = 2;
    let n = k * k + 1;

    let graph = Graph::new(n, k);
    println!(
        "Empty graph with {} nodes for degree k={} created.",
        graph.n, graph.k
    );

    println!("All vertices have degree k? {}", graph.check_degree());
}
