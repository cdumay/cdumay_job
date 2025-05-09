#[macro_export]
macro_rules! define_task {
    ($name:ident) => {
        #[derive(Clone, Debug)]
        pub struct $name {
            message: cdumay_job::Message,
            status: cdumay_job::Status,
            result: cdumay_result::Result,
        }

        impl cdumay_job::TaskInfo for $name {
            fn new(msg: &cdumay_job::Message, result: Option<cdumay_result::Result>) -> $name {
                $name {
                    message: msg.clone(),
                    status: cdumay_job::Status::Pending,
                    result: result.unwrap_or(msg.result.clone()),
                }
            }
            fn path() -> String {
                format!("{}.{}", module_path!(), stringify!($name))
            }
            fn status(&self) -> cdumay_job::Status {
                self.status.clone()
            }
            fn status_mut(&mut self) -> &mut cdumay_job::Status {
                &mut self.status
            }
            fn message(&self) -> cdumay_job::Message {
                self.message.clone()
            }
            fn message_mut(&mut self) -> &mut cdumay_job::Message {
                &mut self.message
            }
            fn result(&self) -> cdumay_result::Result {
                self.result.clone()
            }
            fn result_mut(&mut self) -> &mut cdumay_result::Result {
                &mut self.result
            }
        }

        impl From<&cdumay_job::Message> for $name {
            fn from(msg: &cdumay_job::Message) -> $name {
                $name {
                    message: msg.clone(),
                    status: cdumay_job::Status::Pending,
                    result: msg.result.clone(),
                }
            }
        }
    };
}
