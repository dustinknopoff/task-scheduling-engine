use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub mod date_utils;
pub mod graph_utils;

use date_utils::{add_business_days, shift_to_first_next_business_day, sub_business_days};
use graph_utils::{dfs, make_graph_from_tasks, make_reverse_graph};

pub type ID = String;
pub type Graph = HashMap<ID, HashSet<ID>>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    id: ID,
    title: String,
    start: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    duration: f64,
    position: isize,
    progress: f64,
    resource_id: ID,
    dependencies: Option<Vec<ID>>,
}

impl PartialEq for Task {
    fn eq(&self, other: &Task) -> bool {
        self.id == other.id
    }
}

pub fn schedule_tasks(tasks: &mut Vec<Task>, today: DateTime<FixedOffset>) -> &mut Vec<Task> {
    let graph: Graph = make_graph_from_tasks(&tasks);
    let tasks_by_id = tasks.clone();
    let tasks_by_id =
        tasks_by_id
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut map, (index, task)| {
                map.insert(task.id.clone(), (task, index));
                map
            });

    let reverse_graph = make_reverse_graph(&graph);

    dfs(&graph, |id, _| {
        let (t, index) = tasks_by_id[&id];
        let is_source = match reverse_graph.get(&id) {
            Some(val) => val.is_empty(),
            None => false,
        };
        let is_sink = match graph.get(&id) {
            Some(val) => val.is_empty(),
            None => false,
        };

        let is_disconnected = is_source && is_sink;

        if is_source && is_disconnected {
            tasks[index] = update_start_date(&t, today);
        } else {
            let prerequisition_date = reverse_graph
                .get(&id)
                .unwrap_or(&HashSet::new())
                .iter()
                .map(|id| tasks_by_id[id].0.end)
                .max();
            if let Some(prerequisition_date) = prerequisition_date {
                let saved = update_start_date(&t, add_business_days(prerequisition_date, 1));
                tasks[index] = saved;
            }
        }
    });

    tasks
}

fn update_start_date(task: &Task, start_date: DateTime<FixedOffset>) -> Task {
    let mut new_task = task.clone();
    let corrected_start = shift_to_first_next_business_day(start_date);
    let days_spent = (task.duration * task.progress).floor();
    let new_start_date = sub_business_days(corrected_start, days_spent as usize);

    if task.start == new_start_date {
        new_task
    } else {
        new_task.start = sub_business_days(corrected_start, days_spent as usize);
        new_task.end = add_business_days(new_task.start, (task.duration - 1_f64) as usize);
        new_task
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schedule_tasks;

    #[test]
    fn test_schedule_tasks() {
        let today_for_test =
            DateTime::parse_from_str("2020-01-01 12:29:03.274 +0000", "%Y-%m-%d %H:%M:%S%.3f %z")
                .unwrap();
        let day_plus_one =
            DateTime::parse_from_str("2020-01-02 12:29:03.274 +0000", "%Y-%m-%d %H:%M:%S%.3f %z")
                .unwrap();
        let three =
            DateTime::parse_from_str("2020-01-03 12:29:03.274 +0000", "%Y-%m-%d %H:%M:%S%.3f %z")
                .unwrap();
        let four =
            DateTime::parse_from_str("2020-01-04 12:29:03.274 +0000", "%Y-%m-%d %H:%M:%S%.3f %z")
                .unwrap();
        let six =
            DateTime::parse_from_str("2020-01-06 12:29:03.274 +0000", "%Y-%m-%d %H:%M:%S%.3f %z")
                .unwrap();
        let seven =
            DateTime::parse_from_str("2020-01-07 12:29:03.274 +0000", "%Y-%m-%d %H:%M:%S%.3f %z")
                .unwrap();
        let eight =
            DateTime::parse_from_str("2020-01-08 12:29:03.274 +0000", "%Y-%m-%d %H:%M:%S%.3f %z")
                .unwrap();
        let nine =
            DateTime::parse_from_str("2020-01-09 12:29:03.274 +0000", "%Y-%m-%d %H:%M:%S%.3f %z")
                .unwrap();
        let ten =
            DateTime::parse_from_str("2020-01-10 12:29:03.274 +0000", "%Y-%m-%d %H:%M:%S%.3f %z")
                .unwrap();
        let mut tasks: Vec<Task> = vec![
            Task {
                id: "0".to_string(),
                title: "Make scheduling algorithm".to_string(),
                start: today_for_test,
                end: day_plus_one,
                duration: 2_f64,
                position: 0,
                progress: 0_f64,
                resource_id: "Alice".to_string(),
                dependencies: Some(vec!["1".to_string()]),
            },
            Task {
                id: "1".to_string(),
                title: "Write tests for algorithms".to_string(),
                start: today_for_test,
                end: day_plus_one,
                duration: 2_f64,
                position: 1,
                progress: 0_f64,
                resource_id: "Bob".to_string(),
                dependencies: None,
            },
            Task {
                id: "2".to_string(),
                title: "Research a lot of gantt plotting libs".to_string(),
                start: today_for_test,
                end: day_plus_one,
                duration: 2_f64,
                position: 2,
                progress: 0_f64,
                resource_id: "Bob".to_string().to_string(),
                dependencies: Some(vec!["3".to_string()]),
            },
            Task {
                id: "3".to_string(),
                title: "Write your own".to_string(),
                start: today_for_test,
                end: day_plus_one,
                duration: 2_f64,
                position: 3,
                progress: 0_f64,
                resource_id: "Alice".to_string(),
                dependencies: None,
            },
        ];

        let correctly_scheduled_tasks: Vec<Task> = vec![
            Task {
                id: "0".to_string(),
                title: "Make scheduling algorithm".to_string(),
                start: today_for_test,
                end: day_plus_one,
                duration: 2_f64,
                position: 0,
                progress: 0_f64,
                resource_id: "Alice".to_string(),
                dependencies: Some(vec!["1".to_string()]),
            },
            Task {
                id: "1".to_string(),
                title: "Write tests for algorithms".to_string(),
                start: three,
                end: six,
                duration: 2_f64,
                position: 1,
                progress: 0_f64,
                resource_id: "Bob".to_string(),
                dependencies: None,
            },
            Task {
                id: "2".to_string(),
                title: "Research a lot of gantt plotting libs".to_string(),
                start: seven,
                end: eight,
                duration: 2_f64,
                position: 2,
                progress: 0_f64,
                resource_id: "Bob".to_string().to_string(),
                dependencies: Some(vec!["3".to_string()]),
            },
            Task {
                id: "3".to_string(),
                title: "Write your own".to_string(),
                start: nine,
                end: ten,
                duration: 2_f64,
                position: 3,
                progress: 0_f64,
                resource_id: "Alice".to_string(),
                dependencies: None,
            },
        ];

        let mut tasks_with_progresses: Vec<Task> = vec![
            Task {
                id: "0".to_string(),
                title: "Make scheduling algorithm".to_string(),
                start: today_for_test,
                end: four,
                duration: 4_f64,
                position: 0,
                progress: 0.5,
                resource_id: "Alice".to_string(),
                dependencies: Some(vec!["1".to_string()]),
            },
            Task {
                id: "1".to_string(),
                title: "Write tests for algorithms".to_string(),
                start: today_for_test,
                end: day_plus_one,
                duration: 2_f64,
                position: 1,
                progress: 0.5,
                resource_id: "Bob".to_string(),
                dependencies: None,
            },
        ];

        let correctly_scheduled_tasks_with_progresses: Vec<Task> = vec![
            Task {
                id: "0".to_string(),
                title: "Make scheduling algorithm".to_string(),
                start: sub_business_days(today_for_test, 2),
                end: day_plus_one,
                duration: 4_f64,
                position: 0,
                progress: 0.5,
                resource_id: "Alice".to_string(),
                dependencies: Some(vec!["1".to_string()]),
            },
            Task {
                id: "1".to_string(),
                title: "Write tests for algorithms".to_string(),
                start: add_business_days(today_for_test, 1),
                end: three,
                duration: 2_f64,
                position: 1,
                progress: 0.5,
                resource_id: "Bob".to_string(),
                dependencies: None,
            },
        ];
        assert_eq!(
            schedule_tasks(&mut tasks, today_for_test).clone(),
            correctly_scheduled_tasks
        );

        assert_eq!(
            schedule_tasks(&mut tasks_with_progresses, today_for_test).clone(),
            correctly_scheduled_tasks_with_progresses
        )
    }
}
