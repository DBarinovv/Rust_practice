use std::cmp::Ordering;
use std::collections::BTreeSet;

use time::Date;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum TaskPriority {
    Low,
    Medium,
    High,
}

struct Task {
    id: u64,
    description: String,
    priority: TaskPriority,
    due_date: Option<Date>,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Task { }

impl Task {
    pub fn new(description: String, priority: TaskPriority) -> Self {
        Self { id: 0, description, priority, due_date: None }
    }

    fn print(&self) {
        println!("Task {} with {:?} priority and due_date = {}\n ", self.id, self.priority, self.due_date.unwrap_or(Date::MIN));
    }
}

pub struct App {
    tasks: BTreeSet<Task>,
    last_task_id: u64,
}

impl App {
    pub fn new() -> Self {
        Self { tasks: BTreeSet::new(), last_task_id: 1 }
    }

    fn print_tasks(&self) {
        self.tasks.iter().map(|task| task.print());
    }

    fn add_task(&mut self, mut task: Task) {
        task.id = self.last_task_id;
        self.last_task_id += 1;

        self.tasks.insert(task);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let app = App::new();
        // app.add_task(task);
    }
}
