use std::sync::mpsc::Receiver;

use {std::sync::mpsc, tray_item::TrayItem};

pub enum Message {
    Quit,
    AnonymiseProject,
}

pub fn create_tray() -> (Receiver<Message>, TrayItem) {
    let mut tray = TrayItem::new("DRP Creative", "drp-icon").unwrap();
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    tray.add_label("DRP Creative Options").unwrap();

    tray.add_menu_item("Don't show current project", move || {
        tx.send(Message::AnonymiseProject).unwrap();
    })
    .unwrap();

    tray.add_menu_item("Quit", move || {
        tx1.send(Message::Quit).unwrap();
    })
    .unwrap();
    return (rx, tray);
}
