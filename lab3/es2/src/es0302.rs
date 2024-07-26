use std::time::SystemTime;
use itertools::Itertools;


#[derive(PartialEq, Clone)]
struct File 
{
    name: String,
    modified: SystemTime,
    content: Vec<u8>,
}

#[derive(PartialEq, Clone)]
struct Dir 
{
    name: String,
    modified: SystemTime,
    children: Vec<Node>,
}

// Define this enum in order to be able to store different types in the same vector

#[derive(PartialEq, Clone)]
enum Node 
{
    File(File),
    Dir(Dir),
}


#[derive(Debug)]
enum FSError 
{
    NotFound,     // file or dir not found
    NotADir,      // when trying to ad children to a file
    Duplicate,    // duplicate name in dir
    DirNotEmpty,  // try to remove a dir with children
    GenericError, // generic error
}

// define lifetimes
struct MatchResult<'a>
{
    q: &'a str, // matched query string
    path: String, // matched path
    node: &'a Node, // matched node
}

struct Filesystem 
{
    root: Dir,
}


impl File
{
    fn new(name: &str) -> Self
    {
        File {name: name.to_string(), modified: SystemTime::now(), content: Vec::<u8>::new()}
    }


    pub fn touch(&mut self)
    {
        self.modified = SystemTime::now();
    }
}


impl Dir
{
    pub fn new(name: &str) -> Self
    {
        Self {name: name.to_string(), modified: SystemTime::now(), children: Vec::<Node>::new()}
    }


    pub fn touch(&mut self)
    {
        self.modified = SystemTime::now();
    }

}


impl Filesystem 
{
    // create a new empty filesystem with a root dir
    // (name of the root dir is empty string: "")
    pub fn new() -> Self 
    {
        Self {root: Dir::new("") }
    }


    // create a new filesystem reading from disk all the structure under the given path
    // in the file content just write the firt 1k bytes of the file
    // return the root node of the filesystem
    // (implement this function at the end, after all the other methods, the only purpose is to take a look std::fs functions, use std::fs:read_dir)
    pub fn from(path: &str) -> Self 
    {
        unimplemented!()
    }


    fn split_path<'a>(&self, path: &'a str) -> Vec<&'a str>
    {
        let mut dirs = vec![];
        if path == "/"
        {
            return dirs;
        }
        else 
        {
            dirs = path.split("/").skip(1).collect::<Vec<&str>>();
            dirs
        }
    }


    // Returns parent directory or NotFound / NotADir
    fn search_dir(&mut self, path: &str) -> Result<&mut Dir, FSError>
    {
        let dir_names = self.split_path(path);
        let mut current_dir = &mut self.root;

        for dir_name in dir_names
        {
            let res = current_dir.children.iter_mut().find(|n|
            {
                match n
                {
                    Node::Dir(d) => d.name == dir_name,
                    Node::File(f) => f.name == dir_name
                }
            });
            match res
            {
                Some(Node::Dir(dir)) => 
                {
                    current_dir = dir;
                },
                Some(Node::File(_)) => return Err(FSError::NotADir),
                None => return Err(FSError::NotFound)
            }
        }
        Ok(current_dir)
    }


    // create a new directory in the filesystem under the given path
    // return a reference the created dir
    // possible errors: NotFound, path NotADir, Duplicate
    pub fn mkdir(&mut self, path: &str, name: &str) -> Result<&mut Dir, FSError> 
    {
        match self.search_dir(path)
        {
            Ok(parent) => 
            {
                if parent.children.iter().any(|n| match n
                    {
                        Node::Dir(d) => d.name == name,
                        _ => false
                    }
                )
                {
                    Err(FSError::Duplicate)
                }
                else 
                {
                    parent.children.push(Node::Dir(Dir::new(name)));
                    let new_dir = parent.children.iter_mut().find(|n| match n
                        {
                            Node::Dir(d) => d.name == name,
                            _ => false
                        }
                    );
                    match new_dir
                    {
                        Some(Node::Dir(d)) => Ok(d),
                        // Should never occur
                        _ => Err(FSError::GenericError)
                    }
                }
            },
            Err(e) => Err(e)
        }
    }


    // possible errors: NotFound, path is NotADir, Duplicate
    pub fn create_file(&mut self, path: &str, name: &str) -> Result<&mut File, FSError> 
    {
        match self.search_dir(path)
        {
            Ok(parent) => 
            {
                if parent.children.iter().any(|n| match n
                    {   
                        Node::File(f) => f.name == name,
                        _ => false
                    })
                {
                    Err(FSError::Duplicate)
                }
                else
                {
                    parent.children.push(Node::File(File::new(name)));
                    let new_file = parent.children.iter_mut().find(|n| match n
                        {
                            Node::File(f) => f.name == name,
                            _ => false
                        }
                    );
                    match new_file
                    {
                        Some(Node::File(f)) => Ok(f),
                        // Should never occur
                        _ => Err(FSError::GenericError)
                    }
                }
            },
            Err(e) => Err(e)
        }
    }


    // updated modification time of the file or the dir
    // possible errors: NotFound
    pub fn touch(&mut self, path: &str) -> Result<(), FSError> 
    {
        let parts = self.split_path(path);
        let dir_names = parts[..parts.len() - 1].join("/");
        let name = parts[parts.len() - 1];

        match self.search_dir(&dir_names)
        {
            Ok(parent) => 
            {
                match parent.children.iter_mut().find(|n| match n
                {
                    Node::File(f) => f.name == name,
                    Node::Dir(d) => d.name == name
                })
                {
                    Some(Node::File(f)) => 
                    {
                        f.touch();
                        Ok(())
                    },
                    Some(Node::Dir(d)) => 
                    {
                        d.touch();
                        Ok(())
                    },
                    _ => Err(FSError::NotFound)
                }
            },
            Err(_) => Err(FSError::NotFound)
        } 
    }


    // remove a node from the filesystem and return it
    // if it's a dir, it must be empty
    // possible errors: NotFound, DirNotEmpty
    pub fn delete(&mut self, path: &str) -> Result<Node, FSError> 
    {
        let parts = self.split_path(path);
        let dir_names = parts[..parts.len() - 1].join("/");
        let name = parts[parts.len() - 1];

        match self.search_dir(&dir_names)
        {
            Ok(parent) => 
            {
                match parent.children.iter_mut().position(|n| match n
                    {
                        Node::File(f) => f.name == name,
                        Node::Dir(d) => d.name == name
                    })
                {
                    Some(pos) => 
                    {
                        match &parent.children[pos]
                        {
                            Node::Dir(d) => 
                            {
                                if d.children.is_empty()
                                {
                                    let target = Node::Dir(d.clone());
                                    Ok(target)
                                }
                                else 
                                {
                                    Err(FSError::DirNotEmpty)    
                                }
                            },
                            Node::File(f) => 
                            {
                                let target = Node::File(f.clone());
                                Ok(target)
                            }
                        }
                    },
                    None => Err(FSError::NotFound)
                }
            },
            Err(e) => Err(FSError::NotFound)
        } 
    }


    // get a reference to a node in the filesystem, given the path
    pub fn get(&mut self, path: &str) -> Result<&Node, FSError> 
    {
        let parts = path.split("/").collect::<Vec<&str>>();
        let dir_names = parts[..parts.len() - 1].join("/");
        let name = parts[parts.len() - 1];

        match self.search_dir(&dir_names)
        {
            Ok(d) => 
            {
                match d.children.iter().find(|n| match n
                {
                    Node::Dir(d) => d.name == name,
                    Node::File(f) => f.name == name
                })
                {
                    Some(n) => Ok(n),
                    None => Err(FSError::NotFound)
                }
            },
            Err(e) => Err(e)
        }
    }


    // get a mutable reference to a node in the filesystem, given the path
    pub fn get_mut(&mut self, path: &str) -> Result<&mut Node, FSError> 
    {
        let parts = path.split("/").collect::<Vec<&str>>();
        let dir_names = parts[..parts.len() - 1].join("/");
        let name = parts[parts.len() - 1];

        match self.search_dir(&dir_names)
        {
            Ok(d) => 
            {
                match d.children.iter_mut().find(|n| match n
                {
                    Node::Dir(d) => d.name == name,
                    Node::File(f) => f.name == name
                })
                {
                    Some(n) => Ok(n),
                    None => Err(FSError::NotFound)
                }
            },
            Err(e) => Err(e)
        }
    }


    // search for a list of paths in the filesystem
    // qs is a list query strings with constraints
    // the constraints must be matched in or (it's returned any node matching at least one constraint)
    // constraint format: "type:pattern"
    // constraints:
    // - "type:dir" -> match only directories
    // - "type:file" -> match only files
    // - "name:value" -> match only nodes with the given name
    // - "partname:value" -> match only nodes with the given string in the name
    pub fn find<'a>(&'a self, qs: &[&'a str]) -> Vec<MatchResult> 
    {
        let mut mr_array = Vec::<MatchResult>::new();
        let mut nodes: Vec<&Node> = Vec::new();
        let mut paths: Vec<String> = Vec::new();
        let mut fit: bool = false;

        if self.root.children.is_empty()
        {
            return mr_array;
        }

        // Inizializzazione dei nodi e rispettivi percorsi
        let mut children = self.root.children.iter().collect::<Vec<&Node>>();
        nodes.append(&mut children);
        for n in children
        {
            let mut path = "/".to_string();
            match n
            {
                Node::Dir(d) => path += &d.name,
                Node::File(f) => path += &f.name
            }
            paths.push(path);
        }
        for q in qs
        {
            let (key, val) = q.split(":").collect_tuple::<(&str, &str)>().unwrap();
            while let Some(n) = nodes.pop()
            {
                let path = paths.pop().unwrap();

                // Controllo se il nodo soddisfa la query
                fit = false;
                if key == "type" && val == "dir"
                { 
                    match n
                    {
                        Node::Dir(_) => fit = true,
                        _ => {}
                    }
                }
                else if key == "type" && val == "file"
                {
                    match n
                    {
                        Node::File(_) => fit = true,
                        _ => {}
                    }
                }
                else if key == "name"
                {
                    match n
                    {
                        Node::Dir(d) => fit = d.name == val,
                        Node::File(f) => fit = f.name == val
                    }
                }
                else if key == "partname"
                {
                    match n
                    {
                        Node::Dir(d) => fit = d.name.contains(val),
                        Node::File(f) => fit = f.name.contains(val)
                    }
                }

                // Se il nodo soddisfa la query viene inserito nei MatchResults
                if fit
                {
                    let mr = MatchResult {q: q, path: path.clone(), node: n};
                    mr_array.push(mr);
                }

                // Inserimento dei children del nodo corrente nel vettore dei nodi
                let mut children: Vec<&Node> = Vec::new();
                if let Node::Dir(d) = n
                {
                    children = d.children.iter().collect::<Vec<&Node>>();
                    nodes.append(&mut children);
                }
                
                // Inserimento dei path dei children del nodo corrente nel vettore paths
                for n in children.iter()
                {
                    let mut new_path = path.clone();
                    match n
                    {
                        Node::Dir(d) => new_path = new_path + "/" + &d.name,
                        Node::File(f) => new_path = new_path + "/" + &f.name
                    }
                    paths.push(new_path);
                }
            }
        }
        return mr_array;
    }


    // walk the filesystem, starting from the root, and call the closure for each node with its path
    // the first parameter of the closure is the path of the node, second is the node itself
    pub fn walk(&self, f: impl Fn(&str, &Node)) 
    {
        let mut nodes: Vec<&Node> = Vec::new();
        let mut paths: Vec<String> = Vec::new();

        if self.root.children.is_empty()
        {
            return;
        }

        // Inizializzazione dei nodi e rispettivi percorsi
        let mut children = self.root.children.iter().collect::<Vec<&Node>>();
        nodes.append(&mut children);
        for n in children
        {
            let mut path = "/".to_string();
            match n
            {
                Node::Dir(d) => path += &d.name,
                Node::File(f) => path += &f.name
            }
            paths.push(path);
        }

        // Visita dei nodi
        while let Some(n) = nodes.pop()
            {
                let path = paths.pop().unwrap();

                // Closure
                f(&path, n);

                // Inserimento dei children del nodo corrente nel vettore dei nodi
                let mut children: Vec<&Node> = Vec::new();
                if let Node::Dir(d) = n
                {
                    children = d.children.iter().collect::<Vec<&Node>>();
                    nodes.append(&mut children);
                }
                
                // Inserimento dei path dei children del nodo corrente nel vettore paths
                for n in children.iter()
                {
                    let mut new_path = path.clone();
                    match n
                    {
                        Node::Dir(d) => new_path = new_path + "/" + &d.name,
                        Node::File(f) => new_path = new_path + "/" + &f.name
                    }
                    paths.push(new_path);
                }
            }

    }
}


pub fn demo() 
{

    let mut fs = Filesystem::new();

    // create a directory structure, 10 dirs with a child dir and file each one
    for i in 0..10 
    {
        fs.mkdir("/", format!("dir{}", i).as_str()).unwrap();
        fs.mkdir(format!("/dir{}", i).as_str(), "child1").unwrap();
        fs.create_file(format!("/dir{}", i).as_str(), "file1").unwrap();
    }

    println!("find /child2");
    if let Ok(res) = fs.get_mut("/dir2/child1") 
    {
        match res 
        {
            Node::Dir(d) => 
            {
                d.name = "dir2 found".to_string();
            }
            // try to match all possible errros
            _ => {}
        }
    } 
    else 
    {
        println!("not found");
    }

    // let's try with matches
    let matches = fs.find(&["name:child1", "type:file"]);
    for m in matches {
        match m.node {
            Node::File(f) => {
                // inspect content
            },
            Node::Dir(d) => {
                // inspect children
            },
            _ => {}
        }
    }

    // see note "riferimenti mutabili" in exercise text 
    // now let's try to modify the filesystem using the found matches
    // is it possible to do it? which error do you get from the compiler?
    // let matches = fs.find(&["/dir2/child1", "/dir3/child1"]);
    // for m in matches 
    // {
    //     let node = fs.get_mut(m.path).unwrap();
    //     match node {
    //         Node::File(f) => {
    //             // inspect content
    //         }
    //         _ => {}
    //     }
    // }
    
    // how can you fix the previous code?
    // suggestion: this code using paths which are not referenced by MatchResults should compile. Why?
    // Therefore how can you use the paths returned in the MatchResults to modify the filesystem?
    // let paths = ["/dir1/child1", "/dir2/child1", "/dir3/child1"];
    // for p in paths 
    // {
    //     let n = fs.get_mut(p.as_str());
    // }


    // now let's try to walk the filesystem
    fs.walk(|path, node| 
    {
        match node 
        {
            Node::File(f) => 
            {
                println!("file: {}", path);
            }
            Node::Dir(d) => 
            {
                println!("dir: {}", path);
            }
        }
    });

}

