mod app;
mod task;
mod task_store;

fn main() -> Result<(), app::AppError> {
    println!("=============================================");
    println!(" LESSON 20: PROJECT DEVELOPMENT");
    println!("=============================================");

    app::run_demo()?;

    println!("\nLesson completed successfully!");
    println!("Try commands: add <title>, done <id>, list");
    Ok(())
}
