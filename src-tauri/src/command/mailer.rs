use std::collections::HashMap;
use lettre::{SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::message::MessageBuilder;
use lettre::transport::smtp::authentication::Credentials;
use crate::command::data::{CsvReceiverRecord, CsvSenderRecord};

fn replace_content_in_template(template_content: String, receiver_data: CsvReceiverRecord) -> String {
    let replacement_content = receiver_data.replacement_data.clone();

    let mut result = template_content.clone();
    for (k, v) in replacement_content {
        let tag = format!(r"<<\[{}\]>>", k);
        let re = regex::RegexBuilder::new(&tag)
            .case_insensitive(true)
            .build()
            .unwrap();

        let replace_value = serde_json::to_value(v).unwrap().to_string();
        result = re.replace_all(&result, replace_value.trim_matches('"')).to_string();
    }

    result
}

#[tauri::command]
pub async fn send_email_by_thread(_app: tauri::AppHandle, sender_data: Vec<CsvSenderRecord>, receiver_data: HashMap<String, Vec<CsvReceiverRecord>>, email_subject: HashMap<String, String>, template_path: HashMap<String, String>, timeout: i32) {
    println!("{:?}", timeout);
    let mut thread_pool = vec![];

    for sender in sender_data {
        if !receiver_data.contains_key(&sender.email) || !template_path.contains_key(&sender.email) || !email_subject.contains_key(&sender.email) {
            continue;
        }

        let smtp_credentials = Credentials::new(sender.email.clone(), sender.password.clone());
        let smtp_transport_relay = SmtpTransport::relay("smtp.gmail.com").unwrap();
        let smtp_transport = smtp_transport_relay.credentials(smtp_credentials).build();

        let email_subject = email_subject.clone();

        let receiver = receiver_data[&sender.email].clone();
        let template_content = std::fs::read_to_string(&template_path[&sender.email]).unwrap();
        let handler = std::thread::spawn(move || {
            for rec in receiver {
                let content = replace_content_in_template(template_content.clone(), rec.clone());
                // println!("{}", format!("{:?} <{}>", sender.name.clone().unwrap(), sender.email));
                let email = MessageBuilder::new()
                    .from(format!("{:?} <{}>", sender.name.clone().unwrap(), sender.email).parse().unwrap())
                    .to(rec.email.clone().parse().unwrap())
                    .subject(&email_subject[&sender.email].clone())
                    .header(ContentType::TEXT_HTML)
                    .body(content)
                    .unwrap();

                smtp_transport.send(&email.into()).expect("cannot send the email");
                std::thread::sleep(std::time::Duration::from_secs(timeout as u64));
            }

        });

        thread_pool.push(handler);
    }

    for thread in thread_pool {
        thread.join().unwrap();
    }
}


