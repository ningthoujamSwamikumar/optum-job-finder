use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::env;

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
    let job_id = match env::var("JOB_ID") {
        Ok(val) => val,
        Err(_) => env::args().nth(1).expect("Missing JOB_ID"),
    };

    let url = format!("https://careers.unitedhealthgroup.com/job-search-results/?ref={job_id}");
    let parent_id = "widget-jobsearch-results-list";
    let child_class = "job";

    println!("finding - parent_id: {parent_id}, child_class: '{child_class}' at page: {url}");

    let response = get(url).ok()?.text().ok()?;
    let document = Html::parse_document(&response);

    let parent_selector = Selector::parse(&format!("#{}", parent_id)).ok()?;
    let child_selector = Selector::parse(&format!(".{}", child_class)).ok()?;

    document.select(&parent_selector).find_map(|parent| {
            parent
            .select(&child_selector)
            .next()
            .map(|child| child.text().collect())
    })
}

fn send_email(content: &str) {
    let email_sender = match env::var("EMAIL_SENDER") {
        Ok(val) => val,
        Err(_) => env::args().nth(2).expect("Missing EMAIL_SENDER"),
    };
    let email_password = match env::var("EMAIL_PASSWORD") {
        Ok(val) => val,
        Err(_) => env::args().nth(3).expect("Missing EMAIL_PASSWORD"),
    };
    let email_receiver = match env::var("EMAIL_RECEIVER") {
        Ok(val) => val,
        Err(_) => env::args().nth(4).expect("Missing EMAIL_RECEIVER"),
    };

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
