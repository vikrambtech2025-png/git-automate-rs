
use std::process::{exit, Command};
use names::Generator;

fn update_commit_push() {
    // 1. Stage all changes
    let add_command = Command::new("git")
        .args(["add", "-A"])
        .status()
        .expect("Failed to execute git add");

    if !add_command.success() {
        eprintln!("ERROR: Failed to add files to the Git repo");
        exit(1);
    }

    // 2. Generate commit message
    let commit_message = name_generator();

    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .status()
        .expect("Failed to execute git commit");

    if !commit_command.success() {
        eprintln!("ERROR: Failed to commit changes");
        exit(1);
    }

    // 3. Detect current branch
    let branch_output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .expect("Failed to detect Git branch");

    if !branch_output.status.success() {
        eprintln!("ERROR: Failed to detect Git branch");
        exit(1);
    }

    let branch = String::from_utf8_lossy(&branch_output.stdout)
        .trim()
        .to_string();

    // 4. Push changes
    let push_command = Command::new("git")
        .args(["push", "origin", &branch])
        .status()
        .expect("Failed to execute git push");

    if !push_command.success() {
        eprintln!("ERROR: Failed to push changes");
        exit(1);
    }

    println!("Successfully added, committed and pushed changes!");
    println!("Commit message: {}", commit_message);
}

fn name_generator() -> String {
    let mut generator = Generator::default();
    generator.next().expect("Failed to generate commit name")
}

fn main() {
    update_commit_push();
}
