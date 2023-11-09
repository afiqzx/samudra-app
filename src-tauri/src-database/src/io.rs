#![allow(dead_code)]

use sea_orm::{DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};

// TODO Implement Import From CSV for LemmaWithKonsepView
#[derive(Debug, Serialize, Deserialize)]
pub struct RowValue {
    pub lemma: String,
    pub konsep: String,
}

impl RowValue {
    pub async fn insert(&self, _db: &DatabaseConnection) -> Result<(), DbErr> {
        // let lemma_am = models::lemma::ActiveModel {
        //     nama: ActiveValue::Set(self.lemma.to_owned()),
        //     ..Default::default()
        // };
        // let lemma = lemma_am
        //     .clone()
        //     .insert_with_check(models::lemma::Column::Id, db)
        //     .await?;
        // let konsep = models::konsep::ActiveModel {
        //     lemma_id: ActiveValue::Set(lemma.id),
        //     golongan_id: ActiveValue::Set(None),
        //     keterangan: ActiveValue::Set(Some(self.konsep.to_owned())),
        //     ..Default::default()
        // };
        // konsep
        //     .clone()
        //     .insert_with_check(models::konsep::Column::Id, db)
        //     .await?;
        // Ok(())
        todo!();
    }
}

fn read_csv(
    path: &std::path::Path,
    delimiter: Option<u8>,
    terminator: Option<u8>,
) -> Result<Vec<RowValue>, csv::Error> {
    let delim = delimiter.unwrap_or(b',');
    let term = match terminator {
        Some(c) => csv::Terminator::Any(c),
        None => csv::Terminator::CRLF,
    };
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delim)
        .terminator(term)
        .trim(csv::Trim::All)
        .from_path(path)?;

    rdr.deserialize().into_iter().collect()
}

pub async fn import_from_csv(
    path: &std::path::Path,
    delimiter: Option<u8>,
    terminator: Option<u8>,
    db: &DatabaseConnection,
) -> Result<String, DbErr> {
    let mut count: i128 = 0;
    let data = dbg!(read_csv(path, delimiter, terminator).unwrap());
    for d in data.iter() {
        d.insert(db).await?;
        count += 1
    }
    Ok(format!(
        "{} items imported from {}.",
        count,
        path.as_os_str().to_str().unwrap()
    ))
}
