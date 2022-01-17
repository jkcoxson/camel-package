// jkcoxson

use cursive::{
    align::HAlign,
    traits::{Nameable, Scrollable},
    views::{Dialog, EditView, SelectView, TextView},
    CursiveExt,
};

pub fn yes_or_no(question: &str) -> bool {
    let (sender, receiver) = std::sync::mpsc::channel();
    let cloned_sender = sender.clone();
    let mut prompt = cursive::Cursive::default();
    prompt.add_layer(
        Dialog::around(TextView::new(question).with_name("prompt"))
            .title("Camel Package")
            .button("Yes", move |s| {
                s.pop_layer();
                s.quit();
                sender.send(true).unwrap();
            })
            .button("No", move |s| {
                s.pop_layer();
                s.quit();
                cloned_sender.send(false).unwrap();
            }),
    );
    prompt.run();
    receiver.recv().unwrap()
}

pub fn multi_prompt(question: &str, options: Vec<String>) -> String {
    let (sender, receiver) = std::sync::mpsc::channel();
    let mut prompt = cursive::Cursive::default();
    let mut select_view = SelectView::new()
        .h_align(HAlign::Center)
        .autojump()
        .on_submit(move |s, selection: &str| {
            s.pop_layer();
            s.quit();
            sender.send(selection.to_string()).unwrap();
        });
    select_view.add_all_str(options);

    prompt.add_layer(Dialog::around(select_view).title(question).scrollable());
    prompt.run();
    receiver.recv().unwrap()
}

pub fn text_prompt(question: &str) -> String {
    let (sender, receiver) = std::sync::mpsc::channel();
    let cloned_sender = sender.clone();
    let mut prompt = cursive::Cursive::default();
    // Add an edit view in a dialogue
    prompt.add_layer(
        Dialog::new()
            .title(question)
            .content(EditView::new().on_submit(move |s, text| {
                s.pop_layer();
                s.quit();
                cloned_sender.send(text.to_string()).unwrap();
            })),
    );
    prompt.run();
    receiver.recv().unwrap()
}
