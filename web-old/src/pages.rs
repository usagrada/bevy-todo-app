use std::ops::Deref;

use yew::{
    function_component, html, prelude::*, use_context, use_state, ContextProvider, Properties,
};

#[derive(PartialEq, Eq, Clone, Debug)]
struct Channels {
    channels: Vec<String>,
    select: String,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Tasks {
    tasks: Vec<Task>,
}

impl Tasks {
    fn new() -> Self {
        Self { tasks: vec![] }
    }
    fn add_task(&mut self, task: Task) -> &Self {
        self.tasks.push(task);
        self
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Task {
    text: String,
}

impl Task {
    fn new<T: Into<String>>(text: T) -> Self {
        Self { text: text.into() }
    }
}

#[function_component(TaskPage)]
pub fn task_page() -> Html {
    let channels_handle = use_state(|| Channels {
        channels: vec!["general".to_string(), "random".to_string()],
        select: "general".to_string(),
    });
    let tasks_handle = use_state(|| {
        let mut tasks = Tasks::new();
        tasks.add_task(Task::new("task1"));
        tasks.add_task(Task::new("task2"));
        tasks.add_task(Task::new("task3"));
        tasks
    });
    html!(
        <div class="task-grid">
            <ContextProvider<Tasks> context={(*tasks_handle).clone()}>
                <ContextProvider<UseStateHandle<Channels>> context={(channels_handle).clone()}>
                <SideBar />
                <MainContent/>
                </ContextProvider<UseStateHandle<Channels>>>
            </ContextProvider<Tasks>>
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct Props {
    tasks: Tasks,
}

#[function_component(MainContent)]
// fn main_content(props: &Props) -> Html {
fn main_content() -> Html {
    // let Props { tasks } = props;
    let tasks = use_context::<Tasks>().expect("no ctx found");
    let channels_handle = use_context::<UseStateHandle<Channels>>().expect("no ctx found");

    html!(
        <div class="main-content">
        {
            "main content\n"
        }
        {
            channels_handle.deref().clone().select
        }
        {
            for tasks.tasks.iter().map(|task| {
                html! {
                    <div class="task">
                        {&task.text}
                    </div>
                }
            })
        }
        </div>
    )
}

#[function_component(SideBar)]
fn side_bar() -> Html {
    let channels_handler = use_context::<UseStateHandle<Channels>>().expect("no ctx found");
    let Channels {
        channels,
        select: _,
    } = channels_handler.deref().clone();

    let update_channel = {
        let channels_handler = channels_handler.clone();
        let chs = channels.clone();
        Callback::from(move |new_channel: String| {
            channels_handler.set(Channels {
                select: new_channel.to_string(),
                channels: chs.clone(),
            });
        })
    };

    html!(
        <div class="side-bar">
            <div>
                {"sidebar"}
            </div>
            {
                for channels.iter().map(|ch: &String| {
                    let onclick = {
                        let update_channel = update_channel.clone();
                        let ch = ch.clone();
                        Callback::from(move |_| {
                            update_channel.emit(ch.clone());
                        })
                    };
                    html! {
                        <div class="task" onclick={onclick}>
                            {&ch}
                        </div>
                    }
                })
            }
        </div>
    )
}

#[function_component(StateExample)]
fn state_example() -> Html {
    let name_handle = use_state(|| String::from("Bob"));
    let name = name_handle.deref().clone();
    let onclick = {
        let name = name.clone();
        Callback::from(move |_| name_handle.set(format!("{}y Jr.", name)))
    };

    html! {
        <div>
            <button {onclick}>{ "Update name" }</button>
            <p>
                <b>{ "My name is: " }</b>
                { name }
            </p>
        </div>
    }
}
