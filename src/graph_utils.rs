use crate::{Graph, Task, ID};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator as _;

pub fn make_graph_from_tasks(tasks: &Vec<Task>) -> Graph {
    let mut graph = Graph::new();
    let mut resources: HashMap<ID, Vec<Task>> = HashMap::new();

    tasks.iter().for_each(|task| {
        let tasks_for_resource = resources
            .entry(task.resource_id.clone())
            .or_insert(Vec::new());
        tasks_for_resource.push(task.clone());

        graph.insert(
            task.id.to_owned(),
            HashSet::from_iter(task.dependencies.clone().unwrap_or(Vec::new())),
        );
    });

    resources.values_mut().for_each(|tasks_for_resource| {
        tasks_for_resource.sort_by(|a, b| a.position.cmp(&b.position));
        let prev: Option<&Task> = None;
        tasks_for_resource.iter().for_each(|task| {
            if let Some(prev_task) = prev {
                let entry = graph.entry(prev_task.id.clone()).or_default();
                entry.insert(task.id.clone());
            }
        });
    });

    graph
}

pub fn make_reverse_graph(graph: &Graph) -> Graph {
    let mut return_graph = Graph::new();
    dfs(graph, |id, parent_id| {
        let mut prerequisitions = match graph.get(&id) {
            Some(val) => val.clone(),
            None => HashSet::new(),
        };
        if let Some(parent_id) = parent_id {
            prerequisitions.insert(parent_id.clone());
        }
        return_graph.insert(id.clone(), prerequisitions);
    });
    return_graph
}

pub fn dfs<S>(graph: &Graph, mut vertex_visitor: S)
where
    S: FnMut(ID, Option<ID>) + Sized,
{
    let mut visited: HashSet<ID> = HashSet::new();
    for (key, _) in graph.iter() {
        if visited.contains(key) {
            break;
        }
        let mut stack = vec![key];
        while !stack.is_empty() {
            let current = stack.pop();
            if let Some(current_vertex) = current {
                vertex_visitor(current_vertex.clone(), None);
                visited.insert(current_vertex.clone());
                let dependencies = graph.get(current_vertex);
                if let Some(dependencies) = dependencies {
                    dependencies.iter().for_each(|dependency_id| {
                        vertex_visitor(dependency_id.clone(), Some(current_vertex.clone()));
                        if !visited.contains(dependency_id) {
                            stack.push(dependency_id);
                        }
                    })
                }
            }
        }
    }
}
