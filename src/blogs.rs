use scraper::{ElementRef, Html, Node, Selector};
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlogGroup {
    pub title: Option<String>,
    pub blogs: Vec<Blog>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Blog {
    pub title: String,
    pub href: String,
    pub ty: BlogType,
}

impl fmt::Display for Blog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix_tmp;
        let type_prefix = match &self.ty {
            BlogType::Text => "",
            BlogType::Video => "\\[video\\] ",
            BlogType::Unknown(prefix) => {
                prefix_tmp = format!("\\[{}\\] ", prefix);
                &prefix_tmp
            }
        };

        write!(f, "{}[{}]({})", type_prefix, self.title, self.href)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BlogType {
    Text,
    Video,
    Unknown(String),
}

pub fn parse_blogs(doc: &Html) -> Vec<BlogGroup> {
    use std::ops::Deref;

    let selector_updates = Selector::parse("h2#updates-from-rust-community").unwrap();

    let selector_li = Selector::parse("li").unwrap();
    let selector_a = Selector::parse("a").unwrap();

    let mut updates_blogs = vec![];

    if let Some(h1) = doc.select(&selector_updates).next() {
        let mut group_title = None;

        for sibling in h1
            .next_siblings()
            .flat_map(ElementRef::wrap)
            .take_while(|sib| sib.value().name() != "h2")
        {
            match sibling.value().name() {
                "h3" | "h4" | "h5" | "h6" => {
                    group_title = Some(sibling.text().collect::<String>());
                }
                "ul" => {
                    let blogs = sibling
                        .select(&selector_li)
                        .flat_map(|li| {
                            let a = li.select(&selector_a).next()?;

                            let title = a.text().collect();
                            let href = a.value().attr("href").map(str::to_string)?;

                            let li_text = li
                                .children()
                                .flat_map(|node| match node.value() {
                                    Node::Text(text_node) => Some(text_node.deref()),
                                    _ => None,
                                })
                                .collect::<String>();

                            let ty = match li_text.trim() {
                                "" => BlogType::Text,
                                "[video]" => BlogType::Video,
                                prefix => BlogType::Unknown(prefix.to_string()),
                            };

                            Some(Blog { title, href, ty })
                        })
                        .collect();

                    updates_blogs.push(BlogGroup {
                        title: group_title.take(),
                        blogs,
                    });
                }
                _ => (),
            }
        }
    }

    updates_blogs
}
