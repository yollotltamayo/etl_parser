#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "currencies", rename_all = "lowercase")]
pub enum Currencies {
    Mxn,
    Usd,
    Eur,
}

#[derive(Debug)]
pub struct Ticket {
    pub facturas: Vec<Result<Factura, ParserError>>,
}
use chrono::NaiveDate;

#[derive(Debug)]
pub struct Factura {
    pub header: Header,
    pub items: Vec<Item>,
    pub trailer: Trailer,
}

#[derive(Debug)]
pub struct Trailer {
    pub numero_de_items: u32,
    pub valor_total: f32,
}

#[derive(Debug)]
pub struct Item {
    pub id: String,
    pub antiguedad: u32,
    pub cantidad: u32,
    pub valor_neto: f32,
}

#[derive(Debug)]
pub struct Header {
    pub numero_de_factura: i32,
    pub id_cliente: i32,
    pub fecha: NaiveDate,
    pub denominacion: Currencies,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "parserError")]
pub enum ParserErrorType {
    TrailerError,
    ParseInteger,
    ParseFloat,
    ItemError,
    HeaderError,
    FacturaError,
    CurrencyError,
    InvalidDate,
    InvalidPath,
    FailedDBConnection,
    InvalidNumberOfArguments
}

#[derive(Debug)]
pub struct ParserError {
    pub kind: ParserErrorType,
    pub message: String,
}

impl ParserError {
    pub fn new(kind: ParserErrorType, expected: &str, found: &str) -> Self {
        Self {
            kind,
            message: format!("Expected {expected} found {found}"),
        }
    }
}
