use std::sync::mpsc::Receiver;

use {std::sync::mpsc, tray_item::TrayItem};

pub enum Message {
    Quit,
    AddProjectToList,
    RemoveProjectFromList,
    OpenOptionsFile,
}

pub fn create_tray() -> (Receiver<Message>, TrayItem) {
    let mut tray = TrayItem::new("DRP Creative", "drp-icon").unwrap();
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    let tx3 = tx.clone();
    let tx4 = tx.clone();
    tray.add_menu_item("Open options file", move || {
        tx4.send(Message::OpenOptionsFile).unwrap();
    })
    .unwrap();

    tray.add_menu_item("Add project to include/exclude list", move || {
        tx1.send(Message::AddProjectToList).unwrap();
    })
    .unwrap();

    tray.add_menu_item("Remove project from include/exclude list", move || {
        println!("REMOVE");
        tx3.send(Message::RemoveProjectFromList).unwrap();
    })
    .unwrap();

    tray.add_menu_item("Quit", move || {
        tx2.send(Message::Quit).unwrap();
    })
    .unwrap();

    return (rx, tray);
}
