drop type if exists currencies cascade;  
create type currencies as enum ('mxn', 'usd', 'eur');

drop table if exists header cascade;
create table header (
    id_factura integer primary key,
    id_cliente integer,
    fecha date,
    denominacion currencies
);

drop table if exists item cascade;
create table item (
    id_item text,
    id_factura integer not null references header (id_factura),
    cantidad integer,
    antiguedad integer,
    valor_neto float
);

drop table if exists trailer cascade;
create table trailer (
    id_factura integer not null references header (id_factura),
    numero_items integer not null,
    total integer
);

drop type if exists  parserError cascade;  
create type parserError as enum(
    'TrailerError',
    'ParseInteger',
    'ParseFloat',
    'ItemError',
    'HeaderError',
    'FacturaError',
    'CurrencyError',
    'InvalidDate',
    'InvalidPath',
    'FailedDBConnection'
);

drop table if exists logs cascade;
create table logs(
   log_type parserError,
   message text,
   date timestamp
);
