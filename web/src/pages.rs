use std::ops::Deref;

use yew::{function_component, html, use_state, Properties};

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
    let tasks_handle = use_state(|| {
        let mut tasks = Tasks::new();
        tasks.add_task(Task::new("task1"));
        tasks.add_task(Task::new("task2"));
        tasks.add_task(Task::new("task3"));
        tasks
    });
    html!(
        <div class="task-grid">
            <SideBar />
            <MainContent tasks={tasks_handle.deref().clone()}/>
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct Props {
    tasks: Tasks,
}

#[function_component(MainContent)]
fn main_content(props: &Props) -> Html {
    let Props{tasks} = props;
    html!(
        <div class="main-content">
        {
            "main content"
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
    html!(
        <div class="side-bar">
        {
            "sidebar"
        }
        </div>
    )
}
