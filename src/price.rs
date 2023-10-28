struct Price {}

trait Price {
    fn get_price(&self) -> u32;
    fn get_formula(&self) -> String;
}
