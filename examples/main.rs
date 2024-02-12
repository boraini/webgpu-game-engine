pub mod triangle;
pub mod dice;

struct ExampleDesc {
    name: &'static str,
    function: fn(),
    #[allow(dead_code)] // isn't used on native
    webgl: bool,
    #[allow(dead_code)] // isn't used on native
    webgpu: bool,
}

const EXAMPLES: &[ExampleDesc] = &[ExampleDesc {
    name: "triangle",
    function: crate::triangle::main,
    webgl: false, // No compute
    webgpu: true,
},

ExampleDesc {
    name: "dice",
    function: crate::dice::main,
    webgl: false, // No compute
    webgpu: true,
}];

fn get_example_name() -> Option<String> {
    std::env::args().nth(1)
}

fn main() {
    let Some(example) = get_example_name() else {
        println!("Please provide and example name.");
        return;
    };

    let Some(found) = EXAMPLES.iter().find(|e| e.name == example) else {
        println!("Example {} not known.", example);
        return;
    };

    (found.function)();
}
