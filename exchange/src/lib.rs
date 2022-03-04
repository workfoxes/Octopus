
pub mod binance;
pub mod coinbase;
pub mod client;
pub mod errors;
pub mod shared;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
