// undirected graph
pub struct UGraph(usize, Vec<Vec<usize>>);

impl UGraph {
    pub fn new_from_edges(size: usize, edges: Vec<(usize, usize)>) -> UGraph {
        let mut nodes = vec![vec![]; size];
        for (x, y) in edges {
            nodes[x].push(y);
            nodes[y].push(x);
        }

        UGraph(size, nodes)
    }

    pub fn to_grouped(self) -> Vec<Vec<usize>> {
        let UGraph(size, node) = self;

        let mut groups = vec![];
        let mut visited = vec![false; size];

        for i in 0..size {
            if visited[i] {
                continue;
            }

            let mut current = vec![];
            current.push(i);

            let mut to_visit = node[i].clone();
            while let Some(p) = to_visit.pop() {
                if visited[p] {
                    continue;
                }

                current.push(p);
                visited[p] = true;

                let mut n = node[p].clone();
                to_visit.append(&mut n);
            }

            groups.push(current);
        }

        groups
    }
}
