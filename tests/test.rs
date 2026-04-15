#[path = "../src/graph.rs"]
mod graph;

use graph::Graph;

fn has_edge(graph: &Graph, u: usize, v: usize) -> bool {
    (graph.matrix[u] >> v) & 1 == 1
}

// ── Helpers to build named test graphs ───────────────────────────────────

/// C₅ — the 5-cycle; the Moore graph for k=2, diameter=2
fn petersen_graph() -> Graph {
    let mut g = Graph::new(10, 3);
    // Outer pentagon
    for i in 0..5 {
        g.add_edge(i, (i + 1) % 5);
    }
    // Inner pentagram
    for i in 0..5 {
        g.add_edge(i + 5, ((i + 2) % 5) + 5);
    }
    // Spokes
    for i in 0..5 {
        g.add_edge(i, i + 5);
    }
    g
}

/// K₃ — complete graph on 3 vertices (triangle)
fn triangle_graph() -> Graph {
    let mut g = Graph::new(3, 2);
    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.add_edge(0, 2);
    g
}

/// C₄ — the 4-cycle
fn four_cycle_graph() -> Graph {
    let mut g = Graph::new(4, 2);
    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.add_edge(2, 3);
    g.add_edge(3, 0);
    g
}

/// C₅ — the 5-cycle (Moore graph for k=2)
fn five_cycle_graph() -> Graph {
    let mut g = Graph::new(5, 2);
    for i in 0..5 {
        g.add_edge(i, (i + 1) % 5);
    }
    g
}

/// K₄ — complete graph on 4 vertices (every vertex has degree 3)
fn complete_k4() -> Graph {
    let mut g = Graph::new(4, 3);
    for i in 0..4 {
        for j in (i + 1)..4 {
            g.add_edge(i, j);
        }
    }
    g
}

// ── Adjacency matrix display ──────────────────────────────────────────────

#[test]
fn test_display_empty_graph() {
    let g = Graph::new(5, 2);
    let output = format!("{}", g);
    println!("Empty 5-node graph:\n{}", output);
    // All adjacency rows should be empty bitboards.
    assert!(g.matrix.iter().all(|row| *row == 0));
}

#[test]
fn test_display_triangle() {
    let g = triangle_graph();
    println!("Triangle (K₃):\n{}", g);
    // Diagonal is 0, off-diagonal edges are 1
    assert!(!has_edge(&g, 0, 0));
    assert!(has_edge(&g, 0, 1));
    assert!(has_edge(&g, 1, 0));
}

#[test]
fn test_display_petersen() {
    let g = petersen_graph();
    println!("Petersen graph:\n{}", g);
}

// ── check_degree ─────────────────────────────────────────────────────────

#[test]
fn test_degree_empty_graph_fails() {
    // No edges → degree 0, but k=2 → should fail
    let g = Graph::new(5, 2);
    assert!(!g.check_degree());
}

#[test]
fn test_degree_triangle_passes() {
    let g = triangle_graph();
    assert!(g.check_degree(), "K₃: every vertex has degree 2\n{}", g);
}

#[test]
fn test_degree_five_cycle_passes() {
    let g = five_cycle_graph();
    assert!(g.check_degree(), "C₅: every vertex has degree 2\n{}", g);
}

#[test]
fn test_degree_petersen_passes() {
    let g = petersen_graph();
    assert!(
        g.check_degree(),
        "Petersen: every vertex has degree 3\n{}",
        g
    );
}

#[test]
fn test_degree_partial_graph_fails() {
    let mut g = Graph::new(4, 2);
    g.add_edge(0, 1); // only vertex 0 and 1 have degree 1; others have 0
    assert!(!g.check_degree());
}

#[test]
fn test_degree_after_remove_edge_fails() {
    let mut g = triangle_graph();
    g.remove_edge(0, 1);
    assert!(
        !g.check_degree(),
        "After removing an edge degree check must fail\n{}",
        g
    );
}

// ── check_triangles (3-cycles) ────────────────────────────────────────────

#[test]
fn test_triangle_graph_has_triangle() {
    let g = triangle_graph();
    println!("K₃ (must have triangle):\n{}", g);
    assert!(g.check_triangles(), "K₃ contains a 3-cycle");
}

#[test]
fn test_five_cycle_no_triangle() {
    let g = five_cycle_graph();
    println!("C₅ (must NOT have triangle):\n{}", g);
    assert!(!g.check_triangles(), "C₅ is triangle-free");
}

#[test]
fn test_petersen_no_triangle() {
    let g = petersen_graph();
    println!("Petersen (must NOT have triangle):\n{}", g);
    assert!(!g.check_triangles(), "Petersen graph is triangle-free");
}

#[test]
fn test_complete_k4_has_triangle() {
    let g = complete_k4();
    println!("K₄ (must have triangle):\n{}", g);
    assert!(g.check_triangles(), "K₄ contains many 3-cycles");
}

#[test]
fn test_four_cycle_no_triangle() {
    let g = four_cycle_graph();
    println!("C₄ (must NOT have triangle):\n{}", g);
    assert!(!g.check_triangles(), "C₄ is triangle-free");
}

// ── 4-cycle detection ─────────────────────────────────────────────────────
// Moore graphs require girth ≥ 5, so both 3-cycles AND 4-cycles must be absent.
// You will need a check_four_cycles (or generic check_girth) method.

#[test]
fn test_four_cycle_graph_has_4_cycle() {
    let g = four_cycle_graph();
    println!("C₄ (must have 4-cycle):\n{}", g);
    assert!(g.check_four_cycles(), "C₄ obviously contains a 4-cycle");
}

#[test]
fn test_five_cycle_no_4_cycle() {
    let g = five_cycle_graph();
    println!("C₅ (must NOT have 4-cycle):\n{}", g);
    assert!(!g.check_four_cycles(), "C₅ has no 4-cycle");
}

#[test]
fn test_petersen_no_4_cycle() {
    let g = petersen_graph();
    println!("Petersen (must NOT have 4-cycle):\n{}", g);
    assert!(!g.check_four_cycles(), "Petersen graph has girth 5");
}

#[test]
fn test_complete_k4_has_4_cycle() {
    let g = complete_k4();
    println!("K₄ (must have 4-cycle):\n{}", g);
    assert!(g.check_four_cycles(), "K₄ contains 4-cycles");
}

// ── Moore graph structural smoke-test ─────────────────────────────────────

#[test]
fn test_petersen_is_moore_candidate() {
    // The Petersen graph is THE Moore graph for k=3, diameter=2
    // It must: be 3-regular, triangle-free, 4-cycle-free
    let g = petersen_graph();
    println!("Petersen graph (Moore candidate):\n{}", g);
    assert_eq!(g.n, 10);
    assert_eq!(g.k, 3);
    assert!(g.check_degree());
    assert!(!g.check_triangles());
    assert!(!g.check_four_cycles());
}

#[test]
fn test_five_cycle_is_moore_candidate() {
    // C₅ is the Moore graph for k=2, diameter=2
    let g = five_cycle_graph();
    println!("C₅ (Moore candidate k=2):\n{}", g);
    assert_eq!(g.n, 5);
    assert_eq!(g.k, 2);
    assert!(g.check_degree());
    assert!(!g.check_triangles());
    assert!(!g.check_four_cycles());
}
