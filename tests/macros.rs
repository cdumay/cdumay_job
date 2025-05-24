#[cfg(test)]
mod test_macros {
    use cdumay_error_standard::Unexpected;
    use cdumay_job::{define_task, TaskExec};

    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
    pub struct User {
        pub name: String,
        pub age: i32,
    }
    
    define_task! {
        Hello { params: User}
    }
    
    impl TaskExec for Hello {
        fn run(&mut self, mut result: cdumay_job::Result) -> Result<cdumay_job::Result, cdumay_core::Error> {
            result.stdout = Some(format!("Hello {}", self.params.name));
            Ok(result)
        }
    }
    
    define_task! {
        HelloError { params: User}
    }
    
    impl TaskExec for HelloError {
        fn run(&mut self, _: cdumay_job::Result) -> Result<cdumay_job::Result, cdumay_core::Error> {
            Err(Unexpected::new().with_message("Task failed !".to_string()).into())
        }
    }
    
    #[test]
    fn job_success() {
        let params = User {name: "John".into(), age: 27};
        let mut task = Hello::new(Some(params), None);
        let result = task.execute(None);
        assert_eq!(result.retcode, 0);
        assert_eq!(result.stdout.unwrap(), "Hello John");
    }

    #[test]
    fn job_error() {
        let params = User {name: "John".into(), age: 27};
        let mut task = HelloError::new(Some(params), None);
        let result = task.execute(None);
        assert_eq!(result.retcode, 500);
        assert_eq!(result.stderr.unwrap(), "Task failed !");
    }
}
