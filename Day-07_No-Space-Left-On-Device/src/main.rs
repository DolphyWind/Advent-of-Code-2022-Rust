use std::{io, ptr};

// This code do not work. I tried a lot of times and eventualy solved this problem with c++.
// My mistake was not declaring variables on heap.
// I tried allocating but I get a lot of errors

#[derive(Clone)]
pub struct File{
    pub size: usize,
    pub name: String,
}

#[derive(Clone)]
pub struct Directory{
    pub parent: *mut Box<Directory>,
    pub subdirs: Vec<*mut Box<Directory>>,
    pub files: Vec<File>,
    pub name: String,
    pub path: String,
}

impl Directory{
    fn new(dirName: &str) -> Box<Directory>{
        return Box::new(Directory{
            parent: ptr::null_mut(),
            subdirs: Vec::new(),
            files: Vec::new(),
            name: dirName.to_string().clone(),
            path: format!("{}{}", dirName, "/"),
        });
    }

    unsafe fn addSubdir(&mut self, newSubDir: &mut Box<Directory>){
        for &dir in &self.subdirs{
            if (*dir).name == newSubDir.name{
                return;
            }
        }
        self.subdirs.push(newSubDir);
        newSubDir.parent = self;
        newSubDir.path = format!("{}{}", self.path, newSubDir.name);
        if newSubDir.path.chars().nth(newSubDir.path.len()-1).unwrap() != '/'{
            newSubDir.path.push('/');
        }
    }

    fn addFile(&mut self, filename: &str, filesize: usize){
        self.files.push(File{
            name: filename.to_string(),
            size: filesize,
        });
    }

    fn getFolderSize(&self) -> usize{
        let mut totalSize: usize = 0;
        for &d in &self.subdirs{
            unsafe{totalSize += (*d).getFolderSize();};
        }

        for f in &self.files{
            totalSize += f.size;
        }

        return totalSize;
    }

    fn printFilesystem(&mut self, tabCount: usize){
        println!("{}{}/", "\t".repeat(tabCount), self.name);
        for &d in &self.subdirs{
            unsafe {(*d).printFilesystem(tabCount + 1);};
        }
        for f in &self.files{
            println!("{}{} {}", "\t".repeat(tabCount + 1), f.name, f.size);
        }
    }
}

unsafe fn changeDir(dirptr: &mut *mut Directory, dirname: &str){
    if dirname == ".."{
        if (*(*dirptr)).parent.is_null(){
            return;
        }
        // I am completely lost
        (*dirptr) = (*(*dirptr)).parent;
        return;
    }
    for &d in &(*(*dirptr)).subdirs{
        unsafe{
            if dirname.to_string() == (*d).name {
                (*dirptr) = &mut (*d);
                break;
            }
        };
    }
}
//
// static mut parentFolder: Directory = Directory::new("");
// static mut currentDir: *mut Directory = &mut unsafe { parentFolder };

fn main() {
    let mut inputs: Vec<String> = Vec::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");
        input = input[0..input.len() - 1].to_string();
        if input.is_empty() {
            break;
        }
        inputs.push(input);
    }

    let mut parentDir: Directory = Directory::new("Parent");
    let mut currentDir: *mut Directory = &mut parentDir;
    let mut allDirs: Vec<Directory> = Vec::new();

    let mut somedir: Directory = Directory::new("Somedir");
    unsafe { parentDir.addSubdir(&mut Box::new(Directory::new("asd"))); }

    unsafe{
        for input in &inputs{
            let splitted: Vec<&str> = input.split(" ").collect();
            if splitted[0] == "$"{
                if splitted[1] == "cd"{
                    if splitted[2] == "/"{
                        continue;
                    }
                    println!("Changedir {} to {}", (*currentDir).name, splitted[2]);
                    changeDir(&mut currentDir, splitted[2]);
                }
            }
            else if splitted[0] == "dir"{
                println!("Adddir: {:?}", splitted);
                // somedir = Directory::new(splitted[1]);
                // (*currentDir).addSubdir(&mut somedir);
                parentDir.addSubdir(&mut *Box::new(Directory::new(splitted[1])))
            }
            else{
                println!("Addfile");
                let fsize: usize = splitted[0].parse().unwrap();
                let fname = splitted[1];
                (*currentDir).addFile(fname, fsize);
            }
        }
    };

    parentDir.printFilesystem(0);
    for d in &allDirs{
        println!("{}", d.name);
    }

    // unsafe {
    //     (*currentDir).printFilesystem(0);
    //     changeDir(&mut currentDir, "Child");
    //     (*currentDir).printFilesystem(0);
    //     changeDir(&mut currentDir, "..");
    //     (*currentDir).printFilesystem(0);
    //
    // };
}
