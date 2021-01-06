use scraper::{Html, Selector};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Week {
    pub title: String,
    pub href: String,
    pub date: Option<String>,
}

pub fn parse_weeks(doc: &Html) -> Vec<Week> {
    let selector_list = Selector::parse("ul.past-issues > li").unwrap();

    doc.select(&selector_list)
        .flat_map(|li| {
            let selector_time = Selector::parse("time").unwrap();
            let selector_link = Selector::parse("a").unwrap();

            let mut time_nodes = li.select(&selector_time);
            let mut link_nodes = li.select(&selector_link);

            let found_time = match (time_nodes.next(), time_nodes.next()) {
                (Some(time), None) => Some(time.value().attr("datetime").map(str::to_string)),
                _ => None,
            }?;

            let found_link = match (link_nodes.next(), link_nodes.next()) {
                (Some(link), None) => link
                    .value()
                    .attr("href")
                    .map(str::to_string)
                    .zip(Some(link.text().collect())),
                _ => None,
            }?;

            let (href, title) = found_link;
            Some(Week {
                href,
                title,
                date: found_time,
            })
        })
        .collect()
}
