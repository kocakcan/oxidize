pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button labeled: {}", self.label);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub option: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing a selectbox with width: {}, height: {}",
            self.width, self.height
        );
    }
}

pub struct Dropdown {
    pub width: u32,
    pub height: u32,
    pub items: Vec<String>,
}

impl Draw for Dropdown {
    fn draw(&self) {
        println!("Drawing a dropdown with collection: {:?}", self.items);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 30,
                height: 40,
                label: String::from("Test"),
            }),
            Box::new(SelectBox {
                width: 50,
                height: 60,
                option: vec![
                    String::from("Option #1"),
                    String::from("Option #2"),
                    String::from("Option #3"),
                ],
            }),
            Box::new(Dropdown {
                width: 70,
                height: 80,
                items: vec![
                    String::from("Ali"),
                    String::from("Veli"),
                    String::from("Deli"),
                    String::from("49 50"),
                ],
            }),
        ],
    };

    screen.run();
}
