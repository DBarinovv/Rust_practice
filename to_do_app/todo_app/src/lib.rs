use time::Date;

enum TaskPriority {
    Low,
    Medium,
    High,
}

struct ToDo {
    id: u64,
    description: String,
    priority: TaskPriority,
    due_date: Date,
}

#[cfg(test)]
mod tests {
    use super::*;
}
