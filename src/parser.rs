use std::str::FromStr;

use crate::types::{
    Currencies, Factura, Header, Item, ParserError, ParserErrorType, Ticket, Trailer,
};

impl FromStr for Currencies {
    type Err = ParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "USD" => Ok(Currencies::Usd),
            "MXN" => Ok(Currencies::Mxn),
            "EUR" => Ok(Currencies::Eur),
            _ => Err(ParserError::new(
                ParserErrorType::CurrencyError,
                "Currency",
                s,
            )),
        }
    }
}
impl FromStr for Ticket {
    type Err = ParserError;
    fn from_str(st: &str) -> Result<Self, Self::Err> {
        let s = st.split('\n').collect::<Vec<_>>();

        let (mut ptr, mut chunks, mut prev) = (0, vec![], 0);

        while ptr < s.len() {
            if s[ptr].as_bytes()[0] == b'T' {
                let bind = &s[prev..=ptr];
                chunks.push(bind.join("\n"));
                prev = ptr + 1;
            }
            ptr += 1;
        }

        let facturas = chunks.iter().map(|f| f.parse::<Factura>()).collect();

        Ok(Ticket { facturas })
    }
}

impl FromStr for Factura {
    type Err = ParserError;
    fn from_str(st: &str) -> Result<Self, Self::Err> {
        let s = st.split('\n').collect::<Vec<_>>();

        if s.len() < 2 {
            return Err(ParserError::new(
                ParserErrorType::FacturaError,
                "3 arguments",
                &format!("{} in Factura", s.len()),
            ));
        }

        if s.len() == 2 {
            let header = s[0].parse::<Header>()?;
            let trailer = s[s.len() - 1].parse::<Trailer>()?;
            return Ok(Factura {
                header,
                items : vec![],
                trailer,
            })
        }

        let header = s[0].parse::<Header>()?;
        let items = s[1..s.len() - 1]
            .iter()
            .map(|e| e.parse::<Item>())
            .collect::<Result<_, _>>()?;
        let trailer = s[s.len() - 1].parse::<Trailer>()?;

        Ok(Factura {
            header,
            items,
            trailer,
        })
    }
}

impl FromStr for Trailer {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.as_bytes()[0] != b'T' {
            return Err(ParserError {
                kind: ParserErrorType::TrailerError,
                message: format!("Expected T found {} in Trailer", s.as_bytes()[0]),
            });
        }

        let parts = &s.split(' ').collect::<Vec<_>>();

        if parts.len() != 3 {
            return Err(ParserError {
                kind: ParserErrorType::TrailerError,
                message: format!("Expectet 3 arguments found {} in Trailer", parts.len()),
            });
        }

        let numero_de_items = parse::<u32>(parts[1])?;
        let valor_total = parse::<f32>(parts[2])?;
        Ok(Trailer {
            numero_de_items,
            valor_total,
        })
    }
}

impl FromStr for Item {
    type Err = ParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.as_bytes()[0] != b'I' {
            return Err(ParserError {
                kind: ParserErrorType::ItemError,
                message: format!("Expected I found {} in Item", (s.as_bytes()[0] as char)),
            });
        }
        let parts = &s.split(' ').collect::<Vec<_>>();

        if parts.len() != 4 {
            return Err(ParserError {
                kind: ParserErrorType::ItemError,
                message: format!("Expected 4 arguments found {} in Item", parts.len()),
            });
        }

        Ok(Item {
            id: parts[0][1..].to_string(),
            antiguedad: parse::<u32>(parts[1])?,
            cantidad: parse::<u32>(parts[2])?,
            valor_neto: parse::<f32>(parts[3])?,
        })
    }
}

impl FromStr for Header {
    type Err = ParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use chrono::prelude::*;

        if s.as_bytes()[0] != b'H' {
            return Err(ParserError {
                kind: ParserErrorType::HeaderError,
                message: format!("Expected H found {} in Header", s.as_bytes()[0]),
            });
        }

        let fecha = NaiveDate::parse_from_str(&s[15..=22], "%Y%m%d").map_err(|_| ParserError {
            kind: ParserErrorType::InvalidDate,
            message: format!("Expected %Y%m%d found {}", &s[15..=22]),
        })?;

        Ok(Header {
            numero_de_factura: parse::<i32>(&s[4..=8])?,
            id_cliente: parse::<i32>(&s[10..=12])?,
            fecha,
            denominacion: s[23..].parse::<Currencies>()?,
        })
    }
}

trait ParserErrorMessage {
    fn message(s: &str) -> ParserError;
}

impl ParserErrorMessage for i32 {
    fn message(s: &str) -> ParserError {
        ParserError::new(ParserErrorType::ParseFloat, "i32", s)
    }
}

impl ParserErrorMessage for u32 {
    fn message(s: &str) -> ParserError {
        ParserError::new(ParserErrorType::ParseFloat, "u32", s)
    }
}

impl ParserErrorMessage for f32 {
    fn message(s: &str) -> ParserError {
        ParserError::new(ParserErrorType::ParseFloat, "f32", s)
    }
}

fn parse<T: FromStr + ParserErrorMessage>(s: &str) -> Result<T, ParserError> {
    s.parse::<T>().map_err(|_| T::message(s))
}
