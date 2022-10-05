use std::collections::HashMap;
/// The Text Component API

pub trait Formattable {
    fn fmt(&self) -> Formatting; 
}

pub trait Renderable {
    fn render(&self) -> String;
}

pub enum Component <'a> {
    SingletonComponent {
        id: &'static str,
        data: &'a dyn Renderable 
    },
    ListComponent {
        id: &'static str,
        data: Vec<&'a dyn Renderable>,
        separator: &'static str,
    },
    Empty {
        id: &'static str,
    },
}

impl Component<'_> {
    pub fn  if_true<'b>(b: bool, result: &'b dyn Renderable, id: &'static str) -> Self {
        if b {
            let own = result.render().to_owned();
            Component::SingletonComponent { id, data: &own }
        } else {
            Component::Empty { id }
        }
    }

    pub fn if_false(b: bool, result: &dyn Renderable, id: &'static str) -> Self {
        if !b {
            result.render().as_component(id)
        } else {
            Component::Empty { id }
        }
    }

    pub fn from_vec(vec: Vec<&dyn Renderable>, id: &'static str, empty: &dyn Renderable , sep: &'static str) -> Self {
        if vec.is_empty() {
            empty.render().as_component(id)
        } else {
            Component::ListComponent { id: id, data: vec, separator: sep }
        }
    }

    pub fn get_id(&self) -> &'static str {
        match self {
            Component::SingletonComponent { id, data } => id,
            Component::ListComponent { id, data, separator } => id,
            Component::Empty { id } => id,
        }
    }
}

impl Renderable for Component<'_> {
    fn render(&self) -> String {
        match self {
            Component::SingletonComponent { id, data } => data.render(),
            Component::ListComponent {
                id,
                data,
                separator,
            } => {
                let rendered : Vec<String> = data.iter().map(|x| x.render()).collect();
                rendered.join(separator)
            },
            Component::Empty { id } => "".to_string(),
        }
    }
}

pub trait TextComponent {
    fn as_component(&self, id: &'static str) -> Component;
}


impl Renderable for &'static str {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl Renderable for String {
    fn render(&self) -> String {
        self.clone()
    }
}

impl TextComponent for &'static str {
    fn as_component(&self, id: &'static str) -> Component {
        Component::SingletonComponent {
            id,
            data: self,
        }
    }
}

impl TextComponent for String {
    fn as_component(&self, id: &'static str) -> Component {
        Component::SingletonComponent {
            id,
            data: self,
        }
    }
}
pub struct Formatting<'a> {
    lines: Vec<Line<'a>>,
    currentLine : usize,
}

// impl Renderable for Formatting  {
//     fn render(&self) -> String {
//         let lines: Vec<String> = self.lines.iter().map(|line| line.render()).collect();
//         lines.join("\n")
//     }
// }

pub struct Line<'a> {
    components: HashMap<&'static str, Component<'a>>
}

// impl Renderable for Line {
//     fn render(&self) -> String {
//         let renders : Vec<String> = self.components.iter().map(|(_,v)| (v.render()).clone()).collect();
//         renders.join(" ")
//     }
// }

impl Line<'_> {
    pub fn get(&self, id: &'static str) -> Option<&Component> {
        self.components.get(id)
    }

    pub fn add(&mut self, component: Component) {
        if !self.components.contains_key(component.get_id()) {
            self.components.insert(component.get_id(), component);
        } else {
            panic!("Duplicate component key {}", component.get_id())
        }
    }

    pub fn remove(&mut self, id: &'static str) {
        self.components.remove(id);
    }

    pub fn new() -> Self {
        Line { components: HashMap::new() }
    }
}

impl Formatting<'_> {
    fn push(&mut self, line: Line) {
        self.lines.push(line);
    }

    fn pop(&mut self) {
        self.lines.pop();
    }

    pub fn new() -> Self {
        Formatting { lines: vec![], currentLine: 0 }
    }

    pub fn add(&mut self, component: Component) {
        if self.lines.is_empty() {
            self.push(Line::new())
        }

        let line = self.lines.get_mut(self.currentLine).unwrap();
        line.add(component)
    }

    pub fn remove(&mut self, id: &'static str) {
        let line = self.lines.get_mut(self.currentLine).unwrap();
        line.remove(id);
    }

    pub fn newline(&mut self) {
        self.push(Line::new());
        self.currentLine += 1;
    }

    pub fn dropline(&mut self) {
        self.pop();
        self.currentLine -= 1;
    }
}
