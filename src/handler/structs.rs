use super::*;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Queue {
    #[serde(alias = "records")]
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub message_id: String,
    pub receipt_handle: String,
    pub body: String,
    pub attributes: Attributes,
    pub message_attributes: Value,
    pub md5_of_body: String,
    pub event_source: String,
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: String,
    pub aws_region: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Attributes {
    pub approximate_receive_count: String,
    pub sent_timestamp: String,
    pub sender_id: String,
    pub approximate_first_receive_timestamp: String,
}



#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HttpRequest {
    pub headers: Headers,
    pub is_base64_encoded: bool,
    pub raw_path: String,
    pub request_context: RequestContext,
    pub route_key: String,
    pub body: String,
    pub raw_query_string: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Headers {
    pub content_length: String,
    pub x_amzn_tls_version: String,
    pub x_forwarded_proto: String,
    pub postman_token: String,
    pub x_forwarded_port: String,
    pub x_forwarded_for: String,
    pub accept: String,
    pub x_amzn_tls_cipher_suite: String,
    pub x_amzn_trace_id: String,
    pub host: String,
    pub content_type: String,
    pub cache_control: String,
    pub accept_encoding: String,
    pub user_agent: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestContext {
    pub account_id: String,
    pub time_epoch: i64,
    pub route_key: String,
    pub stage: String,
    pub domain_prefix: String,
    pub request_id: String,
    pub domain_name: String,
    pub http: Http,
    pub time: String,
    pub api_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Http {
    pub path: String,
    pub protocol: String,
    pub method: String,
    pub source_ip: String,
    pub user_agent: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum Type {
    #[default]
    None,
    Person,
    Movie,
    Tv
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum Data {
    #[default]
    None,
    Type(Type),
    HttpRequest(HttpRequest),
    Queue(Queue)
}