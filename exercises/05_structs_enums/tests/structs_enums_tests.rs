use exercise_05_structs_enums::{
    cart_total, next_light, response_summary, ApiResponse, CartItem, MoneyCents, Product,
    TrafficLight,
};

#[test]
fn formats_money() {
    assert_eq!(MoneyCents(0).format(), "$0.00");
    assert_eq!(MoneyCents(7).format(), "$0.07");
    assert_eq!(MoneyCents(1234).format(), "$12.34");
}

#[test]
fn calculates_cart_total() {
    let items = vec![
        CartItem {
            product: Product {
                name: "Book".to_string(),
                price: MoneyCents(1500),
            },
            quantity: 2,
        },
        CartItem {
            product: Product {
                name: "Pen".to_string(),
                price: MoneyCents(250),
            },
            quantity: 3,
        },
    ];

    assert_eq!(cart_total(&items), MoneyCents(3750));
}

#[test]
fn advances_traffic_light() {
    assert_eq!(next_light(TrafficLight::Red), TrafficLight::Green);
    assert_eq!(next_light(TrafficLight::Green), TrafficLight::Yellow);
    assert_eq!(next_light(TrafficLight::Yellow), TrafficLight::Red);
}

#[test]
fn summarizes_api_responses() {
    assert_eq!(
        response_summary(ApiResponse::Success {
            status: 200,
            body: "ok".to_string()
        }),
        "success 200: ok"
    );
    assert_eq!(
        response_summary(ApiResponse::NotFound {
            resource: "/users/9".to_string()
        }),
        "not found: /users/9"
    );
    assert_eq!(
        response_summary(ApiResponse::RateLimited {
            retry_after_seconds: 30
        }),
        "rate limited: retry after 30 seconds"
    );
}

