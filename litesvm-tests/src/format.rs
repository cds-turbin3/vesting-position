//! Human-readable token amounts for the markdown lifecycle reports.

pub const TEST_MINT_DECIMALS: u8 = 6;

fn format_with_commas(n: u128) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// Render a base-unit balance as a decimal token amount (e.g. `100,000`).
pub fn format_tokens(amount: u64) -> String {
    let scale = 10u128.pow(TEST_MINT_DECIMALS as u32);
    let whole = amount as u128 / scale;
    let frac = amount as u128 % scale;
    if frac == 0 {
        format_with_commas(whole)
    } else {
        let frac_str = format!("{:0width$}", frac, width = TEST_MINT_DECIMALS as usize);
        let frac_trimmed = frac_str.trim_end_matches('0');
        format!("{}.{}", format_with_commas(whole), frac_trimmed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_whole_tokens_with_commas() {
        assert_eq!(format_tokens(100_000_000_000), "100,000");
        assert_eq!(format_tokens(0), "0");
    }
}
