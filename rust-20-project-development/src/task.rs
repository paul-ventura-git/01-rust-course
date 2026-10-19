#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: usize, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            completed: false,
        }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }
}

#[cfg(test)]
mod tests {
    use super::Task;

    #[test]
    fn new_tasks_start_incomplete() {
        let task = Task::new(1, "Practice Rust");
        assert_eq!(task.id, 1);
        assert!(!task.completed);
    }

    #[test]
    fn tasks_can_be_completed() {
        let mut task = Task::new(1, "Practice Rust");
        task.mark_completed();
        assert!(task.completed);
    }
}
