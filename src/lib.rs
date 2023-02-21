use std::{collections::HashMap, sync::Arc};

use crabler::*;
use serde::Serialize;
use surf::Url;
use tokio::sync::Mutex;

pub struct ContactPair{
  pub website: String,
  pub emails: Vec<String>
}

#[derive(Serialize)]
pub struct SerContactPair {
  website: String,
  emails: String
}

impl From<ContactPair> for SerContactPair {
  fn from(value: ContactPair) -> Self {
    Self { website: value.website, emails: value.emails.join("+") }
  }
}

#[derive(WebScraper)]
#[on_html("a", contact_handler)]
pub struct ContactScraper {
  list: Vec<String>,
  map: Arc<Mutex<HashMap<String, Vec<String>>>>
}

impl ContactScraper {
  pub fn new(list: Vec<String>) -> Self {
    let mut map = HashMap::new();
    for site in list.iter() {
      map.insert(site.to_owned().into(), vec![]);
    }
    Self {
      list: list.into_iter().map(|s| s.to_owned()).collect(),
      map: Arc::new(Mutex::new(map))
    }
  }

  pub async fn start(self) -> Vec<ContactPair> {
    let opts = Opts::new().with_urls(self.list.iter().map(|s| s.as_str()).collect());
    let map = self.map.clone();
    self.run(opts).await.unwrap();
    let locked = map.lock().await;
    locked.clone().into_iter().map(|(website, emails)| ContactPair {website, emails}).collect()
  }

  async fn contact_handler(&self, mut response: Response, a: Element) -> Result<()> {
    if let Some(text) = a.text() {
      if let Some(href) = a.attr("href") {
        if text.to_lowercase().contains("contact") {
          let full_link = Url::parse(&response.url).unwrap().join(&href).unwrap().to_string();
          println!("Found contact page at {}", &full_link);
          response.navigate(full_link).await?;
        }
        if href.contains("mailto:") {
          let url = Url::parse(&response.url).unwrap();
          let domain = format!("https://{}", url.domain().unwrap());
          match self.map.lock().await.get_mut(&domain) {
            Some(mails) => 
              mails.push(href.replace("mailto:", "")),
            _ => {}
          }
        }
      }    
    } 

    Ok(())
  }
}
