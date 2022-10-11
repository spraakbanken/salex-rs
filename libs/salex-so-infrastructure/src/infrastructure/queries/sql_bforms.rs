use salex_so::application::queries::{BFormDto, GetBForms};
use async_trait::async_trait;
use crate::{DbPool, DbRow};
use sqlx::{Row, FromRow};


pub struct SqlGetBForms {
    db: DbPool,
}

impl SqlGetBForms {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl GetBForms for SqlGetBForms {
    async fn query(&self, s_nr: i32, bklass: &str) -> Vec<BFormDto> {
        let stmt = "SELECT * FROM fullform WHERE s_nr = ? AND bklass = ?";

        let mut bforms = Vec::new();
        let bforms_rows = sqlx::query(stmt)
            .bind(s_nr)
            .bind(bklass)
            .fetch_all(&self.db).await.expect("");
        // for row in bforms_rows {
        //     println!("row = {:?}", row);
        // }
        bforms
    }
}
