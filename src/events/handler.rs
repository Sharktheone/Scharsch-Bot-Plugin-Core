use ws::{Message as WsMessage};


fn handle_message(message: WsMessage) {
    println!("Message: {:?}", message);
}