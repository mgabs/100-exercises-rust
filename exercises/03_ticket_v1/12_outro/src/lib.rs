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

#[derive(Debug)]
pub struct Order {
    product_name: String,
    quantity: u64,
    unit_price: u64,
}

#[derive(Debug)]
pub enum OrderError {
    InvalidName,
    InvalidQuantity,
    InvalidPrice,
}

impl Order {
    // add code here

    pub fn verify(name: &str, quantity: u64, unit_price: u64) -> Result<(), OrderError> {
        if name.is_empty() || name.len() > 300 {
            return Err(OrderError::InvalidName);
        }
        if quantity <= 0 {
            return Err(OrderError::InvalidQuantity);
        }
        if unit_price <= 0 {
            return Err(OrderError::InvalidPrice);
        }

        Ok(())
    }
    pub fn new(name: String, quantity: u64, unit_price: u64) -> Self {
        Order::verify(&name, quantity, unit_price).unwrap();
        Order {
            product_name: name.to_string(),
            quantity,
            unit_price,
        }
    }

    pub fn total(&self) -> u64 {
        self.quantity as u64 * self.unit_price
    }

    // getters
    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn quantity(&self) -> &u64 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u64 {
        &self.unit_price
    }

    // setters
    pub fn set_product_name(&mut self, product_name: String) {
        Order::verify(&product_name, self.quantity, self.unit_price).unwrap();

        self.product_name = product_name;
    }
    pub fn set_quantity(&mut self, quantity: u64) -> () {
        Order::verify(&self.product_name, quantity, self.unit_price).unwrap();

        self.quantity = quantity;
    }
    pub fn set_unit_price(&mut self, unit_price: u64) -> () {
        Order::verify(&self.product_name, self.quantity, unit_price).unwrap();

        self.unit_price = unit_price;
    }
}

fn main() {}
