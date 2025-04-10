// pub struct Message {
//     pub content: String,
//     // pub user: String
// }
//
// impl Message {
//     pub fn new(c : String) -> Self {
//         Message{
//             content:c,
//             // user:u,
//         }
//     }
//
//     pub fn send_sms(&self) -> Result<&str,Option<()>> {
//         if self.content.contains("stupid") || self.content.is_empty() {
//             return Err(None);
//         };
//         Ok(&self.content)
//     }
// }
pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.contains("stupid") || message.is_empty(){
        return Err("ERROR: illegal");
    };
    Ok(message)
}