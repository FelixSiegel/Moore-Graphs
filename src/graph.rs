use std::fmt;

pub struct Graph {
    pub n: usize,         // Verticies/Nodes
    pub k: usize,         // Degree/Edges per Vertex
    pub matrix: Vec<u64>, // Adjacency list with bitboards: 1 = edge, 0 = no edge
}

#[allow(dead_code)]
impl Graph {
    pub fn new(n: usize, k: usize) -> Self {
        Graph {
            n,
            k,
            matrix: vec![0u64; n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.matrix[u] |= 1 << v;
        self.matrix[v] |= 1 << u;
    }

    pub fn remove_edge(&mut self, u: usize, v: usize) {
        self.matrix[u] &= !(1 << v);
        self.matrix[v] &= !(1 << u);
    }

    // check for each vertex in the adjacency matrix if it has exactly k edges
    pub fn check_degree(&self) -> bool {
        self.matrix
            .iter()
            .all(|row| row.count_ones() as usize == self.k)
    }

    // probably completly ass and inefficient BUT it works!
    pub fn check_triangles(&self) -> bool {
        for u in 0..self.matrix.len() {
            for v in (u + 1)..self.matrix.len() {
                // check if u and v are connected
                if (self.matrix[u] >> v) & 1 == 1 {
                    // check if they share a common neighbor w, so that u -> v -> w -> u build a triangle
                    if self.matrix[u] & self.matrix[v] > 0 {
                        return true;
                    }
                }
            }
        }
        false
    }

    // u -> v and
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
