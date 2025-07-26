use std::collections::VecDeque;

fn topological_sort(adj: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut indegree = vec![0; n];
    for u in 0..n {
        for &v in &adj[u] {
            indegree[v] += 1;
        }
    }
    let mut q = VecDeque::new();
    for i in 0..n {
        if indegree[i] == 0 {
            q.push_back(i);
        }
    }
    let mut order = Vec::with_capacity(n);
    while let Some(u) = q.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            indegree[v] -= 1;
            if indegree[v] == 0 {
                q.push_back(v);
            }
        }
    }
    if order.len() == n {
        Some(order)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::topological_sort;

    #[test]
    fn test_empty_graph() {
        assert_eq!(Some(vec![]), topological_sort(&vec![]));
    }

    #[test]
    fn test_linear_chain() {
        let adj = vec![
            vec![1],
            vec![2],
            vec![],
        ];
        assert_eq!(Some(vec![0, 1, 2]), topological_sort(&adj));
    }

    #[test]
    fn test_branching_graph() {
        let adj = vec![
            vec![1, 2],
            vec![],
            vec![],
        ];
        assert_eq!(Some(vec![0, 1, 2]), topological_sort(&adj));
    }

    #[test]
    fn test_disconnected_graph() {
        let adj = vec![
            vec![1],
            vec![],
            vec![],
        ];
        assert_eq!(Some(vec![0, 2, 1]), topological_sort(&adj));
    }

    #[test]
    fn test_diamond_graph() {
        let adj = vec![
            vec![1, 2],
            vec![3],
            vec![3],
            vec![],
        ];
        assert_eq!(Some(vec![0, 1, 2, 3]), topological_sort(&adj));
    }

    #[test]
    fn test_cycle_detection() {
        let adj = vec![
            vec![1],
            vec![0],
        ];
        assert_eq!(None, topological_sort(&adj));
    }
}