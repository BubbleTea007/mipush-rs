use std::collections::HashMap;

pub enum MessageType {
    REGID,
    ALIAS,
    #[allow(non_camel_case_types)]
    USER_ACCOUNT,
    TOPIC,
    #[allow(non_camel_case_types)]
    MULTI_TOPIC,
    ALL,
}

pub enum MultiTopicOp {
    UNION,
    INTERSECTION,
    EXCEPT,
}

#[derive(Debug)]
pub struct Message {
    pub title: String,
    pub description: String,
    pub notify_type: i32,
    pub notify_id: Option<i32>,
    pub pass_through: i32,
    pub restricted_package_name: String,
    pub payload: String,
    pub time_to_live: Option<i64>,
    pub time_to_send: Option<i64>,
    pub extra: HashMap<String, String>,
}

#[derive(Default)]
pub struct MessageBuilder {
    title: String,
    description: String,
    notify_type: i32,
    notify_id: Option<i32>,
    pass_through: i32,
    restricted_package_name: String,
    payload: String,
    time_to_live: Option<i64>,
    time_to_send: Option<i64>,
    extra: HashMap<String, String>,
}

impl MessageBuilder {
    pub fn new() -> MessageBuilder {
        MessageBuilder::default()
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn notify_type(mut self, notify_type: i32) -> Self {
        self.notify_type = notify_type;
        self
    }

    pub fn notify_id(mut self, notify_id: i32) -> Self {
        self.notify_id = Some(notify_id);
        self
    }

    pub fn pass_through(mut self, pass_through: i32) -> Self {
        self.pass_through = pass_through;
        self
    }

    pub fn restricted_package_name(mut self, package_name: &str) -> Self {
        self.restricted_package_name = package_name.to_string();
        self
    }

    pub fn payload(mut self, payload: &str) -> Self {
        self.payload = payload.to_string();
        self
    }

    pub fn time_to_live(mut self, time_to_live: i64) -> Self {
        self.time_to_live = Some(time_to_live);
        self
    }

    pub fn time_to_send(mut self, time_to_send: i64) -> Self {
        self.time_to_send = Some(time_to_send);
        self
    }

    pub fn extra(mut self, k: &str, v: &str) -> Self {
        self.extra.insert(k.to_string(), v.to_string());
        self
    }

    pub fn build(self) -> Message {
        Message {
            title: self.title,
            description: self.description,
            notify_type: self.notify_type,
            notify_id: self.notify_id,
            pass_through: self.pass_through,
            restricted_package_name: self.restricted_package_name,
            payload: self.payload,
            time_to_live: self.time_to_live,
            time_to_send: self.time_to_send,
            extra: self.extra,
        }
    }
}
