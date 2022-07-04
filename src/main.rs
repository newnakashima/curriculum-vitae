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
    let projects = projects_block(&docs[0]["projects"]);

    let sheet = format!("
# スキルシート

{personal}
{certificates}
{self_introduction}
### 参画したプロジェクト一覧
{projects}
");

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
| 名前 | 取得年月 | 備考 |
| --- | --- | --- |
| {name} | {certified_at} | {comment} |
")
}

fn name_description_list(list: &Vec<Yaml>) -> String {
    let mut string_list: Vec<String> = vec![];

    for x in list {
        let name = x["name"].as_str().unwrap();
        if x["descriptions"].as_vec().is_none() {
            string_list.push(format!("
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

        string_list.push(format!("
- {name}
{description}
"
        ));
    }

    return string_list.join("");
}

fn self_introduction_block(self_introduction: &Yaml) -> String {
    let available_vec =
        self_introduction["available"].as_vec().unwrap();
    let available =
        name_description_list(available_vec);

    let languages_use_frequently_vec = 
        self_introduction["languages_use_frequently"].as_vec().unwrap();
    let languages_use_frequently = 
        name_description_list(languages_use_frequently_vec);

    let languages_used_ever_vec = 
        self_introduction["languages_used_ever"].as_vec().unwrap();
    let languages_used_ever =
        name_description_list(languages_used_ever_vec);

    let middleware_use_frequently_vec =
        self_introduction["middleware_use_frequently"].as_vec().unwrap();
    let middleware_use_frequently =
        name_description_list(middleware_use_frequently_vec);

    let middleware_used_ever_vec =
        self_introduction["middleware_used_ever"].as_vec().unwrap();
    let middleware_used_ever = 
        name_description_list(middleware_used_ever_vec);
    
    let virtual_environment_use_frequently_vec =
        self_introduction["virtual_environment_use_frequently"].as_vec().unwrap();
    let virtual_environment_use_frequently =
        name_description_list(virtual_environment_use_frequently_vec);
    
    let cloud_platform_use_frequently_vec =
        self_introduction["cloud_platform_use_frequently"].as_vec().unwrap();
    let cloud_platform_use_frequently =
        name_description_list(cloud_platform_use_frequently_vec);

    return format!("
## 自己PR
### 対応可能な業務
{available}
### 得意な言語
{languages_use_frequently}
### 業務で使用したことのある言語
{languages_used_ever}
### よく使用するミドルウェア
{middleware_use_frequently}
### 業務で使用したことのあるミドルウェア
{middleware_used_ever}
### よく使用する仮想環境
{virtual_environment_use_frequently}
### 得意なクラウドプラットフォーム
{cloud_platform_use_frequently}")
}

fn projects_block(projects: &Yaml) -> String {
    let projects = projects.as_vec().unwrap();
    let mut list: Vec<String> = vec![];

    for (i, p) in projects.iter().enumerate() {
        let description = p["description"].as_str().unwrap();
        let term = p["term"].as_str().unwrap();
        let members = p["members"].as_str().unwrap();
        let role = p["role"].as_str().unwrap();
        let assigned_for = p["assigned_for"].as_str().unwrap();
        let achievement = p["achievement"].as_str().unwrap();
        let phase = p["phase"].as_str().unwrap();
        let technology = p["technology"].as_str().unwrap();
        let others = p["others"].as_str().unwrap();

        let mut owned = format!("| {} | {description} | {term} | {members} | {role} ", i + 1).to_owned();
        owned.push_str(&format!("| {assigned_for} | {achievement} | {phase} | {technology} | {others} |"));
        list.push(owned);
    }

    let projects_str = list.join("\n");

    return format!("
| No. | 概要 | 期間 | 人数 | ロール | 担当業務 | 習得スキル・実績 | 参画フェーズ | 言語・OS・DB | その他ツール等 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
{projects_str}")
}