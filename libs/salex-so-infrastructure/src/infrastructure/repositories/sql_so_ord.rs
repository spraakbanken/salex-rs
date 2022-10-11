use async_trait::async_trait;
use salex_so::application::repositories::SoOrdRepository;
use salex_so::domain::entities::{SoOrd, SoLemmaRef};
use crate::{DbPool, DbRow};
use sqlx::{Row, FromRow};

pub struct SqlSoOrdRepository {
    db: DbPool,
}

impl SqlSoOrdRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }
}

#[derive(Debug, serde::Deserialize)]
struct Extra {
    lemma_referenser: Vec<SoLemmaRef>,
}
struct SoOrdWrapper(SoOrd);

impl SoOrdWrapper {
    fn into_inner(self) -> SoOrd {
        self.0
    }
}

impl FromRow<'_, DbRow> for SoOrdWrapper {
    fn from_row(row: &DbRow) -> sqlx::Result<Self> {
        let extra: Extra = serde_json::from_str(row.try_get("extras")?).expect("");
        let Extra { lemma_referenser } = extra;
        Ok(Self(
            SoOrd {
                ortografi: row.try_get("ortografi")?,
                s_nr: row.try_get("s_nr")?,
                lemmaundertyp: row.try_get("lemmaundertyp")?,
                sorteringsform: row.try_get("sorteringsform")?,
                lm_sabob: row.try_get("lm_sabob")?,
                böjningsklass: row.try_get("böjningsklass")?,
                lemma_referenser,
            }
        ))
    }
}

#[async_trait]
impl SoOrdRepository for SqlSoOrdRepository {
    async fn get(&self, l_nr: u32) -> SoOrd {
        let query = "SELECT *
        FROM so_ord
        WHERE l_nr = ?";
        let so_ord: SoOrdWrapper = sqlx::query_as(query)
            .bind(l_nr)
            .fetch_one(&self.db)
            .await
            .expect("error");
        //      {
        //     Ok(x) => {
        //         tracing::debug!("x = {:?}", x);
        //         todo!()
        //     },
        //     Err(err) => {
        //         tracing::error!("error occurred: {:?}", err);
        //         todo!()
        //     }
        // }
        so_ord.into_inner()
    }

    async fn has_heteronym(&self, lemma: &SoOrd) -> bool {
        let stmt = "
            SELECT COUNT(*) AS num_rows
            FROM so_ord
            WHERE ortografi = ?
            ";
            // COLLATE 'utf8mb4_bin'
            // ";

        let num_heteronyms = self.num_heteronyms(lemma).await;

        num_heteronyms > 1
    }
}
impl SqlSoOrdRepository {

    async fn num_heteronyms(&self, lemma: &SoOrd) -> u32 {
        let stmt = "
            SELECT ortografi
            FROM so_ord
            WHERE ortografi = ?
            ";
            // COLLATE 'utf8mb4_bin'
            // ";

        let heteronyms = sqlx::query(stmt)
            .bind(&lemma.ortografi)
            .fetch_all(&self.db)
            .await
            .expect("sql_so_ord.num_heteronyms: error");
        let mut num_heteronyms = 0;
        for heteronym in heteronyms {
            let value: String = heteronym.get(0);
            if value == lemma.ortografi {
                num_heteronyms += 1;
            }
        }
        num_heteronyms
    }
}
