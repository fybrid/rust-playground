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
        format!("${}.{:02}", self.0 / 100, self.0 % 100)
    }
}

pub fn cart_total(items: &[CartItem]) -> MoneyCents {
    MoneyCents(
        items
            .iter()
            .map(|item| item.product.price.0 * item.quantity)
            .sum(),
    )
}

pub fn next_light(light: TrafficLight) -> TrafficLight {
    match light {
        TrafficLight::Red => TrafficLight::Green,
        TrafficLight::Green => TrafficLight::Yellow,
        TrafficLight::Yellow => TrafficLight::Red,
    }
}

pub fn response_summary(response: ApiResponse) -> String {
    match response {
        ApiResponse::Success { status, body } => format!("success {status}: {body}"),
        ApiResponse::NotFound { resource } => format!("not found: {resource}"),
        ApiResponse::RateLimited {
            retry_after_seconds,
        } => format!("rate limited: retry after {retry_after_seconds} seconds"),
    }
}

