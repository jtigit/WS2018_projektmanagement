
pub struct Graph {
    pub nodes: Vec<usize>,
    pub edges: Vec<(usize, usize)>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self) -> usize {
        let node_id = self.nodes.len();
        self.nodes.push(node_id);
        node_id
    }

    pub fn add_edge(&mut self, edge: (usize, usize)) {
        match edge {
            (src, dst) => {
                assert!(src < self.nodes.len());
                assert!(dst < self.nodes.len());
                self.edges.push(edge);
            }
        }
    }

    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn edge_count(&self) -> usize { self.edges.len() }
}