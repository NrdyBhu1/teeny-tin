extern crate pancurses;

use std::{fs::read, io::Error, path::Path};

use pancurses::{endwin, initscr, noecho, Input, Window};

pub fn read_file_as_string(pt: &Path) -> Result<String, Error> {
    let contents: String = String::from_utf8(read(pt).unwrap()).unwrap();
    Ok(contents)
}

fn addstr(window: &Window, cont: &str) {
    let lines: Vec<_> = cont.split('\n').into_iter().collect();
    let mut c = 1;
    for line in lines.iter() {
        window.mvaddstr(c, 2, line);
        c += 1;
    }
}

fn main() {
    let window = initscr();
    let mut content: String = read_file_as_string(Path::new("/home/bhuvan/todo.md")).unwrap();
    window.keypad(true);
    noecho();
    loop {
        window.clear();
        window.border(0, 0, 0, 0, 0, 0, 0, 0);
        window.mvaddstr(0, 2, "Teeny Tiny");
        window.mv(1, 2);
        addstr(&window, &content);
        window.refresh();

        if let Some(key) = window.getch() {
            match key {
                Input::KeyBackspace => {
                    break;
                }
                Input::Character('r') => {
                    content = read_file_as_string(Path::new("/home/bhuvan/todo.md")).unwrap();
                }
                _ => {}
            }
        }
    }
    endwin();
}
