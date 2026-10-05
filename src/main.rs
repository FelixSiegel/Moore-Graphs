mod graph;

use graph::Graph;
use std::time::Instant;

fn main() {
    let k = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(7);
    let n = k * k + 1;

    let mut graph = Graph::new(n, k);
    println!(
        "Empty graph with {} nodes for degree k={} created.",
        graph.n, graph.k
    );

    println!("Pinning nodes...");
    graph.pin_tree();
    println!("{}", graph);

    println!("Searching graph for k={} (n={})...", k, n);
    let start = Instant::now();
    let result = graph.search();
    let duration = start.elapsed();

    println!("Found graph? {} (took {:?})", result, duration);
    println!("All vertices have degree k? {}", graph.check_degree());
    println!(
        "Girth >= 5 (no triangles or 4-cycles)? {}",
        !graph.check_triangles() && !graph.check_four_cycles()
    );
    if result {
        println!("Graph: \n\n{}", graph);
    }
}
