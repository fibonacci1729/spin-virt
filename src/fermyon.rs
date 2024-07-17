use crate::bindings::exports::fermyon::spin::{
    key_value, 
    llm, 
    mqtt, 
    mysql, 
    postgres,
    redis,
    sqlite,
    variables,
};

pub struct KeyValueStore;

impl key_value::GuestStore for KeyValueStore {
    fn open(_label: String) -> Result<key_value::Store, key_value::Error> {
        Err(key_value::Error::NoSuchStore)
    }

    fn get(&self, _key: String) -> Result<Option<Vec<u8>>, key_value::Error> {
        unimplemented!()
    }

    fn set(&self, _key: String, _value: Vec<u8>) -> Result<(), key_value::Error> {
        unimplemented!()
    }

    fn delete(&self, _key: String) -> Result<(), key_value::Error> {
        unimplemented!()
    }

    fn exists(&self, _key: String) -> Result<bool, key_value::Error> {
        unimplemented!()
    }

    fn get_keys(&self) -> Result<Vec<String>, key_value::Error> {
        unimplemented!()
    }
}

impl key_value::Guest for crate::Component {
    type Store = KeyValueStore;
}

impl llm::Guest for crate::Component {
    fn infer(
        _model: llm::InferencingModel,
        _prompt: String,
        _params: Option<llm::InferencingParams>,
    ) -> Result<llm::InferencingResult, llm::Error> {
        Err(llm::Error::ModelNotSupported)
    }

    fn generate_embeddings(
        _model: llm::EmbeddingModel,
        _text: Vec<String>,
    ) -> Result<llm::EmbeddingsResult, llm::Error> {
        Err(llm::Error::ModelNotSupported)
    }
}

impl mqtt::GuestConnection for crate::Component {
    fn open(
        _address: String,
        _username: String,
        _password: String,
        _keep_alive_interval_in_secs: u64,
    ) -> Result<mqtt::Connection, mqtt::Error> {
        Err(mqtt::Error::Other("Access Denied".to_string()))
    }

    fn publish(
        &self,
        _topic: String,
        _payload: mqtt::Payload,
        _qos: mqtt::Qos,
    ) -> Result<(), mqtt::Error> {
        unimplemented!()
    }
}

impl mqtt::Guest for crate::Component {
    type Connection = Self;
}

impl mysql::GuestConnection for crate::Component {
    fn open(_address: String) -> Result<mysql::Connection, mysql::Error> {
        Err(mysql::Error::Other("Access Denied".to_string()))
    }

    fn query(
        &self,
        _statement: String,
        _params: Vec<mysql::ParameterValue>,
    ) -> Result<mysql::RowSet, mysql::Error> {
        unimplemented!()
    }

    fn execute(
        &self,
        _statement: String,
        _params: Vec<mysql::ParameterValue>,
    ) -> Result<(), mysql::Error> {
        unimplemented!()
    }
}

impl mysql::Guest for crate::Component {
    type Connection = Self;
}

impl postgres::GuestConnection for crate::Component {
    fn open(_address: String) -> Result<postgres::Connection, postgres::Error> {
        Err(postgres::Error::Other("Access Denied".to_string()))
    }

    fn query(
        &self,
        _statement: String,
        _params: Vec<postgres::ParameterValue>,
    ) -> Result<postgres::RowSet, postgres::Error> {
        unimplemented!()
    }

    fn execute(
        &self,
        _statement: String,
        _params: Vec<postgres::ParameterValue>,
    ) -> Result<u64, postgres::Error> {
        unimplemented!()
    }
}

impl postgres::Guest for crate::Component {
    type Connection = Self;
}

impl redis::GuestConnection for crate::Component {
    fn open(_address: String) -> Result<redis::Connection, redis::Error> {
        Err(redis::Error::Other("Access Denied".to_string()))
    }

    fn publish(&self, _channel: String, _payload: redis::Payload) -> Result<(), redis::Error> {
        unimplemented!()
    }

    fn get(&self, _key: String) -> Result<Option<redis::Payload>, redis::Error> {
        unimplemented!()
    }

    fn set(&self, _key: String, _value: redis::Payload) -> Result<(), redis::Error> {
        unimplemented!()
    }

    fn incr(&self, _key: String) -> Result<i64, redis::Error> {
        unimplemented!()
    }

    fn del(&self, _keys: Vec<String>) -> Result<u32, redis::Error> {
        unimplemented!()
    }

    fn sadd(
        &self,
        _key: String,
        _values: Vec<String>,
    ) -> Result<u32, redis::Error> {
        unimplemented!()
    }

    fn smembers(&self, _key: String) -> Result<Vec<String>, redis::Error> {
        unimplemented!()
    }

    fn srem(
        &self,
        _key: String,
        _values: Vec<String>,
    ) -> Result<u32, redis::Error> {
        unimplemented!()
    }

    fn execute(
        &self,
        _command: String,
        _arguments: Vec<redis::RedisParameter>,
    ) -> Result<Vec<redis::RedisResult>, redis::Error> {
        unimplemented!()
    }
}

impl redis::Guest for crate::Component {
    type Connection = Self;
}

impl sqlite::GuestConnection for crate::Component {
    fn open(_database: String) -> Result<sqlite::Connection, sqlite::Error> {
        Err(sqlite::Error::NoSuchDatabase)
    }

    fn execute(
        &self,
        _statement: String,
        _parameters: Vec<sqlite::Value>,
    ) -> Result<sqlite::QueryResult, sqlite::Error> {
        unimplemented!()
    }
}

impl sqlite::Guest for crate::Component {
    type Connection = Self;
}

impl variables::Guest for crate::Component {
    fn get(name: String) -> Result<String, variables::Error> {
        Err(variables::Error::Undefined(name))
    }
}