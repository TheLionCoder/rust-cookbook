use ansi_term::{Colour, Style};

pub fn print_styled_text() {
    println!("This is {} in color, {} and {} in color",
             Colour::Red.paint("red"),
             Colour::Green.paint("green"),
             Colour::Blue.paint("blue")
    );

    println!("{} and this is not",
            Style::new().bold().paint("This is bold"));

    println!("{}, {} and {}",
             Colour::Yellow.paint("This is colored"),
             Style::new().bold().paint("this is bold"),
             Colour::Yellow.bold().paint("this is bold colored")
    )
}