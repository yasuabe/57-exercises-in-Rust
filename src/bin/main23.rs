/* ----------------
# Ex23: Troubleshooting Car Issues
----------------
- Guide the user through a series of yes/no questions based on a decision tree.
- Ask only relevant questions based on previous answers.
- Display appropriate troubleshooting advice depending on the answers.
- Do not prompt for all inputs at once; follow the flow of the decision tree.
*/
use exercises_for_programmer::utils::std_util::read_input;
use once_cell::sync::Lazy;

enum Tree {
    Leaf(String),
    Node(String, Box<Tree>, Box<Tree>),
}

fn leaf(val: &str) -> Tree {
    Tree::Leaf(val.to_string())
}
fn node(val: &str, left: Tree, right: Tree) -> Tree {
    Tree::Node(val.to_string(), Box::new(left), Box::new(right))
}

static TREE: Lazy<Tree> = Lazy::new(|| {
    node(
        "Is the car silent when you turn the key? ",
        node(
            "Are the battery terminals corroded? ",
            leaf("Clean terminals and try starting again."),
            leaf("Replace cables and try again.")
        ),
        node(
            "Does the car make a clicking noise? ",
            leaf("Replace the battery."),
            node(
                "Does the car crank up but fail to start? ",
                leaf("Check spark plug connections."),
                node(
                    "Does the engine start and then die? ",
                    node(
                        "Does your car have fuel injection? ",
                        leaf("Get it in for service."),
                        leaf("Check to ensure the choke is opening and closing.")
                    ),
                    leaf("---")
                )
            )
        )
    )
});

fn traverse(tree: &Tree) -> String {
    match tree {
        Tree::Leaf(val) => val.to_string(),
        Tree::Node(val, left, right) => {
            let choice = read_input(val).to_uppercase();
            let next   = if choice == "Y" { left } else { right };

            traverse(next)
        }
    }
}
fn main() {
    let result = traverse(&TREE);
    println!("{}", result)
}
