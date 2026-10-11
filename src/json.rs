/// this file will manage functionalities of json file (Serialized and Deserialized) json file


struct Options {
    cmd : String,
    deps : Vec<String>
}

#[derive(Debug)]
struct Task {
    id : i32,
    options : Vec<Option>
}

#[derive(Debug)]
struct Template {
    limite_tasks : Option<i32>,
    tasks : Vec<Task>
}

fn load(file_path : str) -> Result<Template> {
    Ok(())
}

fn unload(template : Template) -> Result<()> {
    Ok(())
}
