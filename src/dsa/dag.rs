// src/dsa/dag.rs
// Directed Acyclic Graph (DAG) for Deadlock-Free Task Dependency Topological Sorting.

use anyhow::{bail, Result};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct TaskDag {
    edges: HashMap<String, Vec<String>>,
    in_degree: HashMap<String, usize>,
    nodes: HashSet<String>,
}

impl TaskDag {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            in_degree: HashMap::new(),
            nodes: HashSet::new(),
        }
    }

    pub fn add_task(&mut self, task_id: &str) {
        self.nodes.insert(task_id.to_string());
        self.in_degree.entry(task_id.to_string()).or_insert(0);
        self.edges.entry(task_id.to_string()).or_insert_with(Vec::new);
    }

    pub fn add_dependency(&mut self, parent: &str, child: &str) {
        self.add_task(parent);
        self.add_task(child);
        self.edges.get_mut(parent).unwrap().push(child.to_string());
        *self.in_degree.get_mut(child).unwrap() += 1;
    }

    pub fn topological_sort(&self) -> Result<Vec<String>> {
        let mut in_degree = self.in_degree.clone();
        let mut queue = VecDeque::new();
        let mut order = Vec::new();

        for (task, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(task.clone());
            }
        }

        while let Some(task) = queue.pop_front() {
            order.push(task.clone());

            if let Some(children) = self.edges.get(&task) {
                for child in children {
                    let deg = in_degree.get_mut(child).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(child.clone());
                    }
                }
            }
        }

        if order.len() != self.nodes.len() {
            bail!("Cyclic dependency detected in task DAG!");
        }

        Ok(order)
    }
}
