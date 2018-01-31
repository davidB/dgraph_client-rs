use chrono::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Root {
	pub me: Vec<Person>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct School {
	pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Location {
	#[serde(rename = "type")]
	pub kind: String,
	pub coordinates: Vec<f64>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Person {
	//pub uid: Option<String>,
	pub name: String,
	pub age: Option<u8>,
	pub dob: Option<DateTime<Utc>>,
	pub married: Option<bool>,
	#[serde(rename = "raw_bytes")]
	pub raw: Option<Vec<u8>>,
	#[serde(rename = "friend")]
	pub friends: Option<Vec<Person>>,
	#[serde(rename = "loc")]
	pub location: Option<Location>,
	pub school: Option<Vec<School>>,
}
