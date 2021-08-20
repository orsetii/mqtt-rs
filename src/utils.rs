
#[inline(always)]
fn is_not_space(c: u8) -> bool { ! nom::character::is_space(c) }
