// 项目名称: Create a Rust Project Repository
// 创建一个名为"Create a Rust Project Repository"的GitHub仓库

fn main() {
    // 引入必要的库
    use std::fs;
    use std::path::PathBuf;

    // 获取当前目录下的所有文件和子目录
    let files = fs::read_dir().expect("读取文件夹中的所有子文件失败");

    for file in files {
        match file {
            Ok(file) => {
                println!("文件: {}", file.path.to_str());
                let content = &file.file_name();
                if content.as_slice() == b"main.rs" {
                    let path = file.path.clone().unwrap();
                    println!("正在处理 Rust 文件: {}", path);
                    // 这里可以添加代码来处理该文件的内容，例如：
                    // use std::fs;
                    // let source = fs::read_to_string(path).expect("无法读取文件");
                    // println!("{}", source);
                }
            }
            Err(e) => {
                eprintln!("读取文件夹中的所有子文件失败: {}", e);
            }
        }
    }

    println!("已处理完成.");
}
