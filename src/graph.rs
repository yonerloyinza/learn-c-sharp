use std::collections::{HashMap, HashSet, VecDeque};

pub struct Graph {
    edges: HashMap<String, Vec<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_undirected_edge(&mut self, from: &str, to: &str) {
        self.edges
            .entry(from.to_owned())
            .or_default()
            .push(to.to_owned());
        self.edges
            .entry(to.to_owned())
            .or_default()
            .push(from.to_owned());
    }

    pub fn bfs(&self, start: &str) -> Vec<String> {
        if !self.edges.contains_key(start) {
            return Vec::new();
        }

        let mut visited = HashSet::from([start.to_owned()]);
        let mut queue = VecDeque::from([start.to_owned()]);
        let mut order = Vec::new();

        while let Some(node) = queue.pop_front() {
            order.push(node.clone());
            if let Some(neighbors) = self.edges.get(&node) {
                for neighbor in neighbors {
                    if visited.insert(neighbor.clone()) {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        order
    }

    pub fn dfs(&self, start: &str) -> Vec<String> {
        if !self.edges.contains_key(start) {
            return Vec::new();
        }

        let mut visited = HashSet::new();
        let mut stack = vec![start.to_owned()];
        let mut order = Vec::new();

        while let Some(node) = stack.pop() {
            if !visited.insert(node.clone()) {
                continue;
            }
            order.push(node.clone());

            if let Some(neighbors) = self.edges.get(&node) {
                stack.extend(neighbors.iter().rev().cloned());
            }
        }

        order
    }

    pub fn demo() -> Self {
        let mut graph = Self::new();
        graph.add_undirected_edge("A", "B");
        graph.add_undirected_edge("A", "C");
        graph.add_undirected_edge("B", "D");
        graph.add_undirected_edge("C", "D");
        graph.add_undirected_edge("D", "E");
        graph
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn traverses_each_reachable_node_once() {
        let graph = Graph::demo();

        assert_eq!(
            graph.bfs("A"),
            vec![
                "A".to_owned(),
                "B".to_owned(),
                "C".to_owned(),
                "D".to_owned(),
                "E".to_owned()
            ]
        );
        assert_eq!(
            graph.dfs("A"),
            vec![
                "A".to_owned(),
                "B".to_owned(),
                "D".to_owned(),
                "C".to_owned(),
                "E".to_owned()
            ]
        );
        assert!(graph.bfs("missing").is_empty());
    }
}
