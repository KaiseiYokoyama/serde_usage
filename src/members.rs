use serde::{Serialize, Deserialize};

/// members.json の内容すべて
#[derive(Serialize,Deserialize,Debug)]
pub struct Members(pub Vec<Cluster>);

/// 入部年度ごとのメンバーの集合
#[derive(Serialize,Deserialize,Debug)]
#[serde(untagged)]
pub enum Cluster {
    /// 現役
    #[serde(rename_all = "PascalCase")]
    Active {
        year: u32,
        members: Vec<Member>,
    },
    /// 卒業生
    #[serde(rename_all = "PascalCase")]
    Alumni {
        title: String,
        members: Vec<Member>,
    },
}

#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Member {
    name: String,
    description: String,
    thumbnail: Option<String>,
    tags: Option<Vec<String>>,
    #[serde(rename="SNS")]
    sns: Option<SNS>,
}

#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SNS {
    twitter: Option<String>,
    github: Option<String>,
    link: Option<Link>,
}

#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Link {
    url: String,
    title: String,
}