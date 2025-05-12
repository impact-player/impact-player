use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::models::{CreateOrderPayload, DepthPayload, Order, OrderSide, QuotePayload, User};

pub struct Orderbook {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
    pub base_asset: String,
    pub quote_asset: String,
}

impl Orderbook {
    pub fn new(base_asset: String, quote_asset: String) -> Self {
        Orderbook {
            bids: Vec::new(),
            asks: Vec::new(),
            base_asset,
            quote_asset,
        }
    }

    pub fn fill_orders(
        &mut self,
        order: &CreateOrderPayload,
        users: &mut Arc<Mutex<Vec<User>>>,
        base_asset: &str,
        quote_asset: &str,
    ) -> Decimal {
        let mut remaining_qty = order.quantity;

        match order.side {
            OrderSide::Bid => {
                for i in 0..self.asks.len() {
                    if self.asks[i].price > order.price {
                        break;
                    }

                    let match_qty = if self.asks[i].quantity > remaining_qty {
                        remaining_qty
                    } else {
                        self.asks[i].quantity
                    };

                    self.flip_balance(
                        &order.user_id,
                        &self.asks[i].user_id,
                        self.asks[i].price,
                        match_qty,
                        users,
                        base_asset,
                        quote_asset,
                    );

                    remaining_qty -= match_qty;
                    if self.asks[i].quantity == match_qty {
                        self.asks.remove(i);
                    } else {
                        self.asks[i].quantity -= match_qty;
                    }

                    if remaining_qty == dec!(0) {
                        break;
                    }
                }
            }
            OrderSide::Ask => {
                for i in 0..self.bids.len() {
                    if self.bids[i].price < order.price {
                        break;
                    }

                    let match_qty = if self.bids[i].quantity > remaining_qty {
                        remaining_qty
                    } else {
                        self.bids[i].quantity
                    };

                    self.flip_balance(
                        &self.bids[i].user_id,
                        &order.user_id,
                        self.bids[i].price,
                        match_qty,
                        users,
                        base_asset,
                        quote_asset,
                    );

                    remaining_qty -= match_qty;
                    if self.bids[i].quantity == match_qty {
                        self.bids.remove(i);
                    } else {
                        self.bids[i].quantity -= match_qty;
                    }

                    if remaining_qty == dec!(0) {
                        break;
                    }
                }
            }
        }

        remaining_qty
    }

    pub fn get_depth(&self) -> DepthPayload {
        let mut bids: Vec<[String; 2]> = Vec::new();
        let mut asks: Vec<[String; 2]> = Vec::new();

        let mut bid_map: HashMap<Decimal, Decimal> = HashMap::new();
        for bid in &self.bids {
            *bid_map.entry(bid.price).or_insert(Decimal::ZERO) += bid.quantity;
        }

        for (price, quantity) in bid_map {
            bids.push([price.to_string(), quantity.to_string()]);
        }

        let mut ask_map: HashMap<Decimal, Decimal> = HashMap::new();
        for ask in &self.asks {
            *ask_map.entry(ask.price).or_insert(Decimal::ZERO) += ask.quantity;
        }

        for (price, quantity) in ask_map {
            asks.push([price.to_string(), quantity.to_string()]);
        }

        bids.sort_by(|a, b| {
            let price_a: Decimal = a[0].parse().unwrap_or(Decimal::ZERO);
            let price_b: Decimal = b[0].parse().unwrap_or(Decimal::ZERO);
            price_b.cmp(&price_a)
        });

        asks.sort_by(|a, b| {
            let price_a: Decimal = a[0].parse().unwrap_or(Decimal::ZERO);
            let price_b: Decimal = b[0].parse().unwrap_or(Decimal::ZERO);
            price_a.cmp(&price_b)
        });

        DepthPayload { bids, asks }
    }

    pub fn get_quote_detail(&self, quantity: Decimal, side: OrderSide) -> QuotePayload {
        let mut remaining_qty = quantity;
        let mut total_cost = Decimal::from(0);
        let mut weighted_avg_price = Decimal::from(0);

        match side {
            OrderSide::Bid => {
                for ask in self.asks.iter() {
                    if remaining_qty == Decimal::from(0) {
                        break;
                    }

                    if remaining_qty > ask.quantity {
                        total_cost += ask.price * ask.quantity;
                        remaining_qty -= ask.quantity;
                    } else {
                        total_cost += ask.price * remaining_qty;
                        remaining_qty = Decimal::from(0);
                        break;
                    }
                }
            }
            OrderSide::Ask => {
                for bid in self.bids.iter() {
                    if remaining_qty == Decimal::from(0) {
                        break;
                    }
                    if remaining_qty >= bid.quantity {
                        total_cost += bid.price * bid.quantity;
                        remaining_qty -= bid.quantity;
                    } else {
                        total_cost += bid.price * remaining_qty;
                        remaining_qty = Decimal::from(0);
                        break;
                    }
                }
            }
        }

        if remaining_qty < quantity {
            weighted_avg_price = total_cost / (quantity - remaining_qty);
        }

        QuotePayload {
            avg_price: weighted_avg_price,
            quantity,
            total_cost,
        }
    }

    fn flip_balance(
        &self,
        buyer_id: &str,
        seller_id: &str,
        price: Decimal,
        quantity: Decimal,
        users: &mut Arc<Mutex<Vec<User>>>,
        base_asset: &str,
        quote_asset: &str,
    ) {
        let mut users_guard = users.lock().unwrap();
        let trade_value = price * quantity;

        if let Some(seller) = users_guard.iter_mut().find(|u| u.id == seller_id) {
            if let Some(base_balance) = seller.balances.iter_mut().find(|b| b.ticker == base_asset)
            {
                base_balance.locked_balance =
                    base_balance.locked_balance.checked_sub(quantity).unwrap();
                base_balance.balance = base_balance.balance.checked_sub(quantity).unwrap();
            }

            if let Some(quote_balance) =
                seller.balances.iter_mut().find(|b| b.ticker == quote_asset)
            {
                quote_balance.balance = quote_balance.balance.checked_add(trade_value).unwrap();
            }
        }

        if let Some(buyer) = users_guard.iter_mut().find(|u| u.id == buyer_id) {
            if let Some(base_balance) = buyer.balances.iter_mut().find(|b| b.ticker == base_asset) {
                base_balance.balance = base_balance.balance.checked_add(quantity).unwrap();
            }

            if let Some(quote_balance) = buyer.balances.iter_mut().find(|b| b.ticker == quote_asset)
            {
                quote_balance.locked_balance = quote_balance
                    .locked_balance
                    .checked_sub(trade_value)
                    .unwrap();
                quote_balance.balance = quote_balance.balance.checked_sub(trade_value).unwrap();
            }
        }
    }
}
