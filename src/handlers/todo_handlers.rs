use crate::models::todo_models::{CreateTodo, Todo, TodoRepository, TodoRepositoryForMemory};

fn create_todo(payload: CreateTodo, memory: TodoRepositoryForMemory) -> Todo {
    let todo = memory.create(payload);
    todo
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use axum::Extension;
    use crate::handlers::todo_handlers::create_todo;
    use crate::models::todo_models::{CreateTodo, TodoRepository, TodoRepositoryForMemory};

    #[test]
    fn todoを作成した後そのtodoが返ってくる() {
        // given
        let repository = TodoRepositoryForMemory::new();
        let arc_repository = Arc::new(repository);
        let payload = CreateTodo { action: String::from("帰宅する") };

        // when
        let result = create_todo(payload, Extension(arc_repository));

        // then
        assert_eq!(result.get("action"), payload.action)
    }
}
