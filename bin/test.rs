use cdumay_core::Error;
use cdumay_job::{Result, ResultBuilder, Status, TaskExec, TaskInfo};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct HelloParams {
    user: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hello {
    metadata: (),
    params: Option<HelloParams>,
    result: Result,
    status: Status,
    uuid: uuid::Uuid,
}

impl TaskInfo for Hello {
    type ParamType = HelloParams;
    type MetadataType = ();
    fn path() -> String {
        format!("{}::{}", module_path!(), std::any::type_name::<Self>())
    }
    fn status(&self) -> Status {
        self.status.clone()
    }
    fn status_mut(&mut self) -> &mut Status {
        &mut self.status
    }
    fn uuid(&self) -> uuid::Uuid {
        self.uuid
    }
    fn result(&self) -> Result {
        self.result.clone()
    }
    fn result_mut(&mut self) -> &mut Result {
        &mut self.result
    }
    fn metadata(&self) -> &Self::MetadataType {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut Self::MetadataType {
        &mut self.metadata
    }
    fn params(&self) -> Self::ParamType {
        self.params.clone().unwrap_or_default()
    }
}

impl TaskExec for Hello {
    fn run(&mut self, mut result: Result) -> std::result::Result<Result, Error> {
        let host = match hostname::get() {
            Ok(os_string) => os_string.to_string_lossy().to_string(),
            Err(_) => "localhost".to_string(),
        };
        Ok({
            result.stdout = Some(format!("Hello {} from {}", self.params().user, host));
            result
        })
    }
}

fn main() {
    env_logger::init();

    let mut task = Hello {
        metadata: (),
        params: Some(HelloParams {
            user: "John Smith".to_string(),
        }),
        result: ResultBuilder::default().build(),
        status: Status::Pending,
        uuid: uuid::Uuid::new_v4(),
    };
    println!("{}", serde_json::to_string_pretty(&task.execute(None)).unwrap());
}
