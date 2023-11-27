use crate::{common::util::id::GlobalIdGenerator, model::to::member::MemberTo};
use sqlx::{MySqlPool, QueryBuilder};

// ------ common begin ------
// upsert
pub async fn upsert(cp: &MySqlPool, record: &mut MemberTo) -> bool {
    let result = if let None = record.member_id {
        record.member_id = Some(GlobalIdGenerator::generate());
        let sql = r#"
            INSERT INTO member (member_id, nickname, password, email)
            VALUES (?, ?, ?, ?)
        "#;
        sqlx::query(sql)
            .bind(record.member_id)
            .bind(record.nickname.clone())
            .bind(record.password.clone())
            .bind(record.email.clone())
            .execute(cp)
            .await
            .unwrap()
    } else {
        let sql = "UPDATE member SET nickname = ?, password = ?, email = ? WHERE member_id = ?";
        sqlx::query(sql)
            .bind(record.nickname.clone())
            .bind(record.password.clone())
            .bind(record.email.clone())
            .bind(record.member_id)
            .execute(cp)
            .await
            .unwrap()
    };
    result.rows_affected() == 1
}

// delete
pub async fn truncate(cp: &MySqlPool) {
    let _ = sqlx::query("TRUNCATE TABLE member")
        .execute(cp)
        .await
        .unwrap();
}

pub async fn delete_by_id(cp: &MySqlPool, member_id: u64) -> bool {
    let count = sqlx::query("DELETE FROM member WHERE member_id = ?")
        .bind(member_id)
        .execute(cp)
        .await
        .unwrap()
        .rows_affected();
    count == 1
}

// select
pub async fn count(cp: &MySqlPool) -> usize {
    let row: (i64,) = sqlx::query_as("SELECT count(*) FROM member")
        .fetch_one(cp)
        .await
        .unwrap();
    row.0 as usize
}

pub async fn find_all(cp: &MySqlPool) -> Vec<MemberTo> {
    sqlx::query_as("SELECT * FROM member")
        .fetch_all(cp)
        .await
        .unwrap()
}

pub async fn exist_by_id(cp: &MySqlPool, member_id: u64) -> bool {
    let row: (i64,) = sqlx::query_as("SELECT count(*) FROM member WHERE member_id = ?")
        .bind(member_id)
        .fetch_one(cp)
        .await
        .unwrap();
    row.0 > 0
}

pub async fn find_by_id(cp: &MySqlPool, member_id: u64) -> Option<MemberTo> {
    sqlx::query_as("SELECT * FROM member WHERE member_id = ?")
        .bind(member_id)
        .fetch_optional(cp)
        .await
        .unwrap()
}

pub async fn list_by_id(cp: &MySqlPool, ids: &Vec<u64>) -> Vec<MemberTo> {
    if ids.is_empty() {
        return Default::default();
    }
    QueryBuilder::new("SELECT * FROM member WHERE member_id IN")
        .push_tuples(ids.iter(), |mut b, id| {
            b.push_bind(id);
        })
        .build_query_as()
        .fetch_all(cp)
        .await
        .unwrap()
}
// ------ common end ------

pub async fn find_by_email(cp: &MySqlPool, email: &str) -> Option<MemberTo> {
    sqlx::query_as("SELECT * FROM member WHERE email = ?")
        .bind(email)
        .fetch_optional(cp)
        .await
        .unwrap()
}
