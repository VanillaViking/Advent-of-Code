use std::fs;

#[derive(Debug)]
struct GraphEdge {
    to: usize,
    weight: u32,
}

fn build_graph(input: &str) -> Vec<Vec<GraphEdge>> {
    let height = input.lines().count();
    let width = input.lines().collect::<Vec<&str>>().first().unwrap().len();
    let input_str = input.replace("\n", "");

    let mut graph: Vec<Vec<GraphEdge>> = Vec::new();

    input_str.chars().enumerate().for_each(|(idx, ch)| {
        let mut edges = Vec::new();

        if let Some(i) = idx.checked_sub(1) {
            if idx % 13 != 0 {
                edges.push(GraphEdge { to: i, weight: input_str.chars().nth(i).unwrap().to_digit(10).unwrap() });
            }
        }

        if let Some(i) = idx.checked_sub(width) {
            edges.push(GraphEdge { to: i, weight: input_str.chars().nth(i).unwrap().to_digit(10).unwrap() });
        }

        if idx + 1 < input_str.len() {
            if idx + 1 % 13 != 0 {
                edges.push(GraphEdge { to: idx+1, weight: input_str.chars().nth(idx+1).unwrap().to_digit(10).unwrap() });
            }
        }

        if idx + width < input_str.len() {
            edges.push(GraphEdge { to: idx+width, weight: input_str.chars().nth(idx+width).unwrap().to_digit(10).unwrap() });
        }

        graph.push(edges)
    });

    graph
}

fn dijkstra(graph: &Vec<Vec<GraphEdge>>, start: usize, end: usize) -> Vec<usize> {
    let mut seen = vec![false; graph.len()];
    let mut dists = vec![u32::MAX; graph.len()];
    dists[start] = 0;
    let mut prev = vec![None; graph.len()];

    while has_unvisited(&seen, &dists) {
        let curr = get_lowest_unvisited(&seen, &dists).unwrap();
        seen[curr] = true;

        let adjs = &graph[curr];
        for (_idx, edge) in adjs.iter().enumerate() {
            if seen[edge.to] {
                continue;
            }

            let dist = dists[curr] + edge.weight;
            if dist < dists[edge.to] {
                dists[edge.to] = dist;
                prev[edge.to] = Some(curr);
            }
        }
    }

    let mut out = Vec::new();
    let mut curr = end;

    while let Some(p) = prev[curr] {
        out.push(curr);
        curr = p;
    }
    out.push(start);

    out.reverse();
    out
}

fn get_lowest_unvisited(seen: &Vec<bool>, dists: &Vec<u32>) -> Option<usize> {
    let mut idx: Option<usize> = None;
    let mut lowest_distance = u32::MAX;

    for (i, is_seen) in seen.iter().enumerate() {
        if is_seen.to_owned() {
            continue;
        }

        if lowest_distance > dists[i] {
            lowest_distance = dists[i];
            idx = Some(i);
        }
    }

    idx
}

fn has_unvisited(seen: &Vec<bool>, dists: &Vec<u32>) -> bool {
    seen.iter().enumerate().find(|(i, s)| s.to_owned().to_owned() == false && dists[i.to_owned()] < u32::MAX).is_some()
}

fn main() {
    let input = fs::read_to_string("sample").unwrap();
    let graph = build_graph(&input);

    dbg!(&graph.len());

    dbg!(dijkstra(&graph, 0, (13*13) -1));

}
