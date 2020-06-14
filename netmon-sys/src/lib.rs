extern "C" {
    pub fn get_tcp_network_table() -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tcp_network_table() {
        assert_eq!(true, unsafe { get_tcp_network_table() });
    }
}
