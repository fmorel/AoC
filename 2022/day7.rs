use file_utils::file_to_lines;
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
struct File
{
    _name: String,
    size: u32,
}

#[derive(Debug)]
struct Directory
{
    subdirs: Vec<String>,   /* keys to other elements of the filesystem */
    files: Vec<File>,
    size: u32,   /* Total size of the directory */
}


struct Terminal
{
    path: Vec<String>,  /* Current path */
    filesystem: HashMap<String, Directory>
}

impl Terminal {
    fn get_full_dir(&self) -> String
    {
        self.path.iter().fold(String::from(""), |full, dir| full + "/" + &dir)
    }

    fn parse_command(&mut self, command: &[&str])
    {
        /* cd changes cur_dir and path
         * ls creates an instance of cur_dir in the filesystem, ready to be populated */
        match command[0] {
            "cd" => match command[1] {
                        ".." => {self.path.pop(); ()}
                        "/"  => self.path.clear(),
                        dir  => self.path.push(String::from(dir))
                    }
            "ls" => {
                        let dir = Directory {
                            subdirs: Vec::new(),
                            files: Vec::new(),
                            size: 0,
                        };
                        self.filesystem.insert(self.get_full_dir(), dir);
                    }
            s => panic!("Unknown command {}", s),
        }
    }

    fn parse_line(&mut self, line: &str)
    {
        let tokens : Vec<&str> = line.split_ascii_whitespace().collect();
        let full_dir = self.get_full_dir();
        match tokens[0] {
            "$"   => self.parse_command(&tokens[1..]),
            "dir" => {
                        let dir = self.filesystem.get_mut(&full_dir).unwrap();
                        let subdir_name = full_dir.clone() + "/" + tokens[1];
                        dir.subdirs.push(subdir_name);
                     }
            s     => {
                        let file = File {
                            _name: String::from(tokens[1]),
                            size: s.parse::<u32>().unwrap()
                        };
                        let dir = self.filesystem.get_mut(&full_dir).unwrap();
                        dir.files.push(file);
                      }
        }
    }

    fn compute_dir_size_rec(&mut self, dir_name: &str) -> u32
    {
        /* We cannot keep a mutable reference to dir and perform the recursion at the same time
         * (only a single mutable reference are not allowed)
         * So we first take an immutable reference to retrieve the file sizes and the list of subdirs */
        let dir = self.filesystem.get(dir_name).unwrap();
        /* Early return if size is already known */
        if dir.size > 0 {
            return dir.size
        }
        let mut s = 0;
        for f in dir.files.iter() {
            s += f.size;
        }
        if !dir.subdirs.is_empty() {
            let subdirs = dir.subdirs.clone();
            /* dir is no longer used as a reference, we can perform the recursive call borrowing a
             * mutable reference to self*/
            for d in subdirs {
                s += self.compute_dir_size_rec(&d);
            }
        }
        /* Recursion has finished, retrieve mutable dir reference to update dir.size */
        let mut_dir = self.filesystem.get_mut(dir_name).unwrap();
        mut_dir.size = s;
        s
    }
            
    fn compute_fs_size(&mut self) -> u32
    {
        self.compute_dir_size_rec("")
    }
}

pub fn day7(filename: &Path)
{
    let lines = file_to_lines(filename);
    let mut term = Terminal {
        path: Vec::new(),
        filesystem: HashMap::new()
    };
    for l in lines {
      term.parse_line(l.as_str());  
    }
    let total_size = term.compute_fs_size();
    println!("Total size: {}", total_size);
    //println!("{:#?}", term.filesystem);
    let size_under_x = term.filesystem.values().fold(0, |size, dir| {
                                                            if dir.size <= 100000 { 
                                                                size + dir.size
                                                            } else {
                                                                size
                                                            }
                                                        });
    println!("Sum of sizes under 100000: {}", size_under_x);
    let update_size = 30000000 - (70000000 - total_size);
    /* Choose the smallest directory above update_size */
    let mut dirs = term.filesystem.values().collect::<Vec<&Directory>>();
    dirs.sort_by(|a, b| a.size.cmp(&b.size));
    for d in dirs {
        if d.size > update_size {
            println!("Directory above required size {} has size {}", update_size, d.size);
            break;
        }
    }
}
