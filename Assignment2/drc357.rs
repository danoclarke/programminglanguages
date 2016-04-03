use std::io;
use std::io::prelude::*;

////////// SYNTAX CHECK FUNCTIONS //////////

fn is_input(a: &str) -> bool {
  let lines: Vec<&str> = a.split(".\n").collect();

  if lines.len() > 1 {
    if is_data(lines[0].trim()) && is_query(lines[1].trim()) {
      return true;
    }else{
      //println!("{} - is_input", a);
      return false;
    }
  }else{
    if lines[0] == ".\n" {
      return true;
    }else{
      return false;
    }
  }
}

fn is_data(a: &str) -> bool {
  //LIST '\n' DATA | ""
  //println!("isdata? -- {}", a);

  let lines: Vec<&str> = a.split("\n").collect();

  for line in lines {
    //println!("{} - {}", line.trim(), is_list(line.trim()));
    if is_list(line.trim()) || line == "" {
      continue;
    }else{
      //println!("{} - {}", line, is_list(line.trim()));
      return false;
    }
  }

  return true;
}

fn is_list(a: &str) -> bool {
  // "{" NAME ":" ITEMS "}"
  //println!("islist? -- {}", a);

  if a.starts_with('{') && a.ends_with('}') {
    //println!("{} - is_list", a);
    let a = a[1..(a.trim().len() - 1)].to_string();

    let lines: Vec<&str> = a.trim().splitn(2, ':').collect();
    //println!("{:?}", lines);

    if is_name(lines[0].trim()) {
      if is_items(lines[1].trim()){
        //println!("LIST! - {}", a.trim());
        return true;
      }else{ return false; }
    }else{ return false; }
  }else{
    //println!("{} - is_list", a);
    return false;
  }
}

fn is_name(a: &str) -> bool {
  //println!("isname? -- {}", a);
  return is_string(a);
}

fn is_items(a: &str) -> bool {
  //println!("isitems? -- {}", a);
  // ONEITEM | ONEITEM "," ITEMS

  if a.trim()=="" {
    return true;
  }

  if is_oneitem(a.trim()) {
    //println!("is item - {} - {}", a.trim(), is_oneitem(a.trim()));
    return true;
  }else{
    let lines: Vec<&str> = a.splitn(2, ',').collect();

    if is_oneitem(lines[0].trim()){
      return is_items(lines[1].trim());
    }else{
      let chars: Vec<char> = a.trim().chars().collect();
      if chars[0] == '{' {
        //println!("{}", lines[1].trim());
        let mut sub_list = String::new();
        let mut start_ind = 0;
        let mut ind = 0;
        let mut count = 0;
        let mut open_count = 0;

        for c in chars {
          if c == '{' {
            count = count + 1;
            open_count = open_count + 1;
            if open_count == 1 {
              start_ind = ind;
            }
          }

          if c == '}' {
            count = count - 1;
          }

          ind = ind + 1;

          if open_count > 0 && count == 0 {
            sub_list = a.trim()[start_ind..ind].to_string();
            break;
          }
        }

        //println!("{}", sub_list);
        //println!("NEXT {}",a.trim()[ind+1..(a.len())].trim_left_matches(',').to_string());
        if is_list(&sub_list) {
          if (a.len() as i32) < (ind+1) as i32{
            return true;
          }else{
            return is_items(&a.trim()[ind+1..(a.len())].trim_left_matches(',').to_string());
          }
        }else{
          return false;
        }
      }else{
        return false;
      }
    } 
  }
}

fn is_oneitem(a: &str) -> bool {
  // NUMBER | STRING | PTR | LIST
  //println!("isoneitem? -- {}", a);

  if is_number(a) || is_string(a) || is_ptr(a) || is_list(a){
    //println!("{} - {} - {} - {} - {}", a, is_number(a), is_string(a), is_ptr(a), is_list(a));
    return true;
  }else{
    //println!("{} - is_oneitem", a);
    return false;
  }
}

fn is_number(a: &str) -> bool {
  let a_num: Option<i32> = a.trim().parse().ok();

  match a_num {
    None => return false,//println!("{} - is_number", a),
    _ => return true,
  };
}

fn is_string(a: &str) -> bool {
  let chars: Vec<char> = a.trim().chars().collect();

  if chars[0].is_alphabetic() {
    for c in chars {
      if !c.is_alphanumeric() {
        //println!("{} - is_string", a);
        return false;
      }
    }    
  }else{
    //println!("{} - is_string", a);
    return false;
  }

  return true;
}

fn is_ptr(a: &str) -> bool {
  if a.starts_with('@') {
    return is_string(&a[1..(a.len())].to_string());
  }else{
    //println!("{} - is_ptr", a);
    return false;
  }
}

fn is_query(a: &str) -> bool {
  // ONEQ "\n" QUERY | ""

  let lines: Vec<&str> = a.trim().split("\n").collect();

  for line in lines {
    if line == "" || is_oneq(line) {
      continue;
    }else{
      //println!("{} - is_query", a);
      return false;
    }
  }

  return true;
}

fn is_oneq(a: &str) -> bool {
	match a.trim() {
		"SUM" => return true,
		"sum" => return true,
		"NAMECHECK" => return true,
		"namecheck" => return true,
		"PTRS" => return true,
		"ptrs" => return true,
    _ => {/* DEFAULT */}
	}

  if a.contains("SEARCH") || a.contains("search") {
    let a = &a[6..(a.len())].to_string();

    if is_string(a.trim()) || is_number(a.trim()) {
      return true;
    }else{ return false; }
  }else{ return false; }
}

////////// QUERY PROCESSING FUNCTIONS //////////


fn process_list<'a>(list: &'a str, names: &mut Vec<&'a str>, name_dupes: &mut Vec<&'a str>, items: &mut Vec<&'a str>, search_term: &'a str){
  //println!("{}", data_part_in.list);

  let this_list = &list.trim()[1..(list.len()-1)];
  let name_split: Vec<&str> = this_list.trim().splitn(2, ':').collect();
  
  //println!("This List --- {:?}", this_list);
  //println!("Name Split --- {:?}", name_split);
  
  if names.contains(&name_split[0].trim()) {
    name_dupes.push(name_split[0].trim());
    names.push(name_split[0].trim());
  }else{
    names.push(name_split[0].trim());
  }

  
  let this_items: &str = name_split[1];

  let mut last_item_split: &str = this_items.trim();
  let mut item_split: Vec<&str> = this_items.trim().splitn(2, ',').collect();

  if item_split.len() == 1 {
    let first_item = item_split[0];

    if is_string(first_item.trim()) || is_number(first_item.trim()) || is_ptr(first_item.trim()) {
      items.push(first_item.trim());
    }else if is_list(first_item.trim()) {
      let new_list = first_item.trim();

      process_list(new_list, names, name_dupes, items, search_term);
    }
  }
  
  while item_split.len() > 1 && !items.contains(&search_term){
    //println!("Item Split --- {:?} // last_item_split --- {:?}", item_split, last_item_split);
    let first_item = item_split[0];
    let mut the_rest = item_split[1];

    if is_string(first_item.trim()) || is_number(first_item.trim()) || is_ptr(first_item.trim()) {
      items.push(first_item.trim());
      if first_item == search_term {
        break;
      }
    }else if is_list(first_item.trim()) {
      let new_list = first_item.trim();
      process_list(new_list, names, name_dupes, items, search_term);
    }else{
      let mut ind = 0;
      let mut start_ind = 0;
      let mut count = 0;
      let mut open_count = 0;

      let chars: Vec<char> = last_item_split.trim().chars().collect();

      for c in chars {
        if c == '{' {
          count = count + 1;
          open_count = open_count + 1;
          if open_count == 1 {
            start_ind = ind;
          }
        }

        if c == '}' {
          count = count - 1;
        }

        ind = ind + 1;

        if open_count > 0 && count == 0 {
          let sub_list = &last_item_split.trim()[start_ind..ind];

          //println!("sub-list - {}", sub_list);
          //println!("before recursion sl : {}", sliced);
          //println!("before recursion : {:?}", sub_data_part.list);

          let new_list = sub_list;
          process_list(new_list, names, name_dupes, items, search_term);
          break;
        }
      }

      if (last_item_split.len() as i32) > (ind+1) as i32{
        the_rest = &last_item_split[ind+1..(last_item_split.len())].trim_left_matches(',');
        //println!("NEXT - {}", the_rest);
      } else {
        break;
      }
    }
    //println!("NEXT OUTSIDE - {}", the_rest);
    if is_string(the_rest.trim()) || is_number(the_rest.trim()) || is_ptr(the_rest.trim()){
      //println!("yes - {}", item_split[1]);
      items.push(the_rest.trim());
      break; //at end of lists
    } if is_list(the_rest.trim()) {
      process_list(the_rest.trim(), names, name_dupes, items, search_term);
      break; //at the end of lists
    } else {
      if item_split.len() > 1 {
      //println!("{}", item_split.len());
        last_item_split = the_rest.trim();
        item_split = the_rest.trim().splitn(2, ',').collect();
      }else{
        break; // we're done
      }
    }
  }
}

fn process_search(items: &mut Vec<&str>, search: &str, data_split: &mut Vec<&str>) {
  //For each list, search for term. Ouput keys its contained in.
  let mut revoutput = String::new();
  if items.contains(&search){
    for data in data_split.iter() {
      //println!("{:?}",data);
      if data.contains(&search.trim()) {
        let mut search_names: Vec<&str> = Vec::new();
        let mut search_name_dupes: Vec<&str> = Vec::new();
        let mut search_items: Vec<&str> = Vec::new();
        process_list(&data, &mut search_names, &mut search_name_dupes, &mut search_items, search);

        if search_names.len() > 0{
          let mut output = String::new();

          for s in search_names.iter(){
            output.push_str(s);
            output.push(':')
          }

          output = output[..(output.len()-1)].to_string();

          let chars: Vec<char> = output.chars().collect();
        
          for c in chars.iter().rev(){
            let add: &str = &c.to_string();
            revoutput.push_str(add);
          }
          revoutput.push(',');
        }
      }
    }
    revoutput = revoutput[..(revoutput.len()-1)].to_string();
    println!("{}", revoutput);
  }else{
    println!("NIL");
  }
}

fn process_sum(items: &mut Vec<&str>) {
  //SUM ALL ITEM NUMBERS

  let mut sum: i32 = 0;

  for e in items.iter() {
    if is_number(e) {
      let num: i32 = e.parse().unwrap();
      sum = sum + num;
    }
  }
  println!("{}", sum);
  
}

fn process_namecheck(name_dupes: &mut Vec<&str>) {
  //Check name fields. If all are unique, print OK. Else, print each conflicting name in alphabetical order
  //println!("{:?}", full_data.name_dupes);
  if name_dupes.len() != 0 {
    let mut s = "".to_string();

    name_dupes.sort();
    name_dupes.dedup();

    for e in name_dupes.iter() {
      if s == "" {
        s = s+e;
      }else{
        s = s + "," + e;
      }
    }

    println!("{}", s);
  }else{
    println!("OK");
  }
}

fn process_ptrs(names: &mut Vec<&str>, items: &mut Vec<&str>) {
  //Print OK if no pointers, or if all pointers point to valid list.
  //Else, prints list of dangling pointers (in alphabetical order)
  if names.len() != 0{
    let mut ptrs: Vec<&str> = Vec::new();
    let mut s = "".to_string();

    for e in items.iter() {
      if e.starts_with('@') {
        ptrs.push(e.trim_left_matches('@'));
      }
    }

    if ptrs.len() > 0 {
      ptrs.sort();
      ptrs.dedup();

      for ptr in ptrs.iter() {
        if names.contains(ptr) {
          continue;
        }else{
          if s == "" {
            s = s+ptr;
          }else{
            s = s + "," + ptr;
          }     
        }
      }

      if s == "" {
        println!("OK");
      }else{
        println!("{}", s);
      }
    }else{
      println!("OK");
    }
  }else{
    println!("OK");
  }
}

////////// MAIN FUNCTION //////////

fn main() {
  let mut input = String::new();

  let stdin = io::stdin();
  for line in stdin.lock().lines() {
      input.push_str(&line.unwrap());
      input.push_str("\n");
  }

  //Check for 'QUIT'
  let lines: Vec<&str> = input.trim().split("\n").collect();

  //println!("{:?}", lines);

  if lines[lines.len()-1].trim() == "QUIT" {
    //println!("{} - {}", input, input.len());
    let input = &input[..(input.len() - 5)];

    if is_input(&input) {
      //Process Queries
      let split: Vec<&str> = input.split(".\n").collect();

      let data_full = split[0].trim();
      let queries: Vec<&str> = split[1].trim().split("\n").collect();

      
      let mut data_split: Vec<&str> = data_full.split("\n").collect();
      //println!("{:?}", data_split);

      let mut names: Vec<&str> = Vec::new();
      let mut name_dupes: Vec<&str> = Vec::new();
      let mut items: Vec<&str> = Vec::new();

      for data in data_split.iter() {
        if data != &"" {
          process_list(&data, &mut names, &mut name_dupes, &mut items, &"");
        }
      }

      //println!("{:?}", names);
      //println!("{:?}", name_dupes);
      //println!("{:?}", items);
      
      for query in queries {
        //println!("{}",query);
        if query.trim().starts_with("search") || query.trim().starts_with("SEARCH") {
          process_search(&mut items, &query[6..].trim(), &mut data_split);
          //println!("SEARCH");
        }else{
          match query.trim() {
            "sum" | "SUM" => process_sum(&mut items),
            "namecheck" | "NAMECHECK" => process_namecheck(&mut name_dupes),
            "ptrs" | "PTRS" => process_ptrs(&mut names, &mut items),
            _ => { }
          }
        }
      }
      
    }else{
      println!("ERR");
    }   
  }else{
    println!("ERR");
  }
}