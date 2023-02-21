use contact_csv::{ContactScraper, SerContactPair};
use csv::{Reader, Writer};


#[tokio::main]
async fn main() {
  let file_name = "website_list.csv";
  let output = "contact_list.csv";
  let mut csv_reader = Reader::from_path(file_name)
    .expect(&format!("Can't open {}", file_name));
  let mut list = vec![];
  while let Some(Ok(result)) = csv_reader.records().next() {
    let site = result.get(0).unwrap().to_owned();
    list.push(site);
  }
  
  let scraper = ContactScraper::new(list);
  let mails = scraper.start().await;
  let mut csv_writer = Writer::from_path(output).unwrap();
  for pair in mails {
    csv_writer.serialize(SerContactPair::from(pair)).unwrap()
  }
  csv_writer.flush().unwrap();
}
