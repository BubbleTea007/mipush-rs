use crate::constants;
use crate::message::{Message, MessageType, MultiTopicOp};
use std::error::Error;

pub struct Sender {
    app_secret: String,
    is_global: bool,
}

impl Sender {
    pub fn new(app_secret: &str) -> Sender {
        Sender {
            app_secret: app_secret.to_string(),
            is_global: false,
        }
    }
    pub fn global(mut self) -> Self {
        self.is_global = true;
        self
    }

    fn send(self, params: Vec<(String, String)>, msg_type: MessageType) -> String {
        let mut api_url: String = match self.is_global {
            true => constants::API_ADDRESS_GLOBAL,
            false => constants::API_ADDRESS,
        }
        .to_owned();
        api_url.push_str(match msg_type {
            MessageType::REGID => constants::V3_MESSAGE_REGID,
            MessageType::ALIAS => constants::V3_MESSAGE_ALIAS,
            MessageType::USER_ACCOUNT => constants::V2_MESSAGE_USER_ACCOUNT,
            MessageType::TOPIC => constants::V3_MESSAGE_TOPIC,
            MessageType::MULTI_TOPIC => constants::V3_MESSAGE_MULTI_TOPIC,
            MessageType::ALL => constants::V3_MESSAGE_ALL,
        });

        let res = reqwest::Client::new()
            .post(api_url.as_str())
            .header("Authorization", "key=".to_string() + &self.app_secret)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send();
        match res {
            Result::Ok(mut v) => match v.text() {
                Result::Ok(v) => v,
                Result::Err(e) => e.description().to_string(),
            },
            Result::Err(e) => e.description().to_string(),
        }
    }

    fn build_params(
        &self,
        message: Message,
        msg_type: MessageType,
        target: String,
    ) -> Vec<(String, String)> {
        let mut params = vec![
            (String::from("title"), message.title),
            (String::from("description"), message.description),
            (String::from("payload"), message.payload),
            (String::from("notify_type"), message.notify_type.to_string()),
            (
                String::from("restricted_package_name"),
                message.restricted_package_name,
            ),
        ];
        if let Some(notify_id) = message.notify_id {
            params.push((String::from("notify_id"), notify_id.to_string()));
        };

        if let Some(ttl) = message.time_to_live {
            params.push((String::from("time_to_live"), ttl.to_string()));
        };

        if let Some(tts) = message.time_to_send {
            params.push((String::from("time_to_send"), tts.to_string()));
        };

        for (k, v) in message.extra.iter() {
            params.push(((String::from("extra.") + k), v.to_string()));
        }

        match msg_type {
            MessageType::REGID => params.push((String::from("registration_id"), target)),
            MessageType::ALIAS => params.push((String::from("alias"), target)),
            MessageType::USER_ACCOUNT => params.push((String::from("user_account"), target)),
            MessageType::TOPIC => params.push((String::from("topic"), target)),
            MessageType::MULTI_TOPIC => params.push((String::from("topics"), target)),
            MessageType::ALL => (),
        };
        params
    }

    pub fn send_to_regid(self, message: Message, regid: Vec<&str>) -> String {
        let params = self.build_params(message, MessageType::REGID, regid.join(","));
        self.send(params, MessageType::REGID)
    }

    pub fn send_to_alias(self, message: Message, alias: Vec<&str>) -> String {
        let params = self.build_params(message, MessageType::ALIAS, alias.join(","));
        self.send(params, MessageType::ALIAS)
    }

    pub fn send_to_user_account(self, message: Message, user_account: Vec<&str>) -> String {
        let params = self.build_params(message, MessageType::USER_ACCOUNT, user_account.join(","));
        self.send(params, MessageType::USER_ACCOUNT)
    }

    pub fn send_to_topic(self, message: Message, topic: &str) -> String {
        let params = self.build_params(message, MessageType::TOPIC, topic.to_string());
        self.send(params, MessageType::TOPIC)
    }

    pub fn send_to_multi_topic(
        self,
        message: Message,
        topic: Vec<&str>,
        topic_op: MultiTopicOp,
    ) -> String {
        let mut params = self.build_params(message, MessageType::MULTI_TOPIC, topic.join(";$;"));
        match topic_op {
            MultiTopicOp::EXCEPT => params.push((String::from("topic_op"), String::from("EXCEPT"))),
            MultiTopicOp::INTERSECTION => {
                params.push((String::from("topic_op"), String::from("INTERSECTION")))
            }
            MultiTopicOp::UNION => params.push((String::from("topic_op"), String::from("UNION"))),
        }
        self.send(params, MessageType::MULTI_TOPIC)
    }

    pub fn broadcast_all(self, message: Message) -> String {
        let params = self.build_params(message, MessageType::ALL, String::from("ALL"));
        self.send(params, MessageType::ALL)
    }
}
