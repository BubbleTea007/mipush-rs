extern crate mipush_rs;

use mipush_rs::Message;
use mipush_rs::MessageBuilder;
use mipush_rs::Sender;

#[test]
fn test() {
    let sender = Sender::new("your_app_secret");
    let _msg: Message = MessageBuilder::new()
        .title("title")
        .description("desc")
        .restricted_package_name("your_package_name")
        .payload("payload")
        .pass_through(0)
        .notify_type(1)
        .extra("key", "value")
        .build();
    let result = sender.send_to_alias(_msg, vec!["aaa"]);
    println!("result=====>{}", result);
}
