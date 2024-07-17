use std::collections::BTreeMap;

use cdumay_result::{Result, ResultBuilder};
use cdumay_core::Value;

pub trait MessageInfo {
    fn uuid(&self) -> uuid::Uuid;
    fn entrypoint(&self) -> String;
    fn metadata(&self) -> BTreeMap<String, Value>;
    fn params(&self) -> BTreeMap<String, Value>;
    fn result(&self) -> Result;
}

#[derive(Debug, Clone)]
pub struct Message {
    entrypoint: String,
    metadata: BTreeMap<String, Value>,
    params: BTreeMap<String, Value>,
    result: Result,
    uuid: uuid::Uuid,
}

impl Message {
    pub fn new(uuid: Option<uuid::Uuid>, entrypoint: &str, params: Option<BTreeMap<String, Value>>, result: Option<Result>, metadata: Option<BTreeMap<String, Value>>) -> Message {
        let muuid = uuid.unwrap_or(uuid::Uuid::new_v4());
        Message {
            entrypoint: entrypoint.to_string(),
            params: params.unwrap_or(BTreeMap::new()),
            metadata: metadata.unwrap_or(BTreeMap::new()),
            result: result.unwrap_or(ResultBuilder::default().uuid(muuid.into()).build()),
            uuid: muuid,
        }
    }
    pub fn uuid_mut(&mut self) -> &mut uuid::Uuid { &mut self.uuid }
    pub fn entrypoint_mut(&mut self) -> &mut String { &mut self.entrypoint }
    pub fn metadata_mut(&mut self) -> &mut BTreeMap<String, Value> { &mut self.metadata }
    pub fn params_mut(&mut self) -> &mut BTreeMap<String, Value> { &mut self.params }
    pub fn result_mut(&mut self) -> &mut Result { &mut self.result }
}

impl MessageInfo for Message {
    fn uuid(&self) -> uuid::Uuid { self.uuid.clone() }
    fn entrypoint(&self) -> String { self.entrypoint.clone() }
    fn metadata(&self) -> BTreeMap<String, Value> { self.metadata.clone() }
    fn params(&self) -> BTreeMap<String, Value> { self.params.clone() }
    fn result(&self) -> Result { self.result.clone() }
}

impl Default for Message {
    fn default() -> Message {
        let uuid = uuid::Uuid::new_v4();
        Message {
            uuid,
            entrypoint: String::new(),
            metadata: BTreeMap::new(),
            params: BTreeMap::new(),
            result: ResultBuilder::default().uuid(uuid.into()).build(),
        }
    }
}

impl From<Message> for ResultBuilder {
    fn from(msg: Message) -> ResultBuilder {
        ResultBuilder::default().uuid(msg.uuid().into())
    }
}
