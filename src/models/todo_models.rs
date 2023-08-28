use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use serde::Serialize;

#[derive(PartialEq, Serialize, Debug, Clone)]
enum TodoStatus {
    Undone,
    InProgress,
    Done
}

#[derive(PartialEq, Serialize, Debug, Clone)]
pub struct Todo {
    id: u32,
    action: String,
    status: TodoStatus,
}

pub struct CreateTodo {
    pub(crate) action: String,
}

impl Todo {
    pub(crate) fn new(id: u32, action: String) -> Self {
        Todo {
            id,
            action,
            status: TodoStatus::Undone
        }
    }
}

pub trait TodoRepository {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn read(&self, id: u32) -> Option<Todo>;
}

type TodoDates = HashMap<u32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDates>>
}

impl TodoRepositoryForMemory {
    pub(crate) fn new() -> Self {
        TodoRepositoryForMemory {
            store: Arc::default()
        }
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<TodoDates> {
        self.store.write().unwrap()
    }

    pub(crate) fn read_store_ref(&self) -> RwLockReadGuard<TodoDates> {
        self.store.read().unwrap()
    }
}

impl TodoRepository for TodoRepositoryForMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as u32;
        let todo = Todo::new(id, payload.action.clone());
        store.insert(id, todo.clone());
        todo
    }

    fn read(&self, id: u32) -> Option<Todo> {
        let store = self.read_store_ref();
        store.get(&id).cloned()
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
            memory.read(create_todo.id).unwrap(),
            Todo {
                id: 1,
                action: String::from("掃除をする"),
                status: TodoStatus::Undone,
            }
        );
    }

    #[test]
    fn todoを取得する() {
        // given
        let memory = TodoRepositoryForMemory::new();
        let payload = CreateTodo { action: String::from("勉強をする") };
        let create_todo = memory.create(payload);

        // when
        let read_todo = memory.read(create_todo.id).unwrap();

        // then
        assert_eq!(
            read_todo,
            Todo {
                id: 1,
                action: String::from("勉強をする"),
                status: TodoStatus::Undone,
            }
        )
    }
}
