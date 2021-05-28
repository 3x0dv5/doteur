use std::fmt;

use super::super::add_traits::{Trim};

/// A dot table is the corresponding rendering of a sql table in a dot file
pub struct DotTable {
    /// The header of the table
    header: String,
    /// The attribute of the table
    attributes: Vec<String>,
    /// The footer of the table
    footer: String,
    /// Changes the rendering of the file if true
    dark_mode: bool
}

impl fmt::Display for DotTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}\n{1}\n\n\t{2}\n", self.header, self.attributes.join("\n"), self.footer)
    }
}

impl DotTable {

    /// Creates a new table
    ///
    /// # Arguments
    ///
    /// * `table_name` - The table to render in dot
    /// * `dark_mode` - Changes the rendering of the file if true
    pub fn new(table_name: &str, dark_mode: bool) -> DotTable {
        let header : String = generate_table_header(table_name, dark_mode);
        DotTable {
            header,
            attributes: Vec::new(),
            footer: String::from("</TABLE> >]"),
            dark_mode
        }
    }

    /// Adds an attribute to the table
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the attribute
    /// * `desc` - The description of the attribute
    pub fn add_attribute(&mut self, title: &str, desc : &str) {
        self.attributes.push(generate_attribute(title, desc, self.dark_mode));
    }


    /// Adds a foreign key attribute
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the attribute in the table
    /// * `fk_table` - The refered table
    /// * `fk_col` - The refered key
    pub fn add_attribute_fk(&mut self, key: &str, fk_table : &str, fk_col : &str) {
        self.attributes.push(generate_fk_attribute(key, fk_table, fk_col, self.dark_mode));
    }

}

/// Generate the .dot table header.
///
/// # Arguments
///
/// * `name` - The name of the table
/// * `dark_mode` - Changes the rendering of the table header if true
fn generate_table_header(name: &str, dark_mode: bool) -> String {
    let styles : (&str, &str) = match dark_mode {
            true => ("grey20", "grey10"),
            false => ("grey95", "indigo")
    };
    format!("
    {0} [label=<
        <TABLE BGCOLOR=\"{1}\" BORDER=\"1\" CELLBORDER=\"0\" CELLSPACING=\"0\">

        <TR><TD COLSPAN=\"2\" CELLPADDING=\"5\" ALIGN=\"CENTER\" BGCOLOR=\"{2}\">
        <FONT FACE=\"Roboto\" COLOR=\"white\" POINT-SIZE=\"12\">
        <B>{0}</B>
        </FONT></TD></TR>", name.trim_leading_trailing(), styles.0, styles.1)
}


/// Generate an attribute
///
/// # Arguments
///
/// * `title` - The name of the attribute
/// * `desc` - The description of the attribute
/// * `dark_mode` - Changes the rendering of the table header if true
fn generate_attribute(title: &str, desc: &str, dark_mode: bool) -> String {
    let font_color : &str = match dark_mode {
            true => "white",
            false => "black"
    };
    format!("
        <TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
        <FONT COLOR=\"{0}\" FACE=\"Roboto\"><B>{1}</B></FONT>
        </TD><TD ALIGN=\"LEFT\">
        <FONT COLOR=\"{0}\" FACE=\"Roboto\">{2}</FONT>
        </TD></TR>", font_color, title.trim_leading_trailing(), desc.trim_leading_trailing()
    )
}

/// Generas a foreign key attribute
///
/// # Arguments
///
/// * `key` - The key of the attribute in the table
/// * `fk_table` - The refered table
/// * `fk_col` - The refered key
/// * `dark_mode` - Changes the rendering of the table header if true
fn generate_fk_attribute(key: &str, fk_table: &str, fk_col: &str, dark_mode: bool) -> String {
    let font_color : &str = match dark_mode {
            true => "white",
            false => "black"
    };
    format!("
        <TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
        <FONT COLOR=\"{0}\" FACE=\"Roboto\"><B>{1} \u{1F5DD}</B></FONT>
        </TD><TD ALIGN=\"LEFT\">
        <FONT FACE=\"Roboto\" COLOR=\"{0}\">Refers to <I>{2}[{3}]</I></FONT>
        </TD></TR>", font_color,  key.trim_leading_trailing(), fk_table.trim_leading_trailing(), fk_col.trim_leading_trailing()
    )
}
