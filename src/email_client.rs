use reqwest::Client;

use crate::domain::SubscriberEmail;

#[derive(Clone)]
pub struct EmailClient {
    address: String,
    email: SubscriberEmail,
    client: Client
}

impl EmailClient {
    
    pub fn new(addr: String, sender: SubscriberEmail) -> Self {
        EmailClient{address: addr, email: sender, client: Client::new()}
    }

    pub async fn send(
        &self,
        recipient: SubscriberEmail,
        subject: String, 
        html: String,
        plain: &str 
        ) -> Result<(), reqwest::Error> {
        let body = EmailRequest{
            sender: self.email.as_ref(),
            recipient: recipient.as_ref(),
            subject: subject.as_str(),
            html: html.as_str(),
            plain
        };
        
        match self.client
            .post(self.address.as_str())
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await {
                Ok(_) => Ok(()), 
                Err(e) =>{
                    tracing::error!("Failed to send req: {}", e);
                    Err(e)
                }
            }
    }
}

#[derive(serde::Serialize)]
struct EmailRequest<'a> {
    sender: &'a str,
    recipient: &'a str,
    subject: &'a str,
    html: &'a str,
    plain: &'a str,
}
