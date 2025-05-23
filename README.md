# cdumay_job

[![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
[![cdumay_job on crates.io](https://img.shields.io/crates/v/cdumay_job)](https://crates.io/crates/cdumay_job)
[![cdumay_job on docs.rs](https://docs.rs/cdumay_job/badge.svg)](https://docs.rs/cdumay_job)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_job)

A library to follow job execution using validation and checks steps.

### Quickstart

_Cargo.toml_:
```toml
[dependencies]
cdumay_core = "0.1"
cdumay_job = "1.0"
serde = "1.0"
serde_json = "1.0"
serde-value = "0.7"
env_logger = "0.11"
hostname = "0.4"
```

_main.rs_:
```rust
use cdumay_core::Error;
use cdumay_job::{ResultBuilder, Status, TaskExec, TaskInfo};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct HelloParams {
    user: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hello {
    metadata: (),
    params: Option<HelloParams>,
    result: cdumay_job::Result,
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
    fn result(&self) -> cdumay_job::Result {
        self.result.clone()
    }
    fn result_mut(&mut self) -> &mut cdumay_job::Result {
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
    fn run(&mut self, mut result: cdumay_job::Result) -> Result<cdumay_job::Result, Error> {
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
```
**Log Output (using RUST_LOG=debug)**
```
[2025-05-23T18:19:04Z INFO  cdumay_job::task] cdumay_job::Hello[cbbe52c1-d3f2-4cd0-a050-8966d581c1ab] - TaskExecution-Start
[2025-05-23T18:19:04Z INFO  cdumay_job::task] cdumay_job::Hello[cbbe52c1-d3f2-4cd0-a050-8966d581c1ab] - Run-Start
[2025-05-23T18:19:04Z INFO  cdumay_job::task] cdumay_job::Hello[cbbe52c1-d3f2-4cd0-a050-8966d581c1ab] - Run-End => Ok(0, stdout: Some("Hello John Smith from laptop"))
[2025-05-23T18:19:04Z INFO  cdumay_job::task] cdumay_job::Hello[cbbe52c1-d3f2-4cd0-a050-8966d581c1ab] - TaskExecution-End => Ok(0, stdout: Some("Hello John Smith from laptop"))
```
**Result**
```json
{
  "uuid": "e744d3fa-9da4-45c5-9f1e-aefe7fc50ddb",
  "retcode": 0,
  "stdout": "Hello John Smith from laptop",
  "stderr": null,
  "retval": {}
}
```

### Macros

The macro [`define_task!`] allow to implement [`TaskInfo`] to simplify the code

The following code reuse the previous example

```rust
use cdumay_core::Error;
use cdumay_job::{define_task, ResultBuilder, Status, TaskExec, TaskInfo};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HelloParams {
    user: String
}

define_task!{
    Hello { params: HelloParams }
}

impl TaskExec for Hello {
    fn run(&mut self, mut result: cdumay_job::Result) -> Result<cdumay_job::Result, Error> {
        let host = match hostname::get() {
            Ok(os_string) => os_string.to_string_lossy().to_string(),
            Err(_) => "localhost".to_string()
        };

        Ok({
            result.stdout = Some(format!("Hello {} from {}", self.params().user, host));
            result
        })
    }
}

fn main() {
    env_logger::init();
    let params = HelloParams { user: "John Smith".into() };
    let mut task = Hello::new(Some(params), None);
    println!("{}", serde_json::to_string_pretty(&task.execute(None)).unwrap());
}
```

