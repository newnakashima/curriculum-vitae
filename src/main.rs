extern crate yaml_rust;
extern crate getopts;
use yaml_rust::{YamlLoader, Yaml};
use std::fs;
use getopts::Options;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = Options::new();
    opts.optflag("t", "table", "print skill-sheet as table");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    let render_table = matches.opt_present("t");

    let file = fs::read_to_string("./data/skill-sheet.yaml")
        .expect("Something went wrong reading the file");

    let docs = YamlLoader::load_from_str(&file).unwrap();

    let personal = personal_block(&docs[0]["personal"]);
    let certificates = certificates_block( &docs[0]["certificates"]);
    let self_introduction = self_introduction_block(&docs[0]["self_introduction"]);
    let projects = projects_block(&docs[0]["projects"], render_table);

    let sheet = format!("
# スキルシート

{personal}
{certificates}
{self_introduction}
<div style=\"break-before: page;\"></div>

# 参画したプロジェクト一覧

※直近三件のみ記載。<br/>
それ以前については https://github.com/newnakashima/curriculum-vitae/blob/main/src/skill-sheet.md を御覧ください。

{projects}
");

    println!("{}", sheet);
}

fn render_as_table(headers: Vec<String>, data: Vec<Vec<String>>) -> String {
    let header_row = format!("| {} |", headers.join(" | "));

    let borders: Vec<&str> = headers.iter().map(|_| -> &str { "---" }).collect();

    let border = format!("| {} |", borders.join("|"));

    let data_rows: Vec<String> = data.iter().map(|d| { format!("| {} |", d.join(" | ")) } ).collect();
    let rows = data_rows.join("\n");

    return format!("
{header_row}
{border}
{rows}
");
}

fn render_as_key_value_map(headers: Vec<String>, data: Vec<Vec<String>>) -> String {
    let mut data_blocks: Vec<String> = vec![];
    for line in data {
        let mut inner_block: Vec<String> = vec![];
        for (i, d) in line.iter().enumerate() {
            let mut block = String::from("").to_owned();
            let header = &headers[i];
            if i == 0 {
                block.push_str(&format!("## {d}\n"));
            } else {
                block.push_str(&format!("### {header}
{d}
"));
            }
            inner_block.push(block);
        }
        data_blocks.push(inner_block.join("\n"));
    }

    return data_blocks.join("\n<div style=\"break-before: page\"></div>\n\n");
}

fn personal_block(personal: &Yaml) -> String {
    let name = personal["name"].as_str().unwrap().to_string();
    let birthday = personal["birthday"].as_str().unwrap().to_string();
    let gender = personal["gender"].as_str().unwrap().to_string();

    let headers = vec![
        String::from("名前"),
        String::from("生年月日"),
        String::from("性別"),
    ];

    let data = vec![
        vec![
            name,
            birthday,
            gender,
        ],
    ];

    let table = render_as_table(headers, data);

    return format!("
## 個人情報
{table}
");
}

fn certificates_block(certificates: &Yaml) -> String {
    if certificates.as_vec().is_none() {
        return String::from("");
    }

    let name = certificates["name"].as_str().unwrap().to_string();
    let certified_at = certificates["certified_at"].as_str().unwrap().to_string();
    let comment = certificates["comment"].as_str().unwrap().to_string();

    let headers = vec![
        String::from("名前"),
        String::from("取得年月"),
        String::from("備考"),
    ];
    let data = vec![
        vec![
            name,
            certified_at,
            comment,
        ]
    ];

    let table = render_as_table(headers, data);

    return format!("
## 資格
{table}
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

fn projects_block(projects: &Yaml, render_table: bool) -> String {
    let projects = projects.as_vec().unwrap();
    // let mut list: Vec<String> = vec![];
    let headers = vec![
        String::from("概要"),
        String::from("期間"),
        String::from("人数"),
        String::from("ロール"),
        String::from("担当業務"),
        String::from("習得スキル・実績"),
        String::from("参画フェーズ"),
        String::from("言語・OS・DB"),
        String::from("その他ツール等"),
    ];

    let mut data: Vec<Vec<String>> = vec![];

    for (_i, p) in projects.iter().enumerate() {
        let description = p["description"].as_str().unwrap().to_string();
        let term = p["term"].as_str().unwrap().to_string();
        let members = p["members"].as_str().unwrap().to_string();
        let role = p["role"].as_str().unwrap().to_string();
        let assigned_for = p["assigned_for"].as_str().unwrap().to_string();
        let achievement = p["achievement"].as_str().unwrap().to_string();
        let phase = p["phase"].as_str().unwrap().to_string();
        let technology = p["technology"].as_str().unwrap().to_string();
        let others = p["others"].as_str().unwrap().to_string();

        let line = vec![
            description,
            term,
            members,
            role,
            assigned_for,
            achievement,
            phase,
            technology,
            others,
        ];

        let mapped = line.iter().map(|l| {
            if l == "" {
                return String::from("無し");
            }
            return String::from(l);
        }).collect();

        data.push(mapped);
    }

    return match render_table {
        true => render_as_table(headers, data),
        false => render_as_key_value_map(headers, data),
    };
}
