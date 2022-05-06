use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use petgraph::graphmap::DiGraphMap;
use indexmap::IndexSet;
use indexmap::indexset;

// use petgraph::visit::{IntoNeighborsDirected, NodeCount};
use petgraph::Direction::Outgoing;

use std::collections::hash_map::RandomState;
use std::collections::HashMap;

// use std::{
//     hash::Hash,
//     iter::{from_fn, FromIterator},
// };

/// Finds shortest path between source and target and an optional path length limit.
/// ----------
///
/// :param List[Tuple[(uint, uint)...] edges: A list of triplets [(a,b, w) ...] representing a directed edge from a to b with weight w.
///
/// :param uint start:  The source node
///
/// :param uint goal: The target node
///

#[pyfunction]
fn simple_paths(
    edges: Vec<(usize,usize)>,
    from: usize,
    to: usize
) -> usize
{
    let mut acc = 0;
    let graph = DiGraphMap::<usize, usize>::from_edges(&edges);
    // let mut visited: IndexMap<usize, (), RandomState> = IndexMap::from_iter(Some(from));
    let mut visited: IndexSet<usize, RandomState> = indexset!{from};
    let mut stack = vec![graph.neighbors_directed(from, Outgoing)];


    while let Some(children) = stack.last_mut() {
        if let Some(child) = children.next() {
            if child == to {
                acc += 1;
            } else if !visited.contains(&child) {
                visited.insert(child);
                stack.push(graph.neighbors_directed(child, Outgoing));
            }
        } else {
            stack.pop();
            visited.pop();
        }
    }
    return acc;
}

/// Finds directed shortest path between source and target.
/// ----------
///
/// :param List[Tuple[(uint, uint)...] edges: A list of triplets [(a,b, w) ...] representing a directed edge from a to b with weight w.
///
/// :param uint start:  The source node
///
/// :param uint goal: The target node
///

#[pyfunction]
fn directed_simple_paths(
    edges: Vec<(usize,usize)>,
    from: usize,
    to: usize
) -> usize
{
    let graph = DiGraphMap::<usize, usize>::from_edges(&edges);
    let mut visited: HashMap::<usize, usize> = HashMap::new();
    return dfs(&graph, &mut visited, from, to);
}

fn dfs(
    g: &DiGraphMap::<usize, usize>,
    visited: &mut HashMap::<usize, usize>,
    from: usize,
    to: usize
) -> usize
{
    if from == to {
        return 1;
    }
    match visited.get(&from) {
        Some(&val) => {
            return val;
        },
        _ => {
            let mut acc = 0;
            for child in g.neighbors_directed(from, Outgoing) {
                acc += dfs(g, visited, child, to);
            }
            visited.insert(from, acc);
            return acc;
        }
    }
    // if visited.contains_key(&from) {
    //     return visited[&from];
    // }
    
    // return acc;
}

/// A Python module implemented in Rust.
#[pymodule]
fn r_simple_paths(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simple_paths, m)?)?;
    m.add_function(wrap_pyfunction!(directed_simple_paths, m)?)?;
    Ok(())
}