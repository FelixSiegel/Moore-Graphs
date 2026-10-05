use std::fmt;

pub struct Graph {
    pub n: usize,         // Verticies/Nodes
    pub k: usize,         // Degree/Edges per Vertex
    pub matrix: Vec<u64>, // Adjacency list with bitboards: 1 = edge, 0 = no edge
    needs_edges: u64,     // track which nodes require edges: 1 = missing edges, 0 = full degree
}

#[allow(dead_code)]
impl Graph {
    pub fn new(n: usize, k: usize) -> Self {
        Graph {
            n,
            k,
            matrix: vec![0u64; n],
            needs_edges: if n >= 64 { u64::MAX } else { (1u64 << n) - 1 },
        }
    }

    // Checks whether a degree `k` is spectrally feasible for a Moore graph of diameter 2
    // from first principles.
    //
    // Any Moore graph of diameter 2 must satisfy the matrix equation:
    //   A² + A - (k - 1)I = J
    //
    // This requires the non-trivial eigenvalues to have integer multiplicities:
    //   m₁,₂ = (k² ± k(k - 2) / √(4k - 3)) / 2
    pub fn is_spectrally_feasible(k: usize) -> bool {
        if k < 2 {
            return false;
        }
        if k == 2 {
            return true;
        }
        let disc = 4 * k - 3;
        let s = (disc as f64).sqrt().round() as usize;
        if s * s != disc {
            return false; // √(4k - 3) must be an integer
        }
        if (k * (k - 2)) % s != 0 {
            return false; // eigenvalue trace difference must be an integer
        }
        let diff = (k * (k - 2)) / s;
        (k * k + diff) % 2 == 0 // multiplicity m₁ must be an integer
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.matrix[u] |= 1 << v;
        self.matrix[v] |= 1 << u;

        // check degree of both and update need_edges bitboard accordingly
        if self.degree(self.matrix[u]) == self.k {
            self.needs_edges &= !(1 << u)
        }
        if self.degree(self.matrix[v]) == self.k {
            self.needs_edges &= !(1 << v)
        }
    }

    pub fn remove_edge(&mut self, u: usize, v: usize) {
        self.matrix[u] &= !(1 << v);
        self.matrix[v] &= !(1 << u);
        self.needs_edges |= 1 << v | 1 << u;
    }

    pub fn degree(&self, node: u64) -> usize {
        node.count_ones() as usize
    }

    // check for each vertex in the adjacency list if it has exactly k edges
    pub fn check_degree(&self) -> bool {
        self.needs_edges == 0
    }

    pub fn check_triangles(&self) -> bool {
        for u in 0..self.matrix.len() {
            for v in (u + 1)..self.matrix.len() {
                // check if u and v are connected
                if (self.matrix[u] & (1 << v)) != 0 {
                    // check if they share a common neighbor w, so that u -> v -> w -> u build a triangle
                    if self.matrix[u] & self.matrix[v] > 0 {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn check_four_cycles(&self) -> bool {
        for u in 0..self.matrix.len() {
            for v in (u + 1)..self.matrix.len() {
                // if u and v are opposite (so they arent connected directly), but if
                // they share 2 same neighbors they build a 4-cylce together
                if (self.matrix[u] & self.matrix[v]).count_ones() == 2 {
                    return true;
                }
            }
        }
        false
    }

    // we dont need to check the entire graph for triangles etc everytime if we just check a new conenction before adding it
    pub fn connectable(&self, u: usize, v: usize) -> bool {
        // Triangle check: if u and v share same neighbor -> triangle
        if (self.matrix[u] & self.matrix[v]) != 0 {
            return false;
        }

        // Cube check: defers slighlty from the general function, because u and v arent opposites, but should be connected instead
        // Instead we check if any of their neighbors are already connected
        let mut u_neighbors = self.matrix[u];
        while u_neighbors != 0 {
            let n = u_neighbors.trailing_zeros() as usize;

            if (self.matrix[v] & self.matrix[n]) != 0 {
                return false;
            }
            u_neighbors &= u_neighbors - 1;
        }
        true
    }

    // to much to explain in this code comment, I'll (hopefully write a blog post about this whole topic and explain it there)
    pub fn pin_tree(&mut self) {
        // Start pinning (connecting) the first k nodes to vertex 0
        for i in 1..=self.k {
            self.add_edge(0, i);
        }
        // Pinning the remaining nodes to the remaining k-1 branches of the level 1 nodes
        let mut next = self.k + 1;
        for i in 1..=self.k {
            for _ in 0..(self.k - 1) {
                self.add_edge(i, next);
                next += 1;
            }
        }
    }

    // breaks automorphisms / symmetries among the branches and leaves of the pinned tree:
    // 1. Connects the first leaf of branch 0 to the first leaf of every other branch (1..k).
    // 2. Connects the remaining leaves of branch 0 identically to the leaves of branch 1.
    pub fn pin_symmetries(&mut self) {
        if self.k < 2 {
            return;
        }
        let k = self.k;
        let leaf = move |g: usize, m: usize| -> usize { (k + 1) + g * (k - 1) + m };

        // Symmetry 1: first leaf of group 0 connects to first leaf of all groups 1..k
        for g in 1..self.k {
            let u = leaf(0, 0);
            let v = leaf(g, 0);
            if (self.matrix[u] & (1 << v)) == 0 {
                self.add_edge(u, v);
            }
        }

        // Symmetry 2: leaves 1..k-2 of group 0 match identically to group 1
        for m in 1..(self.k - 1) {
            let u = leaf(0, m);
            let v = leaf(1, m);
            if (self.matrix[u] & (1 << v)) == 0 {
                self.add_edge(u, v);
            }
        }
    }

    // computes the bitmask of all vertices that cannot be connected to `u`
    // without violating girth >= 5 (i.e. all vertices at distance 1 or 2 from `u`).
    #[inline]
    pub fn forbidden_mask(&self, u: usize) -> u64 {
        let mut forbidden = self.matrix[u];
        let mut nbrs = self.matrix[u];
        while nbrs != 0 {
            let w = nbrs.trailing_zeros() as usize;
            forbidden |= self.matrix[w];
            nbrs &= nbrs - 1;
        }
        forbidden
    }

    pub fn search(&mut self) -> bool {
        // spectral feasibility pre-check, eliminating impossible k
        if !Self::is_spectrally_feasible(self.k) {
            return false;
        }

        // if empty graph, pin tree
        if self.matrix[0] == 0 {
            self.pin_tree();
        }
        // break symmetries on leaves
        self.pin_symmetries();

        self.search_recursive()
    }

    fn search_recursive(&mut self) -> bool {
        if self.check_degree() {
            return !self.check_triangles() && !self.check_four_cycles();
        }

        let leaf_start = self.k + 1;
        let leaf_count = self.k - 1;

        let mut min_cands = usize::MAX;
        let mut best_u = usize::MAX;
        let mut best_gv = usize::MAX;

        // MRV (Minimum Remaining Values): find leaf u and target group g_v with the fewest candidates
        for g_u in 0..self.k {
            let u_start = leaf_start + g_u * leaf_count;
            for u_offset in 0..leaf_count {
                let u = u_start + u_offset;
                if (self.needs_edges & (1u64 << u)) == 0 {
                    continue;
                }

                let forbidden = self.forbidden_mask(u);

                for g_v in 0..self.k {
                    if g_v == g_u {
                        continue;
                    }

                    let v_start = leaf_start + g_v * leaf_count;
                    let g_v_mask = ((1u64 << leaf_count) - 1) << v_start;

                    // in case u already connects to group g_v we can skip
                    if (self.matrix[u] & g_v_mask) != 0 {
                        continue;
                    }

                    // count candidate partner vertices in g_v
                    let mut count = 0;
                    for v_offset in 0..leaf_count {
                        let v = v_start + v_offset;
                        if (self.needs_edges & (1u64 << v)) != 0
                            && (forbidden & self.matrix[v]) == 0
                        {
                            count += 1;
                        }
                    }

                    // Fail-first principle: if any group has 0 valid candidates for u it's dead end
                    if count == 0 {
                        return false;
                    }

                    if count < min_cands {
                        min_cands = count;
                        best_u = u;
                        best_gv = g_v;
                    }
                }
            }
        }

        if best_u == usize::MAX {
            return self.check_degree();
        }

        // branch on candidates in best_gv
        let v_start = leaf_start + best_gv * leaf_count;
        let forbidden = self.forbidden_mask(best_u);

        for v_offset in 0..leaf_count {
            let v = v_start + v_offset;
            if (self.needs_edges & (1u64 << v)) != 0 && (forbidden & self.matrix[v]) == 0 {
                self.add_edge(best_u, v);

                if self.search_recursive() {
                    return true;
                }

                self.remove_edge(best_u, v);
            }
        }

        false
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Header row: column indices
        write!(f, "    ")?;
        for j in 0..self.n {
            write!(f, "{:2}", j)?;
        }
        writeln!(f)?;
        write!(f, "   +")?;
        for _ in 0..self.n {
            write!(f, "--")?;
        }
        writeln!(f)?;
        // Data rows
        for i in 0..self.n {
            write!(f, "{:2} |", i)?;
            for j in 0..self.n {
                write!(
                    f,
                    "{:2}",
                    if (self.matrix[i] >> j) & 1 == 1 { 1 } else { 0 }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
