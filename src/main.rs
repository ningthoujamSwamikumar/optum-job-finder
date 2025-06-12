use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::env;

const URL: &str = "https://careers.unitedhealthgroup.com/job-search-results/?ref=21995521";
const ELEMENT_ID: &str = "widget-jobsearch-results-list";
const CHILD_CLASS: &str = "jobid-21995521";

fn main() {
    println!("Welcome to optum Job Finder!");
    match check_website() {
        Some(content) => {
            println!("Element found with matching child: {}", content);
            send_email(&content);
        }
        None => println!("Conditions not met."),
    }
}

fn check_website() -> Option<String> {
    let response = get(URL).ok()?.text().ok()?;
    let document = Html::parse_document(&response);

    let parent_selector = Selector::parse(&format!("#{}", ELEMENT_ID)).ok()?;
    let child_selector = Selector::parse(&format!(".{}", CHILD_CLASS)).ok()?;

    document.select(&parent_selector).find_map(|parent| {
        parent
            .select(&child_selector)
            .next()
            .map(|child| child.text().collect())
    })
}

fn send_email(content: &str) {
    let email_sender = env::var("EMAIL_SENDER").expect("Missing EMAIL_SENDER");
    let email_password = env::var("EMAIL_PASSWORD").expect("Missing EMAIL_PASSWORD");
    let email_receiver = env::var("EMAIL_RECEIVER").expect("Missing EMAIL_RECEIVER");

    let email = Message::builder()
        .from(email_sender.parse().unwrap())
        .to(email_receiver.parse().unwrap())
        .subject("Optum Job Found!")
        .body(format!("Element with child found: {}", content))
        .unwrap();

    let creds = Credentials::new(email_sender.clone(), email_password);
    let mailer = SmtpTransport::relay("smtp.office365.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Failed to send email: {}", e),
    }
}
