#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoneyCents(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    pub name: String,
    pub price: MoneyCents,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CartItem {
    pub product: Product,
    pub quantity: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiResponse {
    Success { status: u16, body: String },
    NotFound { resource: String },
    RateLimited { retry_after_seconds: u64 },
}

impl MoneyCents {
    pub fn format(self) -> String {
        todo!("format cents like $12.34")
    }
}

pub fn cart_total(items: &[CartItem]) -> MoneyCents {
    todo!("sum product price times quantity")
}

pub fn next_light(light: TrafficLight) -> TrafficLight {
    todo!("return the next traffic light state")
}

pub fn response_summary(response: ApiResponse) -> String {
    todo!("summarize each API response variant")
}

