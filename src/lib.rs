pub fn wowo2(x: u32, y: u32) -> u32 {
    return x + y;
}
pub mod com {
    fn wohaha(x: u32, y: u32) -> u32 {
        return x + y;
    }
    pub fn wowo(x: u32, y: u32) -> u32 {
        return x + y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, com::wowo(2, 2));
        // private function
        // assert_eq!(4, com::wohaha(2, 2));
    }
}

// Caused by:
//   no targets specified in the manifest
//   either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present
