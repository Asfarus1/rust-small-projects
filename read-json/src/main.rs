use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize, Debug)]
struct Paragraph {
    name: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Chapter {
    name: String,
    paragraphs: Vec<Paragraph>
}

fn main() {
    let json  = r#"
    {
        "name": "Chapter 1",
        "paragraphs": [
            {
                "name": "Paragraph 1"
            },
            {
                "name": "Paragraph 2"
            }
        ]
    }"#;

    let chapter: Chapter = serde_json::from_str(json).unwrap();
    println!("{:#?}", &chapter);

    println!("{:#?}", serde_json::to_string(&chapter));
}
