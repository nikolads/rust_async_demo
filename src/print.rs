use std::io;

use crate::{BlogGroup, Week};

pub fn print_week<W: io::Write>(mut dest: W, week: &Week, blogs: &[BlogGroup]) -> io::Result<()> {
    match &week.date {
        Some(date) => writeln!(dest, "# {} ({})", week.title, date)?,
        None => writeln!(dest, "# {}", week.title)?,
    }

    for blog_group in blogs {
        if let Some(title) = &blog_group.title {
            writeln!(dest, "## {}", title)?;
        }

        for b in &blog_group.blogs {
            writeln!(dest, "- {}", b)?;
        }

        writeln!(dest)?;
    }

    writeln!(dest)?;

    Ok(())
}
