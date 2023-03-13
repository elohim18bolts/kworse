use std::io::Write;

use search::text::search_in_text;
fn read_input(buffer: &mut String) -> std::io::Result<()> {
    std::io::stdin().read_line(buffer)?;
    Ok(())
}
fn run() -> Result<(), std::io::Error> {
    let mut text = String::new();
    let mut search_term = String::new();
    println!("Insert text: ");
    read_input(&mut text)?;
    print!("Insert term to search: ");
    std::io::stdout().flush()?;
    read_input(&mut search_term)?;
    text = text.trim_end().to_string();
    search_term = search_term.trim_end().to_string();
    let res = search_in_text(&text, &search_term);
    if res.len() == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Nothing found.",
        ));
    }
    let mut n = 0;
    let red = "x1b[31m";
    let reset = "x1b[0m";
    for m in res {
        text.insert_str(m.1 + n, red);
        n += 7;
        text.insert_str(m.1 + search_term.len() + n, reset);
        n += 6;
    }
    println!(
        "{text}",
        text = &text.replace(red, "\x1b[31m").replace(reset, "\x1b[0m")
    );
    Ok(())
}
fn main() -> std::io::Result<()> {
    run()?;
    Ok(())
}
