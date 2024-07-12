use std::env::args;
use std::fs::File;

use walkdir::WalkDir;

fn get_file_size(path: &str) -> Option<u64>
{
    let file = match File::open(path)
    {
        Ok(f) => f,
        Err(_) => return None,
    };

    match file.metadata()
    {
        Ok(md) => Some(md.len()),
        Err(_) => None,
    }
}

fn push_if_big_enough(file_infos: &mut Vec<(String, u64)>, info: (String, u64), capacity: usize)
{
    file_infos.push(info);
    file_infos.sort_by(|(_name1, size1), (_name2, size2)| size2.cmp(size1));

    if file_infos.len() > capacity
    {
        file_infos.pop();
    }
}

fn get_file_size_fmt(size: u64) -> String
{
    let mut size = size;

    let suffixes = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    for suffix in suffixes
    {
        if size < 1024
        {
            return format!("{size}{suffix}");
        }
        size /= 1024;
    }

    return "???".to_string();
}

fn main()
{
    let usage = "searchmaster [path] [entries count]";

    let args: Vec<String> = args().collect();
    if args.len() < 2
    {
        eprintln!("{usage}");
        return;
    }

    let root_path = &args[1];
    let data_capacity = if args.len() >= 3
    {
        args[2].parse().unwrap()
    }
    else { 10 };



    let mut file_infos: Vec<(String, u64)> = vec![];

    for entry in WalkDir::new(root_path)
    {
        let entry = match entry
        {
            Ok(e) => e,
            Err(_) => continue,
        };

        let is_directory = !entry.file_type().is_file();
        if is_directory
        {
            continue;
        }

        let name = entry.path().to_str().unwrap();

        if let Some(size) = get_file_size(name)
        {
            let file_info = (name.to_string(), size);
            push_if_big_enough(&mut file_infos, file_info, data_capacity);
        }
    }

    for info in file_infos
    {
        let (name, size) = info;
        let size = get_file_size_fmt(size);

        println!("{} {}", name, size);
    }
}
