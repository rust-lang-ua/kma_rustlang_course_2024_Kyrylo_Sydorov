use serde::{Serializer, Deserializer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    date: String,
}

// Функція серіалізації дати
fn serialize_date<S>(date: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("Date: {}", date))
}

// Функція десеріалізації дати
fn deserialize_date<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.replace("Date: ", ""))
}

fn main() {
    let event = Event {
        name: "Концерт".to_string(),
        date: "2024-11-15".to_string(),
    };

    let json = serde_json::to_string(&event).expect("Помилка серіалізації");
    println!("Серіалізований JSON з кастомною датою: {}", json);

    let deserialized_event: Event = serde_json::from_str(&json).expect("Помилка десеріалізації");
    println!("Десеріалізована подія: {:?}", deserialized_event);
}