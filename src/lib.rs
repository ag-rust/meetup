extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use reqwest::{Client, Url, Error};

const MEETUP_API_URL: &str = "https://api.meetup.com/";

#[derive(Debug, Deserialize)]
pub struct Venue {
    id: u64,
    name: String,
    lat: f64,
    lon: f64,
    repinned: bool,
    address_1: String,
    city: String,
    country: String,
    localized_country_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GroupJoinMode {
    Open, Approval, Closed
}

#[derive(Debug, Deserialize)]
pub struct Group {
    created: u64,
    name: String,
    id: u64,
    join_mode: GroupJoinMode,
    lat: f64,
    lon: f64,
    urlname: String,
    who: String,
    localized_location: String,
    region: String,
    timezone: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventVisibility {
    Public, PublicLimited, Members
}

#[derive(Debug, Deserialize)]
pub struct Event {
    created: u64,
    duration: u64,
    id: String,
    name: String,
    status: EventStatus,
    time: u64,
    local_date: String,
    local_time: String,
    updated: u64,
    utc_offset: u64,
    waitlist_count: u32,
    yes_rsvp_count: u32,
    venue: Venue,
    group: Group,
    link: String,
    description: String,
    visibility: EventVisibility,
}

pub struct MeetupClient {
    name: String,
    url: Url,
}

impl MeetupClient {
    pub fn new(token: &str, name: &str) -> MeetupClient {
        let mut url = Url::parse(&format!("{}/{}", MEETUP_API_URL, name)).unwrap();
        url.query_pairs_mut().append_pair("key", token);
        MeetupClient{name: name.to_string(), url: url}
    }

    pub fn get_events(&self) -> Result<<Option<Vec<Event>>, Error> {
        let client = Client::new();
        let mut url = self.url.clone();
        url.set_path(&format!("{}/{}", self.name, "events"));
        println!("calling: {}", url);

        let events:Vec<Event> = client.get(url)
            .send()?.json()?;
        Ok(events)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Cancelled, Upcoming, Past, Proposed, Suggested, Draft
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn get_meetups() {
        let api_key = env::var("MEETUP_API_KEY").unwrap();
        let name = env::var("MEETUP_NAME").unwrap();
        let api = MeetupClient::new(&api_key, &name);
        let events = api.get_events();
        println!("{:#?}", events);
    }
}