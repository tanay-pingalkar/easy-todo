use druid::im::{vector, Vector};
use druid::widget::{prelude::*, Scroll};
use druid::widget::{
    BackgroundBrush, Button, Flex, Label, List, MainAxisAlignment, Painter, TextBox,
};
use druid::{lens, theme, AppLauncher, Color, Lens, LensExt, UnitPoint, WidgetExt, WindowDesc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{read_to_string, write};

#[derive(Serialize, Deserialize, Debug)]
struct Todos {
    todos: Vec<Todo>,
    completed: Vec<Todo>,
}

#[derive(Clone, Data, Lens)]
struct State {
    input: String,
    todos: Vector<Todo>,
    completed: Vector<Todo>,
}

#[derive(Clone, Data, Lens, Serialize, Deserialize, Debug)]
struct Todo {
    value: String,
    id: u32,
}
fn main() {
    let window = WindowDesc::new(widget())
        .title("todo")
        .window_size((800.0, 700.0));

    let state;
    match read_to_string("./todos.json") {
        Ok(file) => {
            let ser: Todos = serde_json::from_str(file.as_str()).unwrap();
            let mut todos = vector![];
            for todo in ser.todos {
                todos.push_back(todo);
            }
            let mut completeds = vector![];

            for completed in ser.completed {
                completeds.push_back(completed);
            }

            state = State {
                input: "".to_string(),
                todos,
                completed: completeds,
            }
        }
        Err(..) => {
            let todos = Todos {
                completed: Vec::new(),
                todos: Vec::new(),
            };
            write("./todos.json", serde_json::to_string(&todos).unwrap()).unwrap();
            state = State {
                input: "".to_string(),
                todos: vector![],
                completed: vector![],
            }
        }
    }

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(state)
        .expect("Failed to launch application");
}

fn widget() -> impl Widget<State> {
    let label = Label::new(|_: &State, _: &Env| "TODO".to_string()).with_text_size(25.0);

    let label2 = Label::new(|_: &State, _: &Env| "incomplete".to_string()).with_text_size(20.0);

    let label3 = Label::new(|_: &State, _: &Env| "completed".to_string()).with_text_size(20.0);

    Flex::column()
        .with_spacer(10.0)
        .with_child(label)
        .with_spacer(10.0)
        .with_child(input())
        .with_spacer(20.0)
        .with_child(label2)
        .with_child(todos())
        .with_spacer(20.0)
        .with_child(label3)
        .with_child(completed())
}

fn input() -> impl Widget<State> {
    let input = TextBox::new()
        .with_placeholder("what you wanna save")
        .with_text_size(20.0)
        .fix_width(500.0)
        .lens(State::input);

    let button = Button::new("add")
        .on_click(|_, state: &mut State, _| {
            if state.input.trim() != "" {
                state.todos.push_back(Todo {
                    value: state.input.clone(),
                    id: state.todos.len() as u32,
                });
                state.input = "".to_string();
                let todos = Todos {
                    completed: state.completed.iter().map(|val| val.clone()).collect(),
                    todos: state.todos.iter().map(|val| val.clone()).collect(),
                };
                write("./todos.json", serde_json::to_string(&todos).unwrap()).unwrap();
            }
        })
        .fix_height(35.0);

    Flex::row()
        .with_child(input)
        .with_spacer(5.0)
        .with_child(button)
}

fn todos() -> impl Widget<State> {
    List::new(|| {
        Flex::row()
            .with_flex_child(
                Label::new(|(_, item): &(State, Todo), _: &Env| item.value.to_string())
                    .with_text_size(20.0)
                    .padding(5.0)
                    .background(BackgroundBrush::Color(
                        Color::from_hex_str("#3A3A3A").unwrap(),
                    ))
                    .rounded(3.0)
                    .fix_width(470.0),
                38.0,
            )
            .with_flex_spacer(0.5)
            .with_flex_child(
                Button::new("remove")
                    .on_click(move |_ctx, (state, item): &mut (State, Todo), _env| {
                        println!("redo");
                        state.todos.retain(|v| v.id != item.id);
                        let todos = Todos {
                            completed: state.completed.iter().map(|val| val.clone()).collect(),
                            todos: state.todos.iter().map(|val| val.clone()).collect(),
                        };
                        write("./todos.json", serde_json::to_string(&todos).unwrap()).unwrap();
                    })
                    .fix_height(35.0),
                7.0,
            )
            .with_flex_spacer(0.5)
            .with_flex_child(
                Button::new("done")
                    .on_click(move |_ctx, (state, item): &mut (State, Todo), _env| {
                        state.todos.retain(|v| v.id != item.id);
                        state.completed.push_back(item.clone());
                        let todos = Todos {
                            completed: state.completed.iter().map(|val| val.clone()).collect(),
                            todos: state.todos.iter().map(|val| val.clone()).collect(),
                        };
                        write("./todos.json", serde_json::to_string(&todos).unwrap()).unwrap();
                    })
                    .fix_height(35.0),
                5.0,
            )
    })
    .with_spacing(10.0)
    .fix_width(550.0)
    .lens(lens::Identity.map(
        // Expose shared data with children data
        |d: &State| (d.clone(), d.todos.clone()),
        |d: &mut State, (state, _): (State, Vector<Todo>)| {
            // If shared data was changed reflect the changes in our AppData
            d.todos = state.todos;
            d.completed = state.completed;
        },
    ))
}

fn completed() -> impl Widget<State> {
    List::new(|| {
        Flex::row()
            .with_flex_child(
                Label::new(|(_, item): &(State, Todo), _: &Env| item.value.to_string())
                    .with_text_size(20.0)
                    .padding(5.0)
                    .background(BackgroundBrush::Color(
                        Color::from_hex_str("#3A3A3A").unwrap(),
                    ))
                    .rounded(3.0)
                    .fix_width(475.0),
                38.0,
            )
            .with_flex_spacer(0.5)
            .with_flex_child(
                Button::new("remove")
                    .on_click(move |_ctx, (state, item): &mut (State, Todo), _env| {
                        state.completed.retain(|v| v.id != item.id);
                        let todos = Todos {
                            completed: state.completed.iter().map(|val| val.clone()).collect(),
                            todos: state.todos.iter().map(|val| val.clone()).collect(),
                        };
                        write("./todos.json", serde_json::to_string(&todos).unwrap()).unwrap();
                    })
                    .fix_height(35.0),
                7.0,
            )
            .with_flex_spacer(0.5)
            .with_flex_child(
                Button::new("redo")
                    .on_click(move |_ctx, (state, item): &mut (State, Todo), _env| {
                        state.completed.retain(|v| v.id != item.id);
                        state.todos.push_back(item.clone());
                        let todos = Todos {
                            completed: state.completed.iter().map(|val| val.clone()).collect(),
                            todos: state.todos.iter().map(|val| val.clone()).collect(),
                        };
                        write("./todos.json", serde_json::to_string(&todos).unwrap()).unwrap();
                    })
                    .fix_height(35.0),
                5.0,
            )
    })
    .with_spacing(10.0)
    .fix_width(550.0)
    .lens(lens::Identity.map(
        // Expose shared data with children data
        |d: &State| (d.clone(), d.completed.clone()),
        |d: &mut State, (state, _): (State, Vector<Todo>)| {
            // If shared data was changed reflect the changes in our AppData
            d.todos = state.todos;
            d.completed = state.completed;
        },
    ))
}
