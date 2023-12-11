use itertools::Itertools;

/// Returns the result of parsing [s] as a u32.
pub fn parse_u32(s: &str) -> Option<u32> {
    s.parse().ok()
}

/// Returns a new vector containing each space-delimited u32 in [s].
pub fn parse_u32_list(s: &str) -> Vec<u32> {
    s.split_whitespace().flat_map(parse_u32).collect_vec()
}

/// Returns the result of parsing [s] as a u64.
pub fn parse_u64(s: &str) -> Option<u64> {
    s.parse().ok()
}

/// Returns a new vector containing each space-delimited u64 in [s].
pub fn parse_u64_list(s: &str) -> Vec<u64> {
    s.split_whitespace().flat_map(parse_u64).collect_vec()
}
