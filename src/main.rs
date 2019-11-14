use std::error::Error;
use std::ops::Add;

mod members;

fn main() -> Result<(), Box<dyn Error>> {
    use members::*;

    let members: Members
        = reqwest::get("https://raw.githubusercontent.com/AmusementCreators/WebSite/master/data/members.json")?
        .json()?;

    let Members(members) = members;

    struct Sum {
        active: usize,
        alumni: usize,
    }

    /// Sumに足し算を定義する
    impl Add for Sum {
        type Output = Sum;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                active: self.active + rhs.active,
                alumni: self.alumni + rhs.alumni,
            }
        }
    }

    /// 総和の計算方法を定義する
    impl std::iter::Sum for Sum {
        fn sum<I: Iterator<Item=Sum>>(iter: I) -> Self {
            iter.fold(Sum { active: 0, alumni: 0 }, |sum, i| sum + i)
        }
    }

    let Sum { active, alumni } = members.iter()
        // それぞれのクラスタから数値を読み取る
        .map(|c| match c {
            Cluster::Active { members, .. } => Sum { active: members.len(), alumni: 0 },
            Cluster::Alumni { members, .. } => Sum { active: 0, alumni: members.len() }
        })
        // 総和を取る
        .sum();

    println!("Active: {}, Alumni: {}", active, alumni);

    Ok(())
}