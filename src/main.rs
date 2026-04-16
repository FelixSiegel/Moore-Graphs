mod graph;

use graph::Graph;

fn main() {
    let k = 7;
    let n = k * k + 1;

    let mut graph = Graph::new(n, k);
    println!(
        "Empty graph with {} nodes for degree k={} created.",
        graph.n, graph.k
    );

    println!("Pinning nodes...");
    graph.pin_tree();
    println!("{}", graph);

    println!("Searching graph...");
    let result = graph.search();

    println!("Found graph? {}", result);
    println!("All vertices have degree k? {}", graph.check_degree());
    println!("Graph: \n\n{}", graph);
}
