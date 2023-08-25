use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::Serialize;

#[derive(PartialEq, Serialize, Debug, Clone)]
enum TodoStatus {
    Undone,
    InProgress,
    Done
}

#[derive(PartialEq, Serialize, Debug, Clone)]
pub struct Todo {
    id: i32,
    action: String,
    status: TodoStatus,
}

pub struct CreateTodo {
    action: String,
}

impl Todo {
    pub(crate) fn new(action: String) -> Self {
        Todo {
            id: 1,
            action,
            status: TodoStatus::Undone
        }
    }
}

pub trait TodoRepository {
    fn create(&self, payload: CreateTodo) -> Todo;
}

type TodoDates = HashMap<i32, Todo>;

pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDates>>
}

impl TodoRepositoryForMemory {
    pub(crate) fn new() -> Self {
        TodoRepositoryForMemory {
            store: Arc::default()
        }
    }
}

impl TodoRepository for TodoRepositoryForMemory {
    fn create(&self, _payload: CreateTodo) -> Todo {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todoを作成する() {
        // given
        let memory = TodoRepositoryForMemory::new();
        let payload = CreateTodo { action: String::from("掃除をする") };

        // when
        let create_todo = memory.create(payload);

        // then
        assert_eq!(
            create_todo,
            Todo {
                id: 1,
                action: String::from("掃除をする"),
                status: TodoStatus::Undone,
            }
        );
    }
}
