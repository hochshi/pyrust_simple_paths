use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use petgraph::graphmap::DiGraphMap;
use indexmap::IndexSet;
use indexmap::indexset;

// use petgraph::visit::{IntoNeighborsDirected, NodeCount};
use petgraph::Direction::Outgoing;

use std::collections::hash_map::RandomState;

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

/// A Python module implemented in Rust.
#[pymodule]
fn r_simple_paths(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simple_paths, m)?)?;
    Ok(())
}