use std::io;

//Array/Vector Impl of Family Tree


//Loop to take in user input and build "tree"
fn main() {
        println!("Please enter your name");

    let mut family_tree: Vec<String> = vec!["content_checker".to_string(); 7];
        let mut root = String::new();
        io::stdin().read_line(&mut root);

        family_tree[0] = root.trim().to_string();

        while true {
                println!("Next action please");

                let mut command_line = String::new();
                io::stdin().read_line(&mut command_line);
                let retry_input = command_line.trim_right().split(" ");
                let split:Vec<&str>  = retry_input.collect();

                if split[0] == "add" {
                        if !contains_node(family_tree.as_mut_slice(), split[3].to_string()) {
                                println!("Name not found");

                        }else if contains_node(family_tree.as_mut_slice(), split[1].to_string()) {
                                println!("Name already exists");
                        } else {
                        let test = tree_scanner(family_tree.as_mut_slice(), split[3].to_string());
                        if test == 0 {
                                if split[2] == "mother" {
                                        if family_tree[1] == "content_checker".to_string() {
                                                family_tree[1] = split[1].to_string();
                                        }else {
                                                println!("Relationship already exists");
                                        }
                                } else if split[2] == "father" {
                                        if family_tree[2] == "content_checker".to_string() {
                                                family_tree[2] = split[1].to_string();
                                        }else {
                                                println!("Relationship already exists");
                                        }
                                }else {
                                        println!("Invalid relationship");
                                }
                        }else if test == 1 {
                                if split[2] == "mother" {
                                        if family_tree[3] == "content_checker".to_string() {
                                                family_tree[3] = split[1].to_string();
                                        }else {
                                                println!("Relationship already exists");
                                        }
                                } else if split[2] == "father" {
                                        if family_tree[4] == "content_checker".to_string() {
                                                family_tree[4] = split[1].to_string();
                                        }else {
                                                println!("Relationship already exists");
                                        }
                                }else {
                                        println!("Invalid relationship");
                                }
                        }else {
                                if split[2] == "mother" {
                                        if family_tree[5] == "content_checker".to_string() {
                                                family_tree[5] = split[1].to_string();
                                        }else {
                                                println!("Relationship already exists");
                                        }
                                } else if split[2] == "father" {
                                        if family_tree[6] == "content_checker".to_string() {
                                                family_tree[6] = split[1].to_string();
                                        }else {
                                                println!("Relationship already exists");
                                        }
                                }else {
                                        println!("Invalid relationship");
                                }
                        }
    }
        } else if split[0] == "delete" {
   if !contains_node(family_tree.as_mut_slice(), split[1].to_string()) {
                        println!("Name not found");
                        continue;
                }
                if family_tree[0] == split[1].to_string() {
                        println!("Deletion failed");
                        continue;
                }
                let test = tree_scanner(family_tree.as_mut_slice(), split[1].to_string());
                if test == 1 {
                        family_tree[1] = "content_checker".to_string();
                        family_tree[3] = "content_checker".to_string();
                        family_tree[4] = "content_checker".to_string();
                }else if test == 2 {
                        family_tree[2] = "content_checker".to_string();
                        family_tree[5] = "content_checker".to_string();
                        family_tree[6] = "content_checker".to_string();
                } else if test == 3 {
                        family_tree[3] = "content_checker".to_string();
                }else if test == 4 {
                        family_tree[4] = "content_checker".to_string();
                }
                else if test == 5 {
                        family_tree[5] = "content_checker".to_string();
                }
                else if test == 6 {
                        family_tree[6] = "content_checker".to_string();
                } else {
                    println!("Name not found");
                }

                println!("Delete completed");
        }else if split[0] == "print" {
                println!("{}", family_tree[0]);
                if family_tree[1] != "content_checker" {
                    println!("\t{}", family_tree[1]);
                    if family_tree[3] != "content_checker" {
                        println!("\t\t{}", family_tree[3]);
                    }
                    if family_tree[4] != "content_checker" {
                        println!("\t\t{}",family_tree[4]);
                    }
                }
                if family_tree[2] != "content_checker" {
                    println!("\t{}", family_tree[2]);
                    if family_tree[5] != "content_checker" {
                        println!("\t\t{}", family_tree[5]);
                    }
                    if family_tree[6] != "content_checker" {
                        println!("\t\t{}",family_tree[6]);
                    }
                }
        } else if split[0] == "quit"{
                println!("Good Bye");
        break;
     } else {
            println!("Invalid command");
        }
		
		//Function to check array and see if the name is already inside
        fn contains_node(arr: &mut [String], name: String) -> bool {
                for x in 0..7 {
                    if arr[x] == name {
                        return true;
                    }
                }
                return false;
        }

		//Function to check array and return index
        fn tree_scanner(arr: &mut [String], name: String) -> i32 {
                for x in 0..7 {
                    if arr[x] == name {
                        return (x) as i32;
                    }
                }
                return -1;
        }
}
}
