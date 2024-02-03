use entity::{links, links::Entity as Link};

use links_controller::LinkObj;
use sea_orm::{ ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::links_controller;

#[derive(Debug, Clone)]
pub struct LinksRepository {
    pub db_connection: DatabaseConnection,
}

impl LinksRepository {
    pub async fn get_links(&self) -> Result<Vec<links::Model>, sea_orm::DbErr> {
        let db_res = Link::find().all(&self.db_connection).await;
        match db_res {
            Ok(models) => Ok(models),
            Err(e) => Err(e),
        }
    }
    pub async fn save_link(
        &self,
        payload: actix_web::web::Json<LinkObj>,
    ) -> Result<links::Model, sea_orm::DbErr> {
        let existing_link = Link::find()
            .filter(links::Column::Link.eq(&payload.link))
            .one(&self.db_connection)
            .await
            .unwrap();

        match existing_link {
            Some(link) => {
                let mut links: links::ActiveModel = link.into();
                links.count = Set(links.count.unwrap() + 1);
                let links = links.update(&self.db_connection).await.unwrap();

                Ok(links)
            }
            None => {
                let new_link = links::ActiveModel {
                    link: Set(payload.link.clone()),
                    count: Set(1),
                    ..Default::default()
                };

                let save_result = new_link.insert(&self.db_connection).await;
                match save_result {
                    Ok(value) => Ok(value),
                    Err(e) => {
                        println!("the error is {}", e);
                        Err(e)
                    },
                }
            }
        }
    }
}
