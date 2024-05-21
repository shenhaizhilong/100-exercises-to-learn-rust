// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 characters.
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

pub struct Order {
    name: String,
    quantity: u32,
    price: u32,
}

impl Order {
    pub fn new(name: String, quantity: u32, price: u32) -> Order {
        Self::check_name(&name);
        Self::check_quantity(quantity);
        Self::check_price(price);
        Order {
            name: name,
            quantity: quantity,
            price: price,
        }
    }

    pub fn check_name(name: &String) {
        if (name.is_empty() || name.len() > 300) {
            panic!("invalid name");
        }
    }

    pub fn check_quantity(quantity: u32) {
        if (quantity <= 0) {
            panic!("invalid quantity");
        }
    }

    pub fn check_price(price: u32) {
        if (price <= 0) {
            panic!("invalid price");
        }
    }

    pub fn product_name(&self) -> &String {
        return &self.name;
    }

    pub fn quantity(&self) -> &u32 {
        return &self.quantity;
    }

    pub fn unit_price(&self) -> &u32 {
        return &self.price;
    }

    pub fn total(&self) -> u32 {
        return self.price * self.quantity;
    }

    pub fn set_product_name(&mut self, name: String) -> &Self {
        Self::check_name(&name);
        self.name = name;
        return self;
    }

    pub fn set_quantity(&mut self, quantity: u32) -> &Self {
        Self::check_quantity(quantity);
        self.quantity = quantity;
        return self;
    }

    pub fn set_unit_price(&mut self, price: u32) -> &Self {
        Self::check_price(price);
        self.price = price;
        return self;
    }
}