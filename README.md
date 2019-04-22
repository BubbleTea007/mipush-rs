# mipush-rs
[![crates.io](https://img.shields.io/crates/v/mipush_rs.svg)](https://crates.io/crates/mipush_rs)
[![Documentation](https://docs.rs/mipush_rs/badge.svg)](https://docs.rs/mipush_rs)

A simple mipush server sdk for rust.

[mipush documentation offical website](https://dev.mi.com/console/doc/detail?pId=1163)


Just support android now, including following message types:
- Regid
- Alias
- Useraccount
- Topic/Multi Topic
- Broadcast

## Example

```rust
extern crate mipush_rs;

use mipush_rs::{Message, MessageBuilder, Sender};

#[test]
fn test() {
    let sender = Sender::new("your_app_secret");
//  let sender = Sender::new("your_app_secret").global(true);  //send to global api
    let _msg: Message = MessageBuilder::new()
        .title("title")
        .description("desc")
        .restricted_package_name("your_package_name")
        .payload("payload")
        .pass_through(0)
        .notify_type(1)
        .extra("sound_uri", "value")
        .extra("intent_uri", "value")
        .extra("web_uri", "value")
        .extra("sound_uri", "value")
        .build();
    let result = sender.send_to_alias(_msg, vec!["aaa"]);
    println!("result=====>{}", result);
}
```