use super::teacher_model as teacher;
use sea_orm::{prelude::*, Set};

pub async fn all_teachers(conn: &DbConn) -> Result<Vec<teacher::Model>, DbErr> {
    teacher::Entity::find()
        .all(conn)
        .await
}

pub async fn create_teacher(
    conn: &DbConn,
    teacher_body: teacher::Model,
) -> Result<teacher::ActiveModel, DbErr> {
    teacher::ActiveModel {
        name: Set(teacher_body.name),
        description: Set(teacher_body.description),
        ..Default::default()
    }
    .save(conn)
    .await
}