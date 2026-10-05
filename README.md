# Moore-Graphs

A fast search and verification tool in Rust for [Moore graphs](https://en.wikipedia.org/wiki/Moore_graph) of diameter 2 (graphs with degree $k$, $n = k^2 + 1$ vertices, and girth $\ge 5$).

According to the Hoffman–Singleton theorem, Moore graphs of diameter 2 can only exist for degrees $k \in \{2, 3, 7, 57\}$. This project uses bitboards, canonical symmetry breaking, and MRV backtracking to search for valid graphs or prove non-existence in milliseconds.

## Implemented techniques

- **[64-bit Bitboard Adjacency Representation](https://en.wikipedia.org/wiki/Adjacency_list#Data_structures)**: Each vertex's neighborhood and 2-hop reachability are tracked using bitboards (`u64`).
- **Forbidden Neighborhood Masking**: Checks triangle-free and 4-cycle-free conditions via bitwise operations in $O(1)$.
- **Group Matching Invariant**: Enforces that leaves across pinned subtrees form perfect matchings with each other.
- **Canonical Symmetry Pinning**: Fixes isomorphic leaf permutations up front to drastically reduce the search space.
- **[MRV (Minimum Remaining Values) Heuristic](https://www.geeksforgeeks.org/artificial-intelligence/explain-the-concept-of-backtracking-search-and-its-role-in-finding-solutions-to-csps/)**: Branches on the most constrained vertex/group pair and fails fast on dead ends.
- **Spectral Feasibility Pre-Check**: Dynamically verifies whether $A^2 + A - (k-1)I = J$ has integer eigenvalue multiplicities before searching.

## Usage

### Run the search

Pass the degree $k$ as a command-line argument (defaults to $k=7$):

```bash
# Find the Hoffman-Singleton graph (k=7, n=50)
cargo run --release -- 7

# Find the Petersen graph (k=3, n=10)
cargo run --release -- 3

# Prove non-existence of k=5 (n=26) or k=6 (n=37)
cargo run --release -- 5
cargo run --release -- 6
```

### Run tests

```bash
cargo test --release -- --nocapture
```

## Benchmarks

Measurements taken on release builds:

| Degree $k$ | Vertices $n = k^2 + 1$ | Exists? | Search Time               | Description                       |
| :--------- | :--------------------- | :------ | :------------------------ | :-------------------------------- |
| $k = 2$    | $5$                    | Yes     | $< 1\ \mu\text{s}$        | 5-cycle ($C_5$)                   |
| $k = 3$    | $10$                   | Yes     | $\approx 1\ \mu\text{s}$  | Petersen graph                    |
| $k = 4$    | $17$                   | No      | $\approx 130\ \text{ns}$  | Spectral rejection (non-existent) |
| $k = 5$    | $26$                   | No      | $\approx 140\ \text{ns}$  | Spectral rejection (non-existent) |
| $k = 6$    | $37$                   | No      | $\approx 150\ \text{ns}$  | Spectral rejection (non-existent) |
| $k = 7$    | $50$                   | Yes     | $\approx 90\ \mu\text{s}$ | Hoffman–Singleton graph           |

## Constraints & Higher Graphs

The current bitboard representation stores each vertex adjacency row as a single `u64`, supporting up to **$n \le 64$ nodes** ($k \le 7$).

Inputs where $n > 64$ (such as $k=57$, $n=3250$) are rejected up front. To search for $k=57$, a wide bitset implementation (e.g. `[u64; 51]`) would be required.
