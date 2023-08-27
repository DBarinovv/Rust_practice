use time::Date;

#[derive(Debug)]
enum TaskPriority {
    Low,
    Medium,
    High,
}

struct Task {
    id: u64,
    description: String,
    priority: TaskPriority,
    due_date: Date,
}

impl Task {
    fn print(&self) {
        println!("Task {} with {:?} priority and due_date = {}\n ", self.id, self.priority, self.due_date);
    }
}

struct App {
    tasks: std::collections::BTreeSet<Task>,
}

impl App {
    fn print_tasks(&self) -> Vec<Task> {
        self.tasks.iter().map(|task| task.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
