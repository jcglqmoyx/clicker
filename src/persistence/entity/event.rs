use serde::{Deserialize, Serialize};

mod button_serde {
    use enigo::Button;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(button: &Button, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let button = match button {
            Button::Left => "Left",
            Button::Middle => "Middle",
            Button::Right => "Right",
            _ => "Invalid button"
        };
        serializer.serialize_str(button)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Button, D::Error>
        where
            D: Deserializer<'de>,
    {
        let button_str = String::deserialize(deserializer)?;
        match button_str.as_str() {
            "Left" => Ok(Button::Left),
            "Middle" => Ok(Button::Middle),
            "Right" => Ok(Button::Right),
            _ => Err(serde::de::Error::custom("Invalid button value")),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct Event {
    pub(crate) x: i32,
    pub(crate) y: i32,
    #[serde(with = "button_serde")]
    pub(crate) button: enigo::Button,
    pub(crate) click_type: u8,
    pub(crate) sleep: u64,
}

impl Event {
    pub(crate) fn new(x: i32, y: i32, button: enigo::Button, click_type: u8, sleep: u64) -> Event {
        Event {
            x,
            y,
            button,
            click_type,
            sleep,
        }
    }
}