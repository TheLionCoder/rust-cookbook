use unicode_segmentation::UnicodeSegmentation;

pub fn collect_unicode_graphemes() {
    let name: &str = "José Guimarães\r\n";
    let graphemes: Vec<&str> = UnicodeSegmentation::graphemes(name, true)
        .collect::<Vec<&str>>();
    println!("{:?}", &graphemes);
    assert_eq!(graphemes[3], "é");
}