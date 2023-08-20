use std::io::{self, Write};

struct Note {
    content: String,
    completed: bool,
}

struct NotesApp {
    notes: Vec<Note>,
}

impl NotesApp {
    fn new() -> Self {
        NotesApp { notes: Vec::new() }
    }

    fn create_note(&mut self, content: String) {
        let note = Note {
            content,
            completed: false,
        };
        self.notes.push(note);
        println!("Заметка успешно создана!");
    }

    fn delete_note(&mut self, index: usize) {
        if index < self.notes.len() {
            self.notes.remove(index);
            println!("Заметка успешно удалена!");
        } else {
            println!("Неверный индекс заметки!");
        }
    }

    fn view_notes(&self) {
        if self.notes.is_empty() {
            println!("Заметок нет.");
        } else {
            for (index, note) in self.notes.iter().enumerate() {
                let status = if note.completed { "[x]" } else { "[ ]" };
                println!("{} {}: {}", index, status, note.content);
            }
        }
    }

    fn delete_all_notes(&mut self) {
        self.notes.clear();
        println!("Все заметки успешно удалены!");
    }

    fn mark_note_as_completed(&mut self, index: usize) {
        if index < self.notes.len() {
            self.notes[index].completed = true;
            println!("Заметка успешно отмечена как выполненная!");
        } else {
            println!("Неверный индекс заметки!");
        }
    }

    fn sort_notes_reverse(&mut self) {
        self.notes.reverse();
        println!("Заметки успешно отсортированы в обратном порядке!");
    }
}

fn main() {
    let mut app = NotesApp::new();

    loop {
        println!("");
        println!("\x1B[1m\x1B[3mВыберите действие:\x1B[0m");
        println!("1. Создать заметку");
        println!("2. Удалить заметку");
        println!("3. Посмотреть заметки");
        println!("4. Удалить все заметки");
        println!("5. Отметить заметку как выполненную");
        println!("6. Отсортировать заметки в обратном порядке");
        println!("0. Выйти");
        println!("");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = input.trim().parse().unwrap();

        match choice {
            0 => {
                println!("До свидания!");
                break;
            }
            1 => {
                print!("Введите содержимое заметки: ");
                io::stdout().flush().unwrap();
                let mut content = String::new();
                io::stdin().read_line(&mut content).unwrap();
                app.create_note(content.trim().to_string());
            }
            2 => {
                print!("Введите индекс заметки для удаления: ");
                io::stdout().flush().unwrap();
                let mut index_input = String::new();
                io::stdin().read_line(&mut index_input).unwrap();
                let index: usize = index_input.trim().parse().unwrap();
                app.delete_note(index);
            }
            3 => {
                app.view_notes();
            }
            4 => {
                app.delete_all_notes();
            }
            5 => {
                print!("Введите индекс заметки для отметки как выполненной: ");
                io::stdout().flush().unwrap();
                let mut index_input = String::new();
                io::stdin().read_line(&mut index_input).unwrap();
                let index: usize = index_input.trim().parse().unwrap();
                app.mark_note_as_completed(index);
            }
            6 => {
                app.sort_notes_reverse();
            }
            _ => {
                println!("Неверный выбор. Попробуйте еще раз.");
            }
        }
    }
}
