use crate::types::*;
pub fn validate_ticket(ticket: Ticket) -> Result<Ticket, Vec<ParserError>> {

    let mut errors = vec![];
    for factura in ticket.facturas .iter().filter_map(|factura| factura.as_ref().ok()) {
        if let Err(e) = validate_factura(factura) {
            errors.push(e);
        }
    }
    if errors.is_empty() {
        Ok(ticket)
    } else {
        Err(errors)
    }
}
pub fn validate_factura(factura: &Factura) -> Result<(), ParserError> {
    let suma_de_valor_neto = factura
        .items
        .iter()
        .fold(0 as f32, |acc, item| acc +  item.valor_neto );

    if !(suma_de_valor_neto == factura.trailer.valor_total ) {
        return Err(ParserError::new(
            ParserErrorType::ItemSumNotEqual,
            &factura.trailer.valor_total.to_string(),
            &suma_de_valor_neto.to_string()
        ))
    }
    if !(factura.items.len() == factura.trailer.numero_de_items as usize) {
        return Err(ParserError::new(
            ParserErrorType::NotSameItems,
            &factura.trailer.numero_de_items.to_string(),
            &factura.items.len().to_string()
        ))
    }

    Ok(())
}
