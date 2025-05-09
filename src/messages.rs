use std::collections::BTreeMap;

use cdumay_result::{Result, ResultBuilder};
use serde::{Deserialize, Serialize};
use serde_value::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub entrypoint: String,
    pub metadata: BTreeMap<String, Value>,
    pub params: Option<Value>,
    pub result: Result,
    pub uuid: uuid::Uuid,
}

#[derive(Default)]
pub struct MessageBuilder {
    entrypoint: String,
    metadata: Option<BTreeMap<String, Value>>,
    params: Option<Value>,
    result: Option<Result>,
    uuid: Option<uuid::Uuid>,
}

impl MessageBuilder {
    pub fn new(entrypoint: String) -> Self {
        Self {
            entrypoint,
            metadata: None,
            params: None,
            result: None,
            uuid: None,
        }
    }
    pub fn metadata(mut self, metadata: BTreeMap<String, Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn params(mut self, params: Value) -> Self {
        self.params = Some(params);
        self
    }
    pub fn result(mut self, result: Result) -> Self {
        self.result = Some(result);
        self
    }
    pub fn uuid(mut self, uuid: uuid::Uuid) -> Self {
        self.uuid = Some(uuid);
        self
    }
    pub fn build(self) -> Message {
        let final_uuid = self.uuid.unwrap_or(uuid::Uuid::new_v4());
        Message {
            entrypoint: self.entrypoint,
            metadata: self.metadata.unwrap_or_default(),
            params: self.params,
            result: self.result.unwrap_or(ResultBuilder::default().uuid(final_uuid).build()),
            uuid: final_uuid,
        }
    }
}

impl Default for Message {
    fn default() -> Message {
        let final_uuid = uuid::Uuid::new_v4();
        Message {
            uuid: final_uuid,
            entrypoint: String::new(),
            metadata: BTreeMap::new(),
            params: None,
            result: ResultBuilder::default().uuid(final_uuid).build(),
        }
    }
}

impl From<&Message> for ResultBuilder {
    fn from(msg: &Message) -> ResultBuilder {
        ResultBuilder::default().uuid(msg.uuid.clone())
    }
}
