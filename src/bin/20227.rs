use advent_of_code::load_data;
use std::{cell::RefCell, error::Error, fmt::Display, io::Read, rc::Rc, str::FromStr};

const ADVENT_NUM: &str = "20227";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = load_data(ADVENT_NUM, "input.txt")?;
    let mut text_log = String::new();
    file.read_to_string(&mut text_log)?;

    let root: Dir = text_log.parse()?;
    println!("{root}");

    println!(
        "{:?}",
        root.all_dirs()
            .iter()
            .map(|f| f.borrow().size())
            .filter(|f| *f >= 8_008_081)
            .collect::<Vec<u64>>()
            .iter()
            .min()
    );
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct Dir(String, Vec<Rc<RefCell<Dir>>>, Vec<(String, u64)>);

impl Dir {
    pub fn new(name: String) -> Dir {
        Dir(name, Vec::new(), Vec::new())
    }

    pub fn add_dir(&mut self, name: &str) -> Rc<RefCell<Dir>> {
        for dir in self.1.iter() {
            if dir.borrow().0 == name {
                return dir.clone();
            }
        }
        let new_dir = Rc::new(RefCell::new(Dir::new(name.to_string())));
        self.1.push(new_dir.clone());
        new_dir
    }

    pub fn size(&self) -> u64 {
        let mut result = 0;
        let mut stack = vec![Rc::new(RefCell::new(self.clone()))];
        while let Some(current) = stack.pop() {
            for (_, size) in current.borrow().2.iter() {
                result += size;
            }

            for dir in current.borrow().1.iter() {
                stack.push(dir.clone());
            }
        }
        result
    }

    pub fn all_dirs(&self) -> Vec<Rc<RefCell<Dir>>> {
        let mut result = vec![Rc::new(RefCell::new(self.clone()))];
        let mut stack = vec![Rc::new(RefCell::new(self.clone()))];
        while let Some(current) = stack.pop() {
            for dir in current.borrow().1.iter() {
                stack.push(dir.clone());
                result.push(dir.clone());
            }
        }
        result
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stack = vec![(Rc::new(RefCell::new(self.clone())), 0)];
        while let Some((current, depth)) = stack.pop() {
            for _ in 0..depth {
                write!(f, "  |")?;
            }
            writeln!(f, "{} ({})", current.borrow().0, current.borrow().size())?;

            for file in current.borrow().2.iter() {
                for _ in 0..depth + 1 {
                    write!(f, "  |")?;
                }
                writeln!(f, "{file:?}",)?;
            }

            for dir in current.borrow().1.iter() {
                stack.push((dir.clone(), depth + 1));
            }
        }
        Ok(())
    }
}

impl FromStr for Dir {
    type Err = GenericParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let root = Dir::new("/".to_string());
        let root_rc = Rc::new(RefCell::new(root));
        let mut current_dir = root_rc.clone();
        let mut parent_dir = vec![root_rc.clone()];
        for line in input.lines() {
            match line.parse::<LineType>() {
                Ok(LineType::CdCommand(path)) => match path.as_str() {
                    "/" => {
                        if *current_dir != *root_rc {
                            current_dir = root_rc.clone();
                            parent_dir = vec![root_rc.clone()];
                        }
                    }
                    ".." => {
                        current_dir = match parent_dir.pop() {
                            Some(a) => a,
                            None => {
                                return Err(GenericParseError(
                                    "Couldn't go to parent dir".to_string(),
                                ))
                            }
                        }
                    }
                    name => {
                        let new_dir = current_dir.try_borrow_mut().unwrap().add_dir(name);
                        parent_dir.push(current_dir.clone());
                        current_dir = new_dir;
                    }
                },
                Ok(LineType::Dir(name)) => {
                    current_dir.try_borrow_mut().unwrap().add_dir(&name);
                    ()
                }

                Ok(LineType::File(name, size)) => {
                    current_dir.try_borrow_mut().unwrap().2.push((name, size))
                }
                _ => (),
            }
        }
        drop(parent_dir);
        if let Ok(root) = Rc::try_unwrap(root_rc) {
            return Ok(root.into_inner());
        } else {
            return Err(GenericParseError("Couldn't unwrap Rc".to_string()));
        }
    }
}

#[derive(Debug)]
struct GenericParseError(String);

impl Display for GenericParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GenericParseError {}

enum LineType {
    LsCommand,
    CdCommand(String),
    Dir(String),
    File(String, u64),
    NewLine,
}

impl FromStr for LineType {
    type Err = GenericParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Ok(LineType::NewLine);
        }

        match &s[0..=0] {
            "$" => match &s[2..=2] {
                "l" => return Ok(LineType::LsCommand),
                "c" => return Ok(LineType::CdCommand(s[5..].to_string())),
                _ => return Err(GenericParseError("Unknown Command".to_string())),
            },
            "d" => return Ok(LineType::Dir(s[4..].to_string())),
            _ => {
                if let Some((left, right)) = s.split_once(' ') {
                    return Ok(LineType::File(right.to_string(), left.parse().unwrap()));
                } else {
                    return Err(GenericParseError("Unknown LineType".to_string()));
                }
            }
        }
    }
}
