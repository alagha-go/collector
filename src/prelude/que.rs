use aws_sdk_sqs::operation::send_message_batch::builders::SendMessageBatchFluentBuilder as Builder;
use aws_sdk_sqs::types::builders::SendMessageBatchRequestEntryBuilder as MessageBuilder;
use aws_sdk_sqs::types::SendMessageBatchRequestEntry as Message;
use rayon::prelude::*;
use bson::oid::ObjectId;
use super::*;


pub async fn build_messages(iter: Vec<impl Into<String>>) -> Vec<Builder> {
    let mut index = 0;
    let count = {
        if iter.len() % *SQSCONCURRENCY != 0 {
            iter.len() / *SQSCONCURRENCY +1
        }else {
            iter.len() / *SQSCONCURRENCY
        }
    };
    let mut controller = Controller::new();
    let messages: Vec<Message> = iter.into_iter().map(|body|{
        if controller.count >= *SQSCONCURRENCY {
            controller = Controller::new();
        }
        controller.count+=1;
        let id = ObjectId::new().to_hex();
        Message::builder()
        .id(&id)
        .message_body(body)
        .message_deduplication_id(id)
        .message_group_id(controller.id.to_hex())
        .build()
    }).collect();
    let rounds = split(messages.into_iter(), 10);
    let client = sqs().await;
    rounds.into_iter().map(|messages|{
        client.send_message_batch()
        .queue_url(&*QUEUEURL)
        .set_entries(Some(messages))
    }).collect()
}

struct Controller {
    id: ObjectId,
    count: usize
}

impl Controller {
    fn new() -> Self {
        let id = ObjectId::new();
        let count = 0;
        Self{id, count}
    }
}