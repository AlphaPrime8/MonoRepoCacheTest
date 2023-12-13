
#[tokio::main]
async fn main() {
    let a = utils::ClientRequests::new("APP 1".to_string());
    let b = utils::serialize_client_request(a);
    let c = utils::deserialize_client_request(b);
    println!("{:?}", c);
}
