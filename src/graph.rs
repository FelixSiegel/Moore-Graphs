use std::fmt;

pub struct Graph {
    pub n: usize,               // Verticies/Nodes
    pub k: usize,               // Degree/Edges per Vertex
    pub matrix: Vec<Vec<bool>>, // 1 = edge, 0 = no edge
}

#[allow(dead_code)]

impl Graph {
    pub fn new(n: usize, k: usize) -> Self {
        Graph {
            n,
            k,
            matrix: vec![vec![false; n]; n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.matrix[u][v] = true;
        self.matrix[v][u] = true;
    }

    pub fn remove_edge(&mut self, u: usize, v: usize) {
        self.matrix[u][v] = false;
        self.matrix[v][u] = false;
    }

    // check for each vertex in the adjacency matrix if it has exactly k edges
    pub fn check_degree(&self) -> bool {
        self.matrix
            .iter()
            .all(|row| (row.iter().filter(|&&x| x).count()) == self.k)
    }

    // probably completly ass and inefficient BUT it works!
    pub fn check_triangles(&self) -> bool {
        for u in 0..self.n {
            // find all verticies connected to u
            let neighbors: Vec<usize> = self.matrix[u]
                .iter()
                .enumerate()
                .filter_map(|(idx, &connected)| if connected { Some(idx) } else { None })
                .collect();

            // check if any of the neighbord are connected together, so they build a triangle
            // e.g. u -> v and u -> w, then if v -> w it builds an triangle
            for i in 0..neighbors.len() {
                for j in (i + 1)..neighbors.len() {
                    let v = neighbors[i];
                    let w = neighbors[j];

                    if self.matrix[v][w] {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn check_four_cycles(&self) -> bool {
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
                write!(f, "{:2}", if self.matrix[i][j] { 1 } else { 0 })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
