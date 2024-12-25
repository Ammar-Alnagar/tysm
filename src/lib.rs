mod chatgpt;
mod schema;

pub use chatgpt::ChatClient;

#[cfg(test)]
mod tests {
    use crate::ChatClient;

    use std::sync::LazyLock;
    static CLIENT: LazyLock<ChatClient> = LazyLock::new(|| {
        let my_api = "https://g7edusstdonmn3vxdh3qdypkrq0wzttx.lambda-url.us-east-1.on.aws/v1/chat/completions".to_string();
        ChatClient {
            url: my_api,
            ..ChatClient::from_env("gpt-4o").unwrap()
        }
    });

    #[derive(serde::Deserialize, schemars::JsonSchema, Debug)]
    struct Name {
        first: String,
        last: String,
    }

    #[tokio::test]
    async fn it_works() {
        let name: Name = CLIENT.chat("Who was the first president?").await.unwrap();

        assert_eq!(name.first, "George");
        assert_eq!(name.last, "Washington");

        let usage1 = CLIENT.usage();
        for _ in 0..5 {
            let _name: Name = CLIENT.chat("Who was the first president?").await.unwrap();
        }
        let usage2 = CLIENT.usage();
        assert_eq!(usage1, usage2);
    }

    #[derive(serde::Deserialize, schemars::JsonSchema, Debug)]
    struct NameWithAgeOfDeath {
        first: String,
        last: String,
        age_of_death: Option<u8>,
    }

    #[tokio::test]
    async fn optional_fields() {
        let name: NameWithAgeOfDeath = CLIENT.chat("Who was the famous physicist who was in a wheelchair and needed a computer program to talk?").await.unwrap();

        assert_eq!(name.first, "Stephen");
        assert_eq!(name.last, "Hawking");
        assert_eq!(name.age_of_death, Some(76));

        let name: NameWithAgeOfDeath = CLIENT.chat("Who was the actor in the 3rd reboot of the spiderman movies, this time in the MCU?").await.unwrap();
        assert_eq!(name.first, "Tom");
        assert_eq!(name.last, "Holland");
        assert_eq!(name.age_of_death, None);
    }
}
