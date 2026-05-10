// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

struct Order{
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order{
    fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        if product_name.is_empty() {
            panic!("Product name cannot be empty");
        }
        if product_name.len() > 300 {
            panic!("Product name cannot be longer than 300 bytes");
        }
        if quantity == 0 {
            panic!("Quantity must be greater than zero");
        }
        if unit_price == 0 {
            panic!("Unit price must be greater than zero");
        }
        Self {
            product_name,
            quantity,
            unit_price,
        }
    }

    fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    fn product_name(&self) -> &String {
        &self.product_name
    }

    fn quantity(&self) -> u32 {
        self.quantity
    }

    fn unit_price(&self) -> u32 {
        self.unit_price
    }

    fn set_product_name(&mut self, product_name: String) {
        if product_name.is_empty() {
            panic!("Product name cannot be empty");
        }
        if product_name.len() > 300 {
            panic!("Product name cannot be longer than 300 bytes");
        }
        self.product_name = product_name;
    }

    fn set_quantity(&mut self, quantity: u32) {
        if quantity == 0 {
            panic!("Quantity must be greater than zero");
        }
        self.quantity = quantity;
    }

    fn set_unit_price(&mut self, unit_price: u32) {
        if unit_price == 0 {
            panic!("Unit price must be greater than zero");
        }
        self.unit_price = unit_price;
    }
}

// Test code here
#[cfg(test)]
mod tests {
    use super::Order;

    #[test]
    fn total() {
        let order = Order::new("Product".into(), 2, 100);
        assert_eq!(order.total(), 200);
    }

    #[test]
    fn getters() {
        let order = Order::new("Product".into(), 2, 100);
        assert_eq!(order.product_name(), "Product");
        assert_eq!(order.quantity(), 2);
        assert_eq!(order.unit_price(), 100);
    }

    #[test]
    fn setters() {
        let mut order = Order::new("Product".into(), 2, 100);
        order.set_product_name("New Product".into());
        order.set_quantity(3);
        order.set_unit_price(150);
        assert_eq!(order.product_name(), "New Product");
        assert_eq!(order.quantity(), 3);
        assert_eq!(order.unit_price(), 150);
    }
}