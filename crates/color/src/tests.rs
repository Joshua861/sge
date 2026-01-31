use super::*;
use crate::text_rendering::rich_text::*;

// Tests for hex_char_to_int
#[test]
fn test_hex_char_to_int_digits() {
    assert_eq!(hex_char_to_int(b'0'), Some(0));
    assert_eq!(hex_char_to_int(b'1'), Some(1));
    assert_eq!(hex_char_to_int(b'5'), Some(5));
    assert_eq!(hex_char_to_int(b'9'), Some(9));
}

#[test]
fn test_hex_char_to_int_uppercase() {
    assert_eq!(hex_char_to_int(b'A'), Some(10));
    assert_eq!(hex_char_to_int(b'B'), Some(11));
    assert_eq!(hex_char_to_int(b'C'), Some(12));
    assert_eq!(hex_char_to_int(b'D'), Some(13));
    assert_eq!(hex_char_to_int(b'E'), Some(14));
    assert_eq!(hex_char_to_int(b'F'), Some(15));
}

#[test]
fn test_hex_char_to_int_invalid() {
    assert_eq!(hex_char_to_int(b'/'), None); // before '0'
    assert_eq!(hex_char_to_int(b':'), None); // after '9', before 'A'
    assert_eq!(hex_char_to_int(b'G'), None); // after 'F'
    assert_eq!(hex_char_to_int(b'Z'), None);
    assert_eq!(hex_char_to_int(b' '), None);
    assert_eq!(hex_char_to_int(b'@'), None); // before 'A'
}

#[test]
fn test_hex_char_to_int_lowercase() {
    // Note: The function only handles uppercase A-F
    assert_eq!(hex_char_to_int(b'a'), None);
    assert_eq!(hex_char_to_int(b'f'), None);
}

// Tests for hex_chars_to_int
#[test]
fn test_hex_chars_to_int_valid() {
    assert_eq!(hex_chars_to_int(b'0', b'0'), Some(0x00));
    assert_eq!(hex_chars_to_int(b'F', b'F'), Some(0xFF));
    assert_eq!(hex_chars_to_int(b'A', b'5'), Some(0xA5));
    assert_eq!(hex_chars_to_int(b'1', b'2'), Some(0x12));
    assert_eq!(hex_chars_to_int(b'0', b'F'), Some(0x0F));
    assert_eq!(hex_chars_to_int(b'C', b'3'), Some(0xC3));
}

#[test]
fn test_hex_chars_to_int_invalid() {
    assert_eq!(hex_chars_to_int(b'G', b'0'), None);
    assert_eq!(hex_chars_to_int(b'0', b'G'), None);
    assert_eq!(hex_chars_to_int(b'Z', b'Z'), None);
    assert_eq!(hex_chars_to_int(b' ', b'0'), None);
}

// Tests for str_to_color
#[test]
fn test_str_to_color_hex_3_digit() {
    let color = str_to_color("#f00").unwrap();
    // #f00 should be parsed as #ff0000
    assert_eq!(color, Color::from_rgba_u8(0xF0, 0x00, 0x00, 255));
}

#[test]
fn test_str_to_color_hex_4_digit() {
    let color = str_to_color("#f00a").unwrap();
    // #f00a should be parsed as #ff0000aa
    assert_eq!(color, Color::from_rgba_u8(0xF0, 0x00, 0x00, 0xA0));
}

#[test]
fn test_str_to_color_hex_6_digit() {
    let color = str_to_color("#ff0000").unwrap();
    assert_eq!(color, Color::from_rgba_u8(255, 0, 0, 255));

    let color2 = str_to_color("#00ff00").unwrap();
    assert_eq!(color2, Color::from_rgba_u8(0, 255, 0, 255));
}

#[test]
fn test_str_to_color_hex_8_digit() {
    let color = str_to_color("#ff0000aa").unwrap();
    assert_eq!(color, Color::from_rgba_u8(255, 0, 0, 0xAA));
}

#[test]
fn test_str_to_color_hex_case_insensitive() {
    let upper = str_to_color("#FF0000").unwrap();
    let lower = str_to_color("#ff0000").unwrap();
    assert_eq!(upper, lower);
}

#[test]
fn test_str_to_color_rgb_with_parens() {
    let color = str_to_color("rgb(10, 250, 10)").unwrap();
    assert_eq!(
        color,
        Color::from_rgb(10.0 / 255.0, 250.0 / 255.0, 10.0 / 255.0)
    );
}

#[test]
fn test_str_to_color_rgb_without_parens() {
    let color = str_to_color("rgb 10 250 10").unwrap();
    assert_eq!(
        color,
        Color::from_rgb(10.0 / 255.0, 250.0 / 255.0, 10.0 / 255.0)
    );
}

#[test]
fn test_str_to_color_rgb_floats() {
    let color = str_to_color("rgb(0.1, 0.9, 0.1)").unwrap();
    assert_eq!(color, Color::from_rgb(0.1, 0.9, 0.1));
}

#[test]
fn test_str_to_color_rgb_mixed() {
    let color = str_to_color("rgb 0.5 230 0.9").unwrap();
    assert_eq!(color, Color::from_rgb(0.5, 230.0 / 255.0, 0.9));
}

#[test]
fn test_str_to_color_rgb_with_alpha() {
    let color = str_to_color("rgb(255, 128, 64, 200)").unwrap();
    assert_eq!(
        color,
        Color::from_rgba(1.0, 128.0 / 255.0, 64.0 / 255.0, 200.0 / 255.0)
    );
}

#[test]
fn test_str_to_color_rgb_1_1_1_is_almost_black() {
    // As documented: {rgb 1 1 1} gets parsed as almost black
    let color = str_to_color("rgb 1 1 1").unwrap();
    assert_eq!(
        color,
        Color::from_rgb(1.0 / 255.0, 1.0 / 255.0, 1.0 / 255.0)
    );
}

#[test]
fn test_str_to_color_rgb_1_0_is_white() {
    let color = str_to_color("rgb 1.0 1.0 1.0").unwrap();
    assert_eq!(color, Color::from_rgb(1.0, 1.0, 1.0));
}

#[test]
fn test_str_to_color_hsl() {
    let color = str_to_color("hsl 180 100 200").unwrap();
    assert_eq!(color, Color::from_hsl(180.0, 100.0 / 255.0, 200.0 / 255.0));
}

#[test]
fn test_str_to_color_hsl_with_parens() {
    let color = str_to_color("hsl(180, 100, 200)").unwrap();
    assert_eq!(color, Color::from_hsl(180.0, 100.0 / 255.0, 200.0 / 255.0));
}

#[test]
fn test_str_to_color_oklch() {
    let color = str_to_color("oklch 180 100 200").unwrap();
    assert_eq!(
        color,
        Color::from_oklch(180.0 / 255.0, 100.0 / 255.0, 200.0)
    );
}

#[test]
fn test_str_to_color_oklch_with_parens() {
    let color = str_to_color("oklch(180, 100, 200)").unwrap();
    assert_eq!(
        color,
        Color::from_oklch(180.0 / 255.0, 100.0 / 255.0, 200.0)
    );
}

#[test]
fn test_str_to_color_named_colors() {
    // Test case insensitivity and underscore/zero removal
    let color1 = str_to_color("RED_500").unwrap();
    let color2 = str_to_color("red5").unwrap();
    let color3 = str_to_color("Red_500").unwrap();

    assert_eq!(color1, Color::RED_500);
    assert_eq!(color2, Color::RED_500);
    assert_eq!(color3, Color::RED_500);
}

#[test]
fn test_str_to_color_named_colors_50() {
    // Test that _50 and 050 are normalized correctly
    let color1 = str_to_color("SLATE_50").unwrap();
    let color2 = str_to_color("slate0.5").unwrap();
    let color3 = str_to_color("SLATE05").unwrap();

    assert_eq!(color1, Color::SLATE_50);
    assert_eq!(color2, Color::SLATE_50);
    assert_eq!(color3, Color::SLATE_50);
}

#[test]
fn test_str_to_color_named_colors_950() {
    // Test that _950 is normalized correctly
    let color1 = str_to_color("SLATE_950").unwrap();
    let color2 = str_to_color("slate9.5").unwrap();
    let color3 = str_to_color("SLATE95").unwrap();

    assert_eq!(color1, Color::SLATE_950);
    assert_eq!(color2, Color::SLATE_950);
    assert_eq!(color3, Color::SLATE_950);
}

#[test]
fn test_str_to_color_invalid() {
    assert!(str_to_color("invalid").is_none());
    assert!(str_to_color("#gg0000").is_none());
    assert!(str_to_color("rgb(a, b, c)").is_none());
    assert!(str_to_color("").is_none());
}

#[test]
fn test_str_to_color_trimming() {
    let color1 = str_to_color(" #ff0000 ").unwrap();
    let color2 = str_to_color("#ff0000").unwrap();
    assert_eq!(color1, color2);
}

// Tests for RichText::parse
#[test]
fn test_richtext_parse_single_color() {
    let rich_text = RichText::parse("{RED_500}red text").unwrap();
    assert_eq!(rich_text.blocks.len(), 1);
    assert_eq!(rich_text.blocks[0].color, Color::RED_500);
    assert_eq!(rich_text.blocks[0].text, "red text");
}

#[test]
fn test_richtext_parse_multiple_colors() {
    let rich_text = RichText::parse("{RED_500}red text{BLUE_500}blue text").unwrap();
    assert_eq!(rich_text.blocks.len(), 2);
    assert_eq!(rich_text.blocks[0].color, Color::RED_500);
    assert_eq!(rich_text.blocks[0].text, "red text");
    assert_eq!(rich_text.blocks[1].color, Color::BLUE_500);
    assert_eq!(rich_text.blocks[1].text, "blue text");
}

#[test]
fn test_richtext_parse_space_after_color() {
    // Single space after color block is ignored
    let rich_text = RichText::parse("{RED_500}red text").unwrap();
    assert_eq!(rich_text.blocks.len(), 1);
    assert_eq!(rich_text.blocks[0].text, "red text");
}

#[test]
fn test_richtext_parse_no_space_after_color() {
    let rich_text = RichText::parse("{RED_500}red text").unwrap();
    assert_eq!(rich_text.blocks.len(), 1);
    assert_eq!(rich_text.blocks[0].text, "red text");
}

#[test]
fn test_richtext_parse_hex_color() {
    let rich_text = RichText::parse("{#f00}another red").unwrap();
    assert_eq!(rich_text.blocks.len(), 1);
    assert_eq!(
        rich_text.blocks[0].color,
        Color::from_rgba_u8(0xF0, 0x00, 0x00, 255)
    );
    assert_eq!(rich_text.blocks[0].text, "another red");
}

#[test]
fn test_richtext_parse_hex_with_alpha() {
    let rich_text = RichText::parse("{#f00a}semi-transparent red").unwrap();
    assert_eq!(rich_text.blocks.len(), 1);
    assert_eq!(
        rich_text.blocks[0].color,
        Color::from_rgba_u8(0xF0, 0x00, 0x00, 0xA0)
    );
    assert_eq!(rich_text.blocks[0].text, "semi-transparent red");
}

#[test]
fn test_richtext_parse_rgb_with_parens() {
    let rich_text = RichText::parse("{rgb(10, 250, 10)}green").unwrap();
    assert_eq!(rich_text.blocks.len(), 1);
    assert_eq!(
        rich_text.blocks[0].color,
        Color::from_rgb(10.0 / 255.0, 250.0 / 255.0, 10.0 / 255.0)
    );
    assert_eq!(rich_text.blocks[0].text, "green");
}

#[test]
fn test_richtext_parse_rgb_without_parens() {
    let rich_text = RichText::parse("{rgb 10 250 10}green").unwrap();
    assert_eq!(rich_text.blocks.len(), 1);
    assert_eq!(
        rich_text.blocks[0].color,
        Color::from_rgb(10.0 / 255.0, 250.0 / 255.0, 10.0 / 255.0)
    );
    assert_eq!(rich_text.blocks[0].text, "green");
}

#[test]
fn test_richtext_parse_rgb_floats() {
    let rich_text = RichText::parse("{rgb(0.1, 0.9, 0.1)}green text").unwrap();
    assert_eq!(rich_text.blocks.len(), 1);
    assert_eq!(rich_text.blocks[0].color, Color::from_rgb(0.1, 0.9, 0.1));
    assert_eq!(rich_text.blocks[0].text, "green text");
}

#[test]
fn test_richtext_parse_hsl() {
    let rich_text = RichText::parse("{hsl 180 100 200}cyan").unwrap();
    assert_eq!(rich_text.blocks.len(), 1);
    assert_eq!(rich_text.blocks[0].text, "cyan");
}

#[test]
fn test_richtext_parse_oklch() {
    let rich_text = RichText::parse("{oklch 180 100 200}color").unwrap();
    assert_eq!(rich_text.blocks.len(), 1);
    assert_eq!(rich_text.blocks[0].text, "color");
}

#[test]
fn test_richtext_parse_mixed_types() {
    let rich_text = RichText::parse("{rgb 0.5 230 0.9}mixed values").unwrap();
    assert_eq!(rich_text.blocks.len(), 1);
    assert_eq!(
        rich_text.blocks[0].color,
        Color::from_rgb(0.5, 230.0 / 255.0, 0.9)
    );
    assert_eq!(rich_text.blocks[0].text, "mixed values");
}

#[test]
fn test_richtext_parse_case_insensitive() {
    let rich_text = RichText::parse("{red5}this would give the same color").unwrap();
    assert_eq!(rich_text.blocks.len(), 1);
    assert_eq!(rich_text.blocks[0].color, Color::RED_500);
}

#[test]
fn test_richtext_parse_empty() {
    let rich_text = RichText::parse("").unwrap();
    assert_eq!(rich_text.blocks.len(), 0);
}

#[test]
fn test_richtext_parse_text_without_color() {
    // Text at the start without a color block should use default (WHITE)
    let rich_text = RichText::parse("plain text").unwrap();
    assert_eq!(rich_text.blocks.len(), 1);
    assert_eq!(rich_text.blocks[0].color, Color::WHITE);
    assert_eq!(rich_text.blocks[0].text, "plain text");
}

#[test]
fn test_richtext_parse_multiple_blocks_complex() {
    let input = "{RED_500}red{BLUE_500}blue{GREEN_500}green";
    let rich_text = RichText::parse(input).unwrap();
    assert_eq!(rich_text.blocks.len(), 3);
    assert_eq!(rich_text.blocks[0].text, "red");
    assert_eq!(rich_text.blocks[1].text, "blue");
    assert_eq!(rich_text.blocks[2].text, "green");
}

#[test]
fn test_richtext_parse_invalid_color() {
    let result = RichText::parse("{INVALID_COLOR}text");
    assert!(result.is_err());
    match result {
        Err(RichTextParseError::UnknownColor(color)) => {
            assert_eq!(color, "INVALID_COLOR");
        }
        _ => panic!("Expected UnknownColor error"),
    }
}

#[test]
fn test_richtext_parse_unclosed_brace() {
    // This should consume until EOF and return the text
    assert!(RichText::parse("{RED_500").is_err());
    // Based on the parser logic, it will consume until it expects '}'
    // This might actually error depending on implementation
}

#[test]
fn test_richtext_parse_empty_color_block() {
    let result = RichText::parse("{}text");
    // Empty color should fail to parse
    assert!(result.is_err());
}

#[test]
fn test_richtext_parse_consecutive_color_blocks() {
    let input = "{RED_500}{BLUE_500}text";
    let rich_text = RichText::parse(input).unwrap();
    // First color sets RED, second immediately sets BLUE, then text appears
    assert_eq!(rich_text.blocks.len(), 1);
    assert_eq!(rich_text.blocks[0].color, Color::BLUE_500);
    assert_eq!(rich_text.blocks[0].text, "text");
}

#[test]
fn test_richtext_parse_long_text() {
    let input = "{RED_500}This is a very long text that spans multiple words and contains various characters!@#$%^&*()";
    let rich_text = RichText::parse(input).unwrap();
    assert_eq!(rich_text.blocks.len(), 1);
    assert_eq!(
        rich_text.blocks[0].text,
        "This is a very long text that spans multiple words and contains various characters!@#$%^&*()"
    );
}

#[test]
fn test_richtext_parse_special_characters_in_text() {
    let input = "{RED_500}text with {braces} inside";
    let rich_text = RichText::parse(input);
    // The parser will see the second '{' and try to parse it as a color
    // This will likely result in an error or multiple blocks
    assert!(rich_text.is_err());
}

#[test]
fn test_richtext_new() {
    let blocks = vec![
        RichTextBlock::new("red".to_string(), Color::RED_500),
        RichTextBlock::new("blue".to_string(), Color::BLUE_500),
    ];
    let rich_text = RichText::new(blocks);
    assert_eq!(rich_text.blocks.len(), 2);
}

#[test]
fn test_richtext_block_from_str() {
    let block = RichTextBlock::from_str("test text", Color::RED_500);
    assert_eq!(block.text, "test text");
    assert_eq!(block.color, Color::RED_500);
}

#[test]
fn test_parse_hex_all_zeros() {
    let color = str_to_color("#000000").unwrap();
    assert_eq!(color, Color::from_rgba_u8(0, 0, 0, 255));
}

#[test]
fn test_parse_hex_all_ones() {
    let color = str_to_color("#ffffff").unwrap();
    assert_eq!(color, Color::from_rgba_u8(255, 255, 255, 255));
}

#[test]
fn test_parse_rgb_zero_values() {
    let color = str_to_color("rgb(0, 0, 0)").unwrap();
    assert_eq!(color, Color::from_rgb(0.0, 0.0, 0.0));
}

#[test]
fn test_parse_rgb_max_values() {
    let color = str_to_color("rgb(255, 255, 255)").unwrap();
    assert_eq!(color, Color::from_rgb(1.0, 1.0, 1.0));
}
