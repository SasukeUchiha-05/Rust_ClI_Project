use std::io;
fn main() {
    println!("Hello, world!");

    let mut todo_list: Vec<String> = Vec::new();

    loop{
        println!("\n--To Do List--\n");
        println!("1. Add a task");
        println!("2. View Tasks");
        println!("3. Remove a Task");
        println!("4. Exit");

        let choice = get_user_input("Enter your choice: ");

        match choice.trim() {
            "1"=>{
                let task = get_user_input("Enter a task to add: ");
                todo_list.push(task);
                println!("task added successfully!");
            }
            "2"=>{
                println!("\nTasks:");
                if todo_list.is_empty()
                {
                    println!("No Tasks Found. Please add some.");
                }
                else{
                    for(i,task) in todo_list.iter().enumerate(){
                        println!("{}. {}",i+1,task);
                    }
                }
            }
            "3" =>{
                if todo_list.is_empty()
                {
                    println!("No tasks to remove!");
                }
                else{
                    let task_number = get_user_input("Enter Task number to remove :");
                    match task_number.trim().parse::<usize>(){
                        Ok(num) if num>0 && num<=todo_list.len()=>{
                            todo_list.remove(num-1);
                            println!("Task removes successfully!");
                        }
                        _=>print!("Invalid Task Number!"),
                    }
                }
            }
            "4"=>{
                println!("GoodBye!");
                break;
            }
            _=>println!("Invalid Choice!"),
        }


    }

}
fn get_user_input(prompt:&str)->String
{
    println!("{}",prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
