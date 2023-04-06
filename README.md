# Dynamic Pricing Server for Online Clothing Store

This is a server that calculates dynamic pricing for items sold on an online clothing store. The server is written in Rust and uses the Axum library as the HTTP server.

The server implements an API endpoint that calculates the optimal price for each item based on various input factors. These factors include the number of orders made in the past hour, the number of page views in the past hour, a multiplier set by the supplier, and the number of remaining items to sell.

## Installation

To run the server, you will need to have Rust and Cargo installed on your machine. You can download Rust from the official website at <https://www.rust-lang.org/tools/install>.

Once you have Rust and Cargo installed, you can clone this repository to your local machine using the following command:

```bash
git clone https://github.com/noahlozevski/surge-pricing.git
```

After cloning the repository, navigate to the project directory and build the server using the following command:

```rust
cargo build --release
```

This will create an optimized binary in the target/release/ directory.

Usage
To start the server, run the following command:

```bash
./target/release/dynamic-pricing-server
```

This will start the server on port 12345 of your localhost.

To calculate the price of an item, send a GET request to the following endpoint:

```bash
http://localhost:12345/calculate-surge/with-this-supply/<supply>/and-this-demand/<demand>
```

Replace <supply> and <demand> with the appropriate values for the item you want to price. The server will respond with a JSON object containing the calculated price for the item.

## Technical Details

The server uses the Axum crate to implement an HTTP API that handles requests for surge pricing. The calculate_surge function handles the surge pricing request, taking in the supply and demand values as parameters. These values are used to calculate the optimal price for the item based on the formula:

```
price = (supply / demand) * supplier_multiplier * internal_multiplier
```

where supplier_multiplier is a multiplier set by the supplier and internal_multiplier is a constant internal multiplier set within the server.

The calculate_multiplier function calculates the multiplier based on the supply and demand values.

## Contributing

If you find a bug or have a feature request, please open an issue on the GitHub repository. Pull requests are welcome!
