use names::Generator;
use std::process::{Command, exit};

fn working_tree_clean() -> bool {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .expect("Failed to check git status");

    if !output.status.success() {
        eprintln!("ERROR: Failed to check git status");
        exit(1);
    }

    output.stdout.is_empty()
}

fn has_origin_remote() -> bool {
    Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn current_branch() -> String {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .expect("Failed to detect Git branch");

    if !output.status.success() {
        eprintln!("ERROR: Failed to detect Git branch");
        exit(1);
    }

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn update_commit_push() {
    // Refuse detached HEAD before changing anything, so no stray commit is made.
    let branch = current_branch();
    if branch.is_empty() {
        eprintln!("ERROR: Detached HEAD - check out a branch before pushing");
        exit(1);
    }

    // 1. Stage all changes
    let add_command = Command::new("git")
        .args(["add", "-A"])
        .status()
        .expect("Failed to execute git add");

    if !add_command.success() {
        eprintln!("ERROR: Failed to add files to the Git repo");
        exit(1);
    }

    // Nothing to do when there are no changes. `git commit` would fail here.
    if working_tree_clean() {
        println!("Nothing to commit - working tree clean.");
        return;
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

    // 3. Push changes, if this repo has somewhere to push to
    if !has_origin_remote() {
        println!("No 'origin' remote configured - skipped push.");
        println!("Commit message: {}", commit_message);
        return;
    }

    let push_command = Command::new("git")
        .args(["push", "-u", "origin", &branch])
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

#[cfg(test)]
mod tests {
    use super::name_generator;

    #[test]
    fn generated_commit_message_is_not_empty() {
        let message = name_generator();
        assert!(!message.trim().is_empty());
    }
}
