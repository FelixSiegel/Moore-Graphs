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
            needs_edges: (1u64 << n) - 1,
        }
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

    pub fn search(&mut self) -> bool {
        // Final stop condition (graph is full and valid)
        if self.check_degree() && !self.check_triangles() && !self.check_four_cycles() {
            return true;
        }

        let u = self.needs_edges.trailing_zeros() as usize;
        let node = self.matrix[u];

        // find a potential partner node
        for v in 0..self.n {
            // If same node or already connected or v is already full degree => skip
            if u == v || (node & (1 << v)) != 0 || self.needs_edges & (1 << v) == 0 {
                continue;
            }

            if self.connectable(u, v) {
                self.add_edge(u, v);

                if self.search() {
                    return true;
                }

                // not valid so backtrack
                self.remove_edge(u, v);
            }
        }

        // if no possible connection found -> current u cant reach full degree, so path is invalid
        return false;
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
