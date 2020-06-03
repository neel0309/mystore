use crate::models::price::{Price, PriceList};
use crate::models::product::{FullProduct, ListProduct, Product};
use crate::models::sale::{FormSale, FullSale, ListSale, Sale};
use crate::models::Context;
use juniper::FieldResult;

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn listSale(context: &Context, search: Option<FormSale>, limit: i32) -> FieldResult<ListSale> {
        Sale::list(context, search, limit)
    }

    fn showSale(context: &Context, sale_id: i32) -> FieldResult<FullSale> {
        Sale::show(context, sale_id)
    }

    fn listProduct(
        context: &Context,
        search: String,
        limit: i32,
        rank: f64,
    ) -> FieldResult<ListProduct> {
        Product::list(context, search, limit, rank)
    }

    fn showProduct(context: &Context, product_id: i32) -> FieldResult<FullProduct> {
        Product::show(context, product_id)
    }

    fn ListPrice(context: &Context) -> FieldResult<PriceList> {
        PriceList::list(context)
    }

    fn findPrice(context: &Context, price_id: i32) -> FieldResult<Price> {
        Price::find(context, price_id)
    }
}
