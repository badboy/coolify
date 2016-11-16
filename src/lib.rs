//! Coolify everything
//!
//! Example
//!
//! ```rust
//! assert_eq!("cool", coolify::to_cool("rust"));
//! ```

/// Coolify every value.
pub fn to_cool<T>(_val: T) -> &'static str {
    "cool"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("cool", to_cool("rust"));
    }
}
