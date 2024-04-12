pub fn get_template(template_name: String) -> std::vec::Vec<String> {
    let template = template_name;
    let template_path = "./templates/".to_owned() + &template + ".tpl";
    let template_path = std::path::Path::new(&template_path);

    let mut result = Vec::new();

    for line in std::fs::read_to_string(template_path).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn tree(path: std::path::PathBuf, indent: usize) {
    let mut ancestors = path.ancestors();
    if ancestors.next().is_none() {
        return;
    }
    else {
        println!("{:indent$}> {}", " ".repeat(indent), path.display());
        tree(ancestors.next().unwrap().to_path_buf(), indent + 2); // ancestors.next()
    }
    
    //}
}

pub fn pprint(template_name: String) {
    let template = template_name;
    let template_path = "./templates/".to_owned() + &template + ".tpl";
    let template_path = std::path::Path::new(&template_path);
    println!("> {}", template_path.display());
    let lines = std::fs::read_to_string(&template_path).unwrap();
    for line in lines.lines() {
        let path = std::path::PathBuf::from(line);
        //for ancestor in path.ancestors() {
            //println!("{}", ancestor.display());
        //}

        tree(path, 2);
    }
}
