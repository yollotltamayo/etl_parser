use anyhow::Result;
use sqlx::{postgres::PgPool, Postgres};

use futures::future::join_all;

use crate::types::{Factura, Header, Item, ParserError, ParserErrorType, Ticket, Trailer};
pub const DATABASE_URL: &str = "postgres://yollotl@localhost:5432/parser";

pub async fn insert_into_db(ticket: &Ticket) -> Result<(), ParserError> {
    let db = get_db_connection()
        .await
        .map_err(|_| ParserError::new(ParserErrorType::InvalidPath, "", ""))?;

    let Ticket { facturas } = ticket;
    for factura in facturas.iter() {
        match factura {
            Ok(factura) => {
                let Factura {
                    header,
                    items,
                    trailer,
                } = factura;
                let Header {
                    numero_de_factura,
                    id_cliente,
                    fecha,
                    denominacion,
                } = header;
                let Trailer {
                    numero_de_items,
                    valor_total,
                } = trailer;

                let mut queries = vec![];
                queries.push(sqlx::query!(
                    "insert into header 
                        (id_factura, id_cliente, fecha, denominacion)
                        values ( $1 , $2 , $3 , $4 )",
                    numero_de_factura,
                    id_cliente,
                    fecha,
                    denominacion as _
                ));

                queries.push(sqlx::query!(
                    "insert into trailer 
                        (id_factura, numero_items,total)
                        values ( $1 , $2 , $3 )",
                    numero_de_factura,
                    *numero_de_items as i32,
                    *valor_total as i32
                ));

                for Item {
                    id,
                    antiguedad,
                    cantidad,
                    valor_neto,
                } in items.iter()
                {
                    queries.push(sqlx::query!(
                        "insert into item 
                            (id_item, id_factura, cantidad, antiguedad, valor_neto)
                            values ( $1 , $2 , $3 , $4, $5)",
                        id,
                        numero_de_factura,
                        *cantidad as i32,
                        *antiguedad as f64,
                        *valor_neto as f64,
                    ));
                }

                let future_queries = queries
                    .into_iter()
                    .map(|query| query.execute(&db))
                    .collect::<Vec<_>>();

                let res = join_all(future_queries).await;
                println!("{:?}", res);
            }
            Err(err) => {
                sqlx::query!(
                    "insert into logs 
                    (log_type, message , date)
                    values ( $1 , $2 , $3)",
                    err.kind as _,
                    err.message,
                    chrono::Utc::now().naive_utc() as _
                )
                .execute(&db)
                .await
                .map_err(|e| {
                    dbg!(e);
                    ParserError::new(ParserErrorType::FailedDBConnection, "", "")
                })?;
            }
        }
    }
    Ok(())
}
pub fn init_db() -> Result<()> {
    let mut child = std::process::Command::new("psql")
        .args(vec!["-d", "parser", "-f", "sql/create_database.sql"])
        .spawn()?;
    child.wait()?;
    Ok(())
}

async fn get_db_connection() -> Result<sqlx::Pool<Postgres>, ParserError> {
    let pool = PgPool::connect(DATABASE_URL)
        .await
        .map_err(|_| ParserError::new(ParserErrorType::FailedDBConnection, "", ""))?;
    Ok(pool)
}
