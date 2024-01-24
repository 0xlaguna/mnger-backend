use rocket::{Build, Rocket};
use revolt_rocket_okapi::{mount_endpoints_and_merged_docs, revolt_okapi::openapi3::OpenApi, settings::OpenApiSettings};

mod users;

pub fn mount(mut rocket: Rocket<Build>) -> Rocket<Build> {
    let settings = OpenApiSettings::default();

    mount_endpoints_and_merged_docs! {
        rocket, "/".to_owned(), settings,
        "/" => (vec![], custom_openapi_spec()),
    }

    rocket
}


fn custom_openapi_spec() -> OpenApi {
    use revolt_rocket_okapi::revolt_okapi::openapi3::*;

    let mut extensions = schemars::Map::new();
    extensions.insert(
        "x-logo".to_owned(),
        json!({
            "url": "https://mnger.com/header.png",
            "altText": "Mnger Header"
        }),
    );

    extensions.insert(
        "x-tagGroups".to_owned(),
        json!([
          {
            "name": "Users",
            "tags": [
              "User Information",
              "Direct Messaging",
              "Relationships"
            ]
          },
        ]),
    );

    OpenApi {
        openapi: OpenApi::default_version(),
        info: Info {
            title: "Mnger API".to_owned(),
            description: Some("Maintenance Platform".to_owned()),
            terms_of_service: Some("https://mnger.com/terms".to_owned()),
            contact: Some(Contact {
                name: Some("Mnger Support".to_owned()),
                url: Some("https://mnger.com".to_owned()),
                email: Some("contact@mnger.com".to_owned()),
                ..Default::default()
            }),
            version: "0.1.0".to_string(),
            ..Default::default()
        },
        servers: vec![
            Server {
                url: "https://api.mnger.com".to_owned(),
                description: Some("Mnger Production".to_owned()),
                ..Default::default()
            },
        ],
        external_docs: Some(ExternalDocs {
            url: "https://developers.mgner.com".to_owned(),
            description: Some("Mnger Developer Documentation".to_owned()),
            ..Default::default()
        }),
        tags: vec![
            Tag {
                name: "User Information".to_owned(),
                description: Some("Query and fetch users on Mnger".to_owned()),
                ..Default::default()
            },
        ],
        ..Default::default()
    }
}
