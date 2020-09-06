use crate::artists::{Artist};
use prettytable::{Table, Row, Cell, color, Attr, format};
use prettytable::color::Color;

pub fn output_artist_table(artist: &Artist) {
    let name = format!("{}", artist.strArtist);
    let formed_at  = format!("{}", artist.intFormedYear);
    let genre  = format!("{}", artist.strGenre);
    let bio  = format!("{}", artist.strBiographyEN);

    output_head_table( Table::new(), name, formed_at, genre);
    output_bio(Table::new(), bio);
}

fn style_cell(key: String, color: Color, is_bold: bool) -> Cell {
   match is_bold {
       true => Cell::new(&key)
           .with_style(Attr::Bold)
           .with_style(Attr::ForegroundColor(color)),
       false => Cell::new(&key)
           .with_style(Attr::ForegroundColor(color))
   }
}

fn style_cell_bg(key: String, color: Color) -> Cell {
    Cell::new(&key).with_style(Attr::BackgroundColor(color))
}

fn output_head_table(mut table: Table, name: String, formed_at: String, genre: String) {
    table.add_row(Row::new(vec![
        style_cell(name, color::GREEN, true),
        style_cell(formed_at, color::RED, true),
        style_cell_bg(genre, color::BLUE)
    ]));
    table.printstd();
}

fn output_bio(mut table: Table, bio: String) {
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.add_row(Row::new(vec![style_cell(bio, color::MAGENTA, false)]));
    print!("Bio:\n");
    table.printstd();
}
