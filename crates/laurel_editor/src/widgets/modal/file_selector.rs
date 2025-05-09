use fuzzy_matcher::{skim::SkimMatcher, FuzzyMatcher};
use iced::{
    widget::{
        button, column, container, scrollable, scrollable::Scrollbar, text, text_input, Column,
    },
    Length, Padding,
};
use laurel_common::{appearance, Element};
use std::{collections::VecDeque, fs, path::Path};

use crate::Message;

/**
 * A modal that can be of different types
 */

#[derive(Debug)]
pub enum Modal {
    FileSelector(String),
}

impl Modal {
    pub fn show(&self, filter: &str) -> Element<'static, Message> {
        match self {
            Modal::FileSelector(file) => file_selector(file, filter),
        }
    }
}

fn matches(choice: &str, pattern: &str) -> bool {
    let matcher = SkimMatcher::default();
    if pattern.is_empty() {
        return true;
    }
    if let Some(value) = matcher.fuzzy_match(choice, pattern) {
        return value > 0;
    }
    false
}

pub fn file_selector(path: &str, filter: &str) -> Element<'static, Message> {
    let excluded_list = ["target", ".git"];
    let files = get_files(path, &excluded_list);

    let mut buttons = Column::new();

    for file in files {
        let filename = file.clone();
        let relative_path = filename.replace(path, "");

        if matches(&file, filter) {
            buttons = buttons.push(
                button(text(relative_path))
                    .width(Length::Fill)
                    .style(appearance::button::menu)
                    .on_press(Message::Open(file.to_owned())),
            )
        }
    }

    container(
        column!(
            text_input("Filter files", filter).on_input(Message::FileFilter),
            scrollable(buttons)
                .width(Length::Fill)
                .direction(scrollable::Direction::Vertical(
                    Scrollbar::default().scroller_width(0.0).width(0.0),
                ))
        )
        .spacing(0)
        .padding(Padding::from([10, 0])),
    )
    .height(Length::Fixed(600.0))
    .width(Length::Fixed(600.0))
    .into()
}

/**
 * Uses breadth first search to find all the folders in a given directory. Does not search for the files within the excluded list.
 */
pub fn get_files(path: &str, excluded_list: &[&str]) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    let mut folders: VecDeque<String> = VecDeque::new();

    folders.push_front(path.to_owned());

    while !folders.is_empty() {
        let folder_string = folders.pop_front().unwrap();
        let folder = Path::new(&folder_string);
        let paths = fs::read_dir(folder).unwrap();

        for path in paths {
            let entry = path.unwrap().path();
            if entry.is_dir() {
                let is_in_list = excluded_list
                    .iter()
                    .find(|filename| &&entry.file_name().unwrap().to_str().unwrap() == filename);
                if is_in_list.is_some() {
                    continue;
                }
                folders.push_back(entry.to_str().unwrap().to_owned())
            } else if entry.is_file() {
                files.push(entry.to_str().unwrap().to_owned())
            }
        }
    }
    files
}

#[cfg(test)]
mod file_selector_tests {
    use crate::widgets::modal::file_selector::get_files;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_breadth_first_search() {
        let excluded_list = ["target", ".git"];
        let files = get_files("./", &excluded_list);
        assert_eq!(files.get(0), Some(&"./Cargo.lock".to_owned()));
    }
}
