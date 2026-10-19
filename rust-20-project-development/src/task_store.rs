use crate::task::Task;

#[derive(Default)]
pub struct TaskStore {
    tasks: Vec<Task>,
}

impl TaskStore {
    pub fn add(&mut self, title: impl Into<String>) -> usize {
        let id = self.tasks.len() + 1;
        self.tasks.push(Task::new(id, title));
        id
    }

    pub fn complete(&mut self, id: usize) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.mark_completed();
            true
        } else {
            false
        }
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }
}

#[cfg(test)]
mod tests {
    use super::TaskStore;

    #[test]
    fn adds_and_completes_tasks() {
        let mut store = TaskStore::default();
        let id = store.add("Write tests");
        assert!(store.complete(id));
        assert!(store.tasks()[0].completed);
    }

    #[test]
    fn completing_unknown_task_reports_failure() {
        let mut store = TaskStore::default();
        assert!(!store.complete(42));
    }
}
