use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8
}

impl FromStr for Rgb {
    type Err = std::num::ParseIntError;

    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
        let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
        let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

        Ok(Rgb {r, g, b})
    }

}

pub fn parse_hex_color() {
    let code: &str = r"#fa7268";
    match Rgb::from_str(code) {
        Ok(rgb) => {
            println!(
                "The RGB color code is: R: {}, G: {}, B: {}",
                rgb.r, rgb.g, rgb.b
            );
        }
        Err(_) => {
            println!("{} is not a valid hex color code", code)
        }
    }

    // tests whether from_str performs as expected
    assert_eq!(
        Rgb::from_str(r"#fa7268").unwrap(),
        Rgb {
            r: 250,
            g: 114,
            b: 104
        }
    )
}