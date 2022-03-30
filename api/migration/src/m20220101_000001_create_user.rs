use sea_schema::migration::prelude::*;

#[derive(Debug)]
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_user"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        use entity::user;

        manager
            .create_table(
                sea_query::Table::create()
                    .table(user::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user::Column::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(user::Column::Name).string().not_null())
                    .col(ColumnDef::new(user::Column::Email).string().not_null())
                    .col(ColumnDef::new(user::Column::Password).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        use entity::user;

        manager
            .drop_table(sea_query::Table::drop().table(user::Entity).to_owned())
            .await
    }
}
