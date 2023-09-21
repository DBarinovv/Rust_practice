use std::cmp::Ordering;
use std::collections::BTreeSet;

use time::Date;
use time::macros::date;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum TaskPriority {
    Low,
    Medium,
    High,
}

#[derive(Debug)]
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
    pub fn new(description: &str, priority: TaskPriority) -> Self {
        Self { id: 0, description: description.to_owned(), priority, due_date: None }
    }

    pub fn new_with_due_date(description: &str, priority: TaskPriority, due_date: Date) -> Self {
        Self { id: 0, description: description.to_owned(), priority, due_date: Some(due_date) }
    }

    fn print(&self) {
        println!("Task {} with {:?} priority and due_date = {}\n ", self.id, self.priority, self.due_date.map_or("No due date".to_string(), |date| date.to_string()));
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
        for task in &self.tasks {
            task.print();
        }
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
        let mut app = App::new();

        let task1 = Task::new("Huawei", TaskPriority::Low);
        let task2 = Task::new_with_due_date("Make bed", TaskPriority::Medium, date!(2023-09-04));

        app.add_task(task1);
        app.add_task(task2);

        app.print_tasks();
    }
}
