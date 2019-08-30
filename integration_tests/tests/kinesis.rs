#![cfg(feature = "kinesis")]

extern crate rusoto_core;
extern crate rusoto_kinesis;

use rusoto_core::Region;
use rusoto_kinesis::{Kinesis, KinesisClient, ListStreamsInput};

#[test]
fn should_list_streams() {
    let client = KinesisClient::new(Region::UsEast1);
    let request = ListStreamsInput::default();

    client.list_streams(request).sync().unwrap();
}