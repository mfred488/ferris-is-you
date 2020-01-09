enum Object {
    FERRIS,
}

fn get_printable_character(object: &Object) -> String {
    match object {
        Object::FERRIS => return String::from("🦀"),
    };
}

struct ObjectWithLocation {
    x: usize,
    y: usize,
    object: Object,
}

struct Level {
    width: usize,
    height: usize,
    objects_with_locations: Vec<ObjectWithLocation>,
}

impl Level {
    fn new(width: usize, height: usize) -> Level {
        Level {
            width,
            height,
            objects_with_locations: Vec::new(),
        }
    }

    fn add_object(&mut self, x: usize, y: usize, object: Object) {
        self.objects_with_locations
            .push(ObjectWithLocation { x, y, object })
    }

    fn print(&self) {
        let mut characters: Vec<Vec<String>> = Vec::with_capacity(self.width);
        for _ in 0..self.width {
            let mut line = Vec::with_capacity(self.height);
            for _ in 0..self.height {
                line.push(String::from(" "));
            }
            characters.push(line);
        }

        for object_with_location in &self.objects_with_locations {
            characters[object_with_location.x][object_with_location.y] =
                get_printable_character(&object_with_location.object);
        }

        for y in 0..self.height {
            let mut line = String::with_capacity(self.width);
            for x in 0..self.width {
                line.push_str(&characters[x][y]);
            }
            println!("{}", line);
        }
    }
}

fn main() {
    let mut level = Level::new(10, 10);
    level.add_object(2, 3, Object::FERRIS);
    level.add_object(6, 6, Object::FERRIS);
    //println!("{}", get_printable_character(&Object::FERRIS));
    level.print();
}
