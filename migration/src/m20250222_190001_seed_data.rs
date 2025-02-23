use entity::*;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::entity::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Insert bowls
        let bowls = vec![
            ("Teriyaki Chicken Bowl", 13.99),
            ("Salmon Poke Bowl", 15.99),
            ("Veggie Buddha Bowl", 12.99),
            ("Korean BBQ Bowl", 14.99),
        ];

        // Insert bowls and store their IDs
        for (name, price) in bowls {
            bowl::ActiveModel {
                name: Set(name.into()),
                price: Set(price.into()),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }

        let results = bowl::Entity::find().all(db).await?;

        // Insert customizations for each bowl
        for row in results {
            let bowl_id: i32 = row.id;
            let bowl_name: String = row.name;

            // Define customizations based on bowl type
            let customizations = match bowl_name.as_str() {
                "Teriyaki Chicken Bowl" => vec![
                    ("Extra Chicken", 3.50),
                    ("Avocado", 2.00),
                    ("Extra Teriyaki Sauce", 0.50),
                ],
                "Salmon Poke Bowl" => vec![
                    ("Extra Salmon", 4.00),
                    ("Masago", 1.50),
                    ("Spicy Mayo", 0.50),
                ],
                "Veggie Buddha Bowl" => vec![
                    ("Extra Quinoa", 2.00),
                    ("Extra Avocado", 2.00),
                    ("Extra Roasted Veggies", 1.50),
                ],
                "Korean BBQ Bowl" => vec![
                    ("Extra Bulgogi", 3.50),
                    ("Kimchi", 1.50),
                    ("Gochujang Sauce", 0.50),
                ],
                _ => vec![],
            };

            for (name, price) in customizations {
                customization::ActiveModel {
                    bowl_id: Set(bowl_id),
                    name: Set(name.into()),
                    price: Set(price.into()),
                    ..Default::default()
                }
                .insert(db)
                .await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        customization::Entity::delete_many().exec(db).await?;
        bowl::Entity::delete_many().exec(db).await?;

        Ok(())
    }
}
