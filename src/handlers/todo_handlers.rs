
#[cfg(test)]
mod tests {
    use crate::models::todo_models::{CreateTodo, TodoRepository, TodoRepositoryForMemory};

    #[test]
    fn todoを作成した後そのtodoが返ってくる() {
        // given
        let memory = TodoRepositoryForMemory::new();
        let payload = CreateTodo { action: String::from("帰宅する") };

        // when
        let todo = create_todo(payload);

        // then
        assert_eq!(
            todo,
            memory.read(1)
        )
    }
}
