use crate::models::todo_models::{CreateTodo, Todo, TodoRepository, TodoRepositoryForMemory};

fn create_todo(payload: CreateTodo, memory: TodoRepositoryForMemory) -> Todo {
    let todo = memory.create(payload);
    todo
}

#[cfg(test)]
mod tests {
    use crate::handlers::todo_handlers::create_todo;
    use crate::models::todo_models::{CreateTodo, TodoRepository, TodoRepositoryForMemory};

    #[test]
    fn todoを作成した後そのtodoが返ってくる() {
        // given
        let memory = TodoRepositoryForMemory::new();
        let payload = CreateTodo { action: String::from("帰宅する") };

        // when
        let todo = create_todo(payload, memory.clone());

        // then
        assert_eq!(
            todo,
            memory.read(1).unwrap()
        )
    }
}
