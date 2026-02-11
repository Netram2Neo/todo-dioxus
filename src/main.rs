use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Todo {
    id: u32,
    text: String,
    completed: bool,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut todos = use_signal(|| vec![
        Todo {
            id: 1,
            text: "Erste Aufgabe".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            text: "Zweite Aufgabe".to_string(),
            completed: true,
        },
    ]);

    let mut input_text = use_signal(|| String::new());
    let mut next_id = use_signal(|| 3u32);

    let add_todo = move |_| {
        let text = input_text.read().clone();
        if !text.is_empty() {
            let new_todo = Todo {
                id: *next_id.read(),
                text,
                completed: false,
            };
            todos.push(new_todo);
            input_text.set(String::new());
            *next_id.write() += 1;
        }
    };

    let toggle_todo = move |id: u32| {
        let mut todos = todos.write();
        if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
    };

    let delete_todo = move |id: u32| {
        todos.write().retain(|t| t.id != id);
    };

    let completed_count = todos
        .read()
        .iter()
        .filter(|t| t.completed)
        .count();
    let total_count = todos.read().len();

    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-purple-50 to-blue-50 p-8",
            div { class: "max-w-2xl mx-auto",
                h1 { class: "text-4xl font-bold text-gray-800 mb-8 text-center",
                    "📝 Todo App"
                }

                // Add new todo
                div { class: "bg-white rounded-lg shadow-md p-6 mb-6",
                    div { class: "flex gap-3",
                        input {
                            class: "flex-1 px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent",
                            placeholder: "Neue Aufgabe...",
                            value: "{input_text}",
                            oninput: move |e| input_text.set(e.value())
                        }
                        button {
                            class: "px-6 py-3 bg-purple-600 text-white rounded-lg font-semibold hover:bg-purple-700 transition-colors duration-200",
                            onclick: add_todo,
                            "Hinzufügen"
                        }
                    }
                }

                // Stats
                div { class: "bg-white rounded-lg shadow-md p-4 mb-6",
                    p { class: "text-gray-600 text-center",
                        "{completed_count} von {total_count} Aufgaben erledigt"
                    }
                    if total_count > 0 {
                        div { class: "mt-2 h-2 bg-gray-200 rounded-full overflow-hidden",
                            div {
                                class: "h-full bg-gradient-to-r from-purple-500 to-blue-500 transition-all duration-300",
                                style: "width: {(completed_count as f32 / total_count as f32 * 100.0)}%"
                            }
                        }
                    }
                }

                // Todo list
                if todos.read().is_empty() {
                    div { class: "bg-white rounded-lg shadow-md p-12 text-center",
                        p { class: "text-gray-500 text-lg", "Keine Aufgaben vorhanden" }
                        p { class: "text-gray-400 mt-2", "Füge oben eine neue Aufgabe hinzu!" }
                    }
                } else {
                    div { class: "space-y-3",
                        for todo in todos.read().iter() {
                            div { class: "bg-white rounded-lg shadow-md p-4 flex items-center gap-4 transition-all duration-200 hover:shadow-lg",
                                input {
                                    type: "checkbox",
                                    checked: todo.completed,
                                    class: "w-5 h-5 rounded border-gray-300 text-purple-600 focus:ring-purple-500 cursor-pointer",
                                    onchange: move |_| toggle_todo(todo.id)
                                }
                                p {
                                    class: "flex-1 text-lg transition-all duration-200 {if todo.completed { \"line-through text-gray-400\" } else { \"text-gray-800\" }}",
                                    "{todo.text}"
                                }
                                button {
                                    class: "px-3 py-1 bg-red-100 text-red-600 rounded-lg hover:bg-red-200 transition-colors duration-200",
                                    onclick: move |_| delete_todo(todo.id),
                                    "Löschen"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
