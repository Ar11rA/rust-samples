#![allow(dead_code)]

// basic example of a jira task
/*
{
    "id": 1,
    "title": "explore rust",
    "description": "explore all data structures in rust.",
    "assignee": "Ar11rA",
    "status": "READY"
}
 */
// status can be enum of READY, IN-PROGRESS, COMPLETED, IN-REVIEW
#[derive(Debug)]
pub enum TaskStatus {
    Ready,
    InProgress,
    InReview,
    Completed,
}

struct Task {
    id: i32,
    title: String,
    description: String,
    assignee: String,
    status: TaskStatus,
}

pub fn set_default_task() {
    let task: Task = Task {
        id: 0,
        title: "Basics".to_string(),
        description: "Starting rust".to_string(),
        assignee: "Ar11rA".to_string(),
        status: TaskStatus::Ready,
    };
    println!("{:?} to do {} by {}", task.status, task.title, task.assignee)
}

pub fn update_status(status: TaskStatus) {
    let mut task: Task = Task {
        id: 0,
        title: "Basics".to_string(),
        description: "Starting rust".to_string(),
        assignee: "Ar11rA".to_string(),
        status: TaskStatus::Ready,
    };

    task.status = status;
    let result: Option<&str> = match task.status {
        TaskStatus::Completed => { Some("done!") }
        _ => { None }
    };
    // Equivalent of Optional<T>
    if let Some(z) = result {
        println!("Got {}", z);
    } else {
        println!("Still working...");
    }
}

pub fn array_ops(){
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Length {}", a.len());
    println!("Array {:?}", a);
    a[2] = 10;
    println!("Array {:?}", a);
}

pub fn slice_ops(input: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for i in input {
        sum = sum + *i;
    }
    println!("Sum: {}", sum);
    return sum;
}

pub fn tuple_ops() {
    let x = 3;
    let y = 4;
    let r = (x + y, x * y);
    println!("Sum {} Product {}", r.0, r.1);
}