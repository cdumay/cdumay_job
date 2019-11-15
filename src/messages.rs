use std::collections::HashMap;

use cdumay_result::{ResultBuilder, ResultRepr};
use serde_value::Value;

pub trait Message {
    fn uuid(&self) -> uuid::Uuid;
    fn entrypoint(&self) -> String;
    fn metadata(&self) -> HashMap<String, Value>;
    fn params(&self) -> HashMap<String, Value>;
    fn result(&self) -> ResultRepr;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageRepr {
    entrypoint: String,
    metadata: HashMap<String, Value>,
    params: HashMap<String, Value>,
    result: ResultRepr,
    uuid: uuid::Uuid,
}

impl MessageRepr {
    pub fn new(uuid: Option<uuid::Uuid>, entrypoint: &str, params: Option<HashMap<String, Value>>, result: Option<ResultRepr>, metadata: Option<HashMap<String, Value>>) -> MessageRepr {
        let muuid = uuid.unwrap_or(uuid::Uuid::new_v4());
        MessageRepr {
            entrypoint: entrypoint.to_string(),
            params: params.unwrap_or(HashMap::new()),
            metadata: metadata.unwrap_or(HashMap::new()),
            result: result.unwrap_or(ResultBuilder::default().uuid(muuid).build()),
            uuid: muuid,
        }
    }
    pub fn uuid_mut(&mut self) -> &mut uuid::Uuid { &mut self.uuid }
    pub fn entrypoint_mut(&mut self) -> &mut String { &mut self.entrypoint }
    pub fn metadata_mut(&mut self) -> &mut HashMap<String, Value> { &mut self.metadata }
    pub fn params_mut(&mut self) -> &mut HashMap<String, Value> { &mut self.params }
    pub fn result_mut(&mut self) -> &mut ResultRepr { &mut self.result }
}

impl Message for MessageRepr {
    fn uuid(&self) -> uuid::Uuid { self.uuid.clone() }
    fn entrypoint(&self) -> String { self.entrypoint.clone() }
    fn metadata(&self) -> HashMap<String, Value> { self.metadata.clone() }
    fn params(&self) -> HashMap<String, Value> { self.params.clone() }
    fn result(&self) -> ResultRepr { self.result.clone() }
}

impl Default for MessageRepr {
    fn default() -> MessageRepr {
        let uuid = uuid::Uuid::new_v4();
        MessageRepr {
            uuid,
            entrypoint: String::new(),
            metadata: HashMap::new(),
            params: HashMap::new(),
            result: ResultBuilder::default().uuid(uuid).build(),
        }
    }
}

impl From<&MessageRepr> for ResultBuilder {
    fn from(msg: &MessageRepr) -> ResultBuilder {
        ResultBuilder::default().uuid(msg.uuid())
    }
}

impl From<&MessageRepr> for ResultRepr {
    fn from(msg: &MessageRepr) -> ResultRepr {
        ResultBuilder::from(msg).build()
    }
}