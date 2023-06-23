use crate::{people, prelude::que};
use rayon::iter::Either;
use rayon::prelude::*;
use super::*;

pub async fn collection(typ: Type) -> Result<Value> {
    let rounds = match typ {
        Type::None => Vec::new(),
        Type::Person => people::ids().await?,
        Type::Movie => Vec::new(),
        Type::Tv => Vec::new()
    };
    if rounds.is_empty() {
        return  Ok(Value::Null);
    }
    let (iter, errors): (Vec<_>, Vec<Option<String>>) = rounds.into_par_iter().map(|round|{
        let message = Message{r#type: typ, round};
        let result = serde_json::to_string(&message);
        match result {
            Ok(json) => Either::Left(json),
            _ => Either::Right(None)
        }
    }).collect();
    let requests = que::build_messages(iter).await;
    let (mut successful, mut failed) = (Vec::new(), Vec::new());
    for request in requests {
        let output = request.send().await?;
        match output.successful() {
            None => (),
            Some(success) => {
                let mut success: Vec<Value> = success.into_iter().map(|message| {Value::String(message.id().unwrap_or_default().to_string())}).collect();
                successful.append(&mut success);
            }
        }
        match output.failed() {
            None => (),
            Some(failure) => {
                let mut failure: Vec<Value> = failure.into_iter().map(|message| {Value::String(message.id().unwrap_or_default().to_string())}).collect();
                failed.append(&mut failure);
            }
        }
    }
    let mut map = Map::new();
    map.insert(String::from("successful"), Value::Array(successful));
    map.insert(String::from("failed"), Value::Array(failed));
    Ok(Value::Object(map))
}

pub async fn queue_data(queue: Queue) -> Result<Value> {
    todo!()
}

pub async fn http_data(req: HttpRequest) -> Result<Value> {
    todo!()
}