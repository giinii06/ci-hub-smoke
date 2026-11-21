pub fn ready() -> bool { true }

#[cfg(test)]
mod tests {
    #[test]
    fn ready() { assert!(super::ready()); }
}
