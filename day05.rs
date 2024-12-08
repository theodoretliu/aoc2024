use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut pipe_pairs = Vec::new();
    let mut number_arrays = Vec::new();
    let mut reading_pairs = true;

    // Read lines from stdin
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        // Skip empty lines
        if line.trim().is_empty() {
            reading_pairs = false;
            continue;
        }

        if reading_pairs {
            // Handle pipe separated pairs
            if let Some((left, right)) = line.split_once('|') {
                if let (Ok(num1), Ok(num2)) = (left.parse::<i32>(), right.parse::<i32>()) {
                    pipe_pairs.push((num1, num2));
                }
            }
        } else {
            // Handle comma separated arrays
            let numbers: Vec<i32> = line
                .split(',')
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            if !numbers.is_empty() {
                number_arrays.push(numbers);
            }
        }
    }

    let mut sum = 0;

    let mut second_sum = 0;

    for array in &number_arrays {
        let filtered_pairs: Vec<_> = pipe_pairs
            .iter()
            .filter(|(a, b)| array.contains(a) && array.contains(b))
            .collect();

        // println!("{:?}", array);
        // println!("{:?}", filtered_pairs);

        let graph = create_graph(&filtered_pairs);

        let topological_order = topological_sort(&graph);
        // println!("{:?}", topological_order);

        if topological_order.iter().eq(array.iter()) {
            sum += array[array.len() / 2];
        } else {
            second_sum += topological_order[array.len() / 2];
        }
    }

    println!("{} {}", sum, second_sum);
}

fn create_graph(pairs: &Vec<&(i32, i32)>) -> HashMap<i32, Vec<i32>> {
    let mut graph = HashMap::new();

    for (from, to) in pairs.iter() {
        graph.entry(*from).or_insert(Vec::new()).push(*to);
    }

    return graph;
}

fn topological_sort(graph: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut permanent_marks = std::collections::HashSet::new();
    let mut temporary_marks = std::collections::HashSet::new();
    let mut sorted_nodes = Vec::new();

    // Find all nodes (both source and destination)
    let mut all_nodes = std::collections::HashSet::new();
    for (from, neighbors) in graph.iter() {
        all_nodes.insert(*from);
        for to in neighbors {
            all_nodes.insert(*to);
        }
    }

    fn visit(
        node: i32,
        graph: &HashMap<i32, Vec<i32>>,
        permanent_marks: &mut HashSet<i32>,
        temporary_marks: &mut HashSet<i32>,
        sorted_nodes: &mut Vec<i32>,
    ) {
        if permanent_marks.contains(&node) {
            return;
        }
        if temporary_marks.contains(&node) {
            panic!("Graph contains at least one cycle");
        }

        temporary_marks.insert(node);

        // Visit all neighbors
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                visit(
                    neighbor,
                    graph,
                    permanent_marks,
                    temporary_marks,
                    sorted_nodes,
                );
            }
        }

        temporary_marks.remove(&node);
        permanent_marks.insert(node);
        sorted_nodes.insert(0, node);
    }

    // Visit each unmarked node
    let mut nodes: Vec<i32> = all_nodes.into_iter().collect();
    nodes.sort(); // For deterministic ordering
    for node in nodes {
        if !permanent_marks.contains(&node) {
            visit(
                node,
                graph,
                &mut permanent_marks,
                &mut temporary_marks,
                &mut sorted_nodes,
            );
        }
    }

    sorted_nodes
}
