use crate::entities::teacher;
use sea_orm::{prelude::*, Set};

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