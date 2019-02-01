use cdumay_result::ResultRepr;
use serde_value::Value;
use std::collections::HashMap;
use cdumay_result::ResultReprBuilder;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageRepr {
    uuid: uuid::Uuid,
    entrypoint: String,
    metadata: HashMap<String, Value>,
    params: HashMap<String, Value>,
    result: ResultRepr,
}

impl MessageRepr {
    pub fn new(uuid: Option<uuid::Uuid>, entrypoint: &str, params: Option<HashMap<String, Value>>, result: Option<ResultRepr>, metadata: Option<HashMap<String, Value>>) -> MessageRepr {
        let muuid = uuid.unwrap_or(uuid::Uuid::new_v4());
        MessageRepr {
            entrypoint: entrypoint.to_string(),
            params: params.unwrap_or(HashMap::new()),
            metadata: metadata.unwrap_or(HashMap::new()),
            result: result.unwrap_or(ResultReprBuilder::new(Some(muuid), None).build()),
            uuid: muuid,
        }
    }
    pub fn uuid(&self) -> &uuid::Uuid { &self.uuid }
    pub fn uuid_mut(&mut self) -> &mut uuid::Uuid { &mut self.uuid }
    pub fn entrypoint(&self) -> &String { &self.entrypoint }
    pub fn entrypoint_mut(&mut self) -> &mut String { &mut self.entrypoint }
    pub fn metadata(&self) -> &HashMap<String, Value> { &self.metadata }
    pub fn metadata_mut(&mut self) -> &mut HashMap<String, Value> { &mut self.metadata }
    pub fn params(&self) -> &HashMap<String, Value> { &self.params }
    pub fn params_mut(&mut self) -> &mut HashMap<String, Value> { &mut self.params }
    pub fn result(&self) -> &ResultRepr { &self.result }
    pub fn result_mut(&mut self) -> &mut ResultRepr { &mut self.result }
}

impl Default for MessageRepr {
    fn default() -> MessageRepr {
        let uuid = uuid::Uuid::new_v4();
        MessageRepr {
            uuid,
            entrypoint: String::new(),
            metadata: HashMap::new(),
            params: HashMap::new(),
            result: ResultReprBuilder::new(Some(uuid), None).build(),
        }
    }
}

impl From<&MessageRepr> for ResultRepr {
    fn from(msg: &MessageRepr) -> ResultRepr {
        ResultReprBuilder::new(Some(*msg.uuid()), None).build()
    }
}