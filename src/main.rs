extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};
use std::{fs};

fn main() {
    let file = fs::read_to_string("./data/skill-sheet.yaml")
        .expect("Something went wrong reading the file");
    
    let docs = YamlLoader::load_from_str(&file).unwrap();

    let personal = personal_block(&docs[0]["personal"]);
    let certificates = certificates_block( &docs[0]["certificates"]);
    let self_introduction = self_introduction_block(&docs[0]["self_introduction"]);

    let sheet = format!("
# スキルシート

{personal}
{certificates}
{self_introduction}");

    println!("{}", sheet);
}

fn personal_block(personal: &Yaml) -> String {
    let name = personal["name"].as_str().unwrap();
    let birthday = personal["birthday"].as_str().unwrap();
    let gender = personal["gender"].as_str().unwrap();

    return format!("
## 個人情報
| 名前 | 生年月日 | 性別 |
| --- | --- | --- |
| {name} | {birthday} | {gender} |
");
}

fn certificates_block(certificates: &Yaml) -> String {
    let name = certificates["name"].as_str().unwrap();
    let certified_at = certificates["certified_at"].as_str().unwrap();
    let comment = certificates["comment"].as_str().unwrap();

    return format!("
## 資格
| 名前 | 生年月日 | 備考 |
| --- | --- | --- |
| {name} | {certified_at} | {comment} |
")
}

fn name_description_list(available: &Vec<Yaml>) -> String {
    let mut available_list: Vec<String> = vec![];

    for x in available {
        let name = x["name"].as_str().unwrap();
        if x["descriptions"].as_vec().is_none() {
            available_list.push(format!("
- {name}
"
            ));

            continue;
        }

        let descriptions = x["descriptions"].as_vec().unwrap();
        let mut description_list = vec![];

        for y in descriptions {
            let line = y.as_str().unwrap();
            description_list.push(format!("    - {line}"));
        }
        let description = description_list.join("\n");

        available_list.push(format!("
- {name}
{description}
"
        ));
    }

    return available_list.join("");
}

fn self_introduction_block(self_introduction: &Yaml) -> String {
    let available_vec = self_introduction["available"].as_vec().unwrap();
    let available = name_description_list(available_vec);

    let languages_use_frequently_vec = self_introduction["languages_use_frequently"].as_vec().unwrap();
    let languages_use_frequently = name_description_list(languages_use_frequently_vec);

    return format!("
## 自己PR
### 対応可能な言語
{available}
{languages_use_frequently}")
}