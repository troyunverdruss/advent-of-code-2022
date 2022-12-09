use std::collections::{HashMap};
use crate::day07::DiskItemType::{DIR, FILE};
use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let lines = read_chunks("day07.txt", "\n");
  solve_one(&lines)
}

pub fn part_two() -> u64 {
  let lines = read_chunks("day07.txt", "\n");
  solve_two(&lines)
}

fn solve_one(lines: &Vec<String>) -> u64 {
  let disk_items = build_file_system_tree(lines);
  let computed_sizes = compute_dir_sizes(&disk_items, 0, &HashMap::new());
  computed_sizes
    .iter()
    .filter(|(_, v)| **v <= 100000)
    .map(|(_, v)| v)
    .sum()
}

fn solve_two(lines: &Vec<String>) -> u64 {
  let disk_items = build_file_system_tree(lines);
  let computed_sizes = compute_dir_sizes(&disk_items, 0, &HashMap::new());
  let total_disk_size = 70000000;
  let total_required_disk_size = 30000000;

  let currently_unused_space = total_disk_size - computed_sizes.get(&0).unwrap();
  let additional_required_space = total_required_disk_size - currently_unused_space;

  let mut possible_sizes: Vec<u64> = computed_sizes
    .iter()
    .filter(|(_, v)| **v >= additional_required_space)
    .map(|(_, v)| v.to_owned())
    .collect();

  possible_sizes.sort();

  possible_sizes.get(0).unwrap().to_owned()
}


fn build_file_system_tree(lines: &Vec<String>) -> Vec<DiskItem> {
  let mut location = 0;
  let mut items: Vec<DiskItem> = Vec::new();
  let mut items_by_path: HashMap<String, usize> = HashMap::new();
  let mut next_id = 0;

  // Setup root
  items.push(DiskItem {
    _name: "/".to_owned(),
    full_path: "/".to_owned(),
    item_type: DIR,
    size: 0,
    parent_id: 0,
    id: next_id,
  });
  items_by_path.insert("/".to_owned(), 0);
  next_id += 1;

  lines.iter().for_each(|line| {
    let curr_path = items
      .get(location)
      .expect("Our current path better be in there!")
      .full_path.to_owned();

    if line.starts_with("$") {
      if line == "$ cd /" || line == "$ ls" {
        // noop since we either:
        // - setup root already, or
        // - this line is an 'ls' which means there's nothing to do right now
      } else if line.starts_with("$ cd") {
        let parts: Vec<&str> = line.split(" ").collect();
        assert_eq!(parts.len(), 3, "cd command should have 3 parts");

        let destination_dir = parts.get(2).unwrap().to_owned();

        if destination_dir == ".." {
          location = items.get(location).unwrap().parent_id.to_owned();
        } else {
          let target_path = if location == 0 {
            "/".to_owned() + destination_dir
          } else {
            curr_path + "/" + destination_dir
          };
          let cd_target_id = items_by_path
            .get(&target_path)
            .expect("Expecting that we've seen all the items in an 'ls' before trying to 'cd' but maybe that's a bad assumption");

          location = *cd_target_id;
        }
      }
    } else {
      // The only line that don't start with $ are the outputs of 'ls', so we just need
      // to process through them and add items as appropriate for each one
      let parts: Vec<&str> = line.split(" ").collect();
      if parts.get(0).unwrap() == &"dir" {
        let name = parts.get(1).unwrap().to_string();
        let full_path = if location == 0 {
          ("/".to_string() + &name).to_owned()
        } else {
          (curr_path + "/" + &name).to_owned()
        };
        let disk_item = DiskItem {
          _name: name,
          full_path: full_path.to_owned(),
          item_type: DIR,
          size: 0,
          parent_id: location,
          id: next_id,
        };
        items.push(disk_item);
        items_by_path.insert(full_path.to_owned(), next_id);
        next_id += 1;
      } else {
        let size = parts.get(0).unwrap().parse::<u64>().expect("Should have been a number");
        let name = parts.get(1).unwrap().to_string();
        let full_path = if location == 0 {
          ("/".to_string() + &name).to_owned()
        } else {
          (curr_path + "/" + &name).to_owned()
        };

        let disk_item = DiskItem {
          _name: name,
          full_path: full_path.to_owned(),
          item_type: FILE,
          size: size,
          parent_id: location,
          id: next_id,
        };
        items.push(disk_item);
        items_by_path.insert(full_path.to_owned(), next_id);
        next_id += 1;
      }
    }
  });
  items
}

fn compute_dir_sizes(disk_items: &Vec<DiskItem>, location: usize, known_sizes: &HashMap<usize, u64>) -> HashMap<usize, u64> {
  // println!("Starting computation for location {}", location);
  let mut new_known_sizes = known_sizes.clone();
  let to_traverse: Vec<&DiskItem> = disk_items.iter().filter(|di| di.parent_id == location && di.id != 0).collect();
  let to_traverse_ids: Vec<usize> = to_traverse.iter().map(|tt| tt.id).collect();
  let mut dir_size = 0 as u64;
  for disk_item in to_traverse {
    dir_size += disk_item.size;
    if disk_item.item_type == DIR {
      let result = compute_dir_sizes(disk_items, disk_item.id, known_sizes);
      for (id, size) in result {
        // let curr_val = new_known_sizes.entry(id).or_insert_with(|| 0).to_owned();
        new_known_sizes.insert(id, size);
        // only add this to THIS directory's size if it is a direct descendant
        if to_traverse_ids.contains(&id) {
          dir_size += size;
        }
        // println!("Dir {}, adding {}, new total: {}", location, size, dir_size);
      };
    };
  }
  // println!("Inserting size {} for dir {}", dir_size, location);
  new_known_sizes.insert(location, dir_size);

  new_known_sizes
}

#[derive(PartialOrd, PartialEq, Debug)]
enum DiskItemType {
  DIR,
  FILE,
}

struct DiskItem {
  _name: String,
  full_path: String,
  item_type: DiskItemType,
  size: u64,
  parent_id: usize,
  id: usize,
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use crate::day07::{build_file_system_tree, compute_dir_sizes, solve_one, solve_two};
  use crate::day07::DiskItemType::{DIR, FILE};

  #[test]
  fn test_building_the_tree() {
    let inputs = get_inputs();
    let disk_items = build_file_system_tree(&inputs);

    assert_eq!(disk_items.get(1).unwrap()._name, "a");
    assert_eq!(disk_items.get(1).unwrap().full_path, "/a");
    assert_eq!(disk_items.get(1).unwrap().size, 0);
    assert_eq!(disk_items.get(1).unwrap().item_type, DIR);
    assert_eq!(disk_items.get(1).unwrap().parent_id, 0);
    assert_eq!(disk_items.get(1).unwrap().id, 1);

    assert_eq!(disk_items.get(5).unwrap()._name, "e");
    assert_eq!(disk_items.get(5).unwrap().full_path, "/a/e");
    assert_eq!(disk_items.get(5).unwrap().size, 0);
    assert_eq!(disk_items.get(5).unwrap().item_type, DIR);
    assert_eq!(disk_items.get(5).unwrap().parent_id, 1);
    assert_eq!(disk_items.get(5).unwrap().id, 5);

    assert_eq!(disk_items.get(9).unwrap()._name, "i");
    assert_eq!(disk_items.get(9).unwrap().full_path, "/a/e/i");
    assert_eq!(disk_items.get(9).unwrap().size, 584);
    assert_eq!(disk_items.get(9).unwrap().item_type, FILE);
    assert_eq!(disk_items.get(9).unwrap().parent_id, 5);
    assert_eq!(disk_items.get(9).unwrap().id, 9);

    assert_eq!(disk_items.get(13).unwrap()._name, "k");
    assert_eq!(disk_items.get(13).unwrap().full_path, "/d/k");
    assert_eq!(disk_items.get(13).unwrap().size, 7214296);
    assert_eq!(disk_items.get(13).unwrap().item_type, FILE);
    assert_eq!(disk_items.get(13).unwrap().parent_id, 4);
    assert_eq!(disk_items.get(13).unwrap().id, 13);
  }

  #[test]
  fn test_traversing_the_tree() {
    let inputs = get_inputs();
    let disk_items = build_file_system_tree(&inputs);
    let computed_sizes = compute_dir_sizes(&disk_items, 0, &HashMap::new());

    assert_eq!(*computed_sizes.get(&5).unwrap(), 584);
    assert_eq!(*computed_sizes.get(&1).unwrap(), 94853);
    assert_eq!(*computed_sizes.get(&4).unwrap(), 24933642);
    assert_eq!(*computed_sizes.get(&0).unwrap(), 48381165);
  }

  #[test]
  fn test_solving_part_1() {
    let inputs = get_inputs();
    let result = solve_one(&inputs);

    assert_eq!(result, 95437);
  }

  #[test]
  fn test_solving_part_2() {
    let inputs = get_inputs();
    let result = solve_two(&inputs);

    assert_eq!(result, 24933642);
  }


  fn get_inputs() -> Vec<String> {
    // 0 - / (dir)
    // 1   - a (dir)
    // 5     - e (dir)
    // 9       - i (file, size=584)
    // 6     - f (file, size=29116)
    // 7     - g (file, size=2557)
    // 8     - h.lst (file, size=62596)
    // 2   - b.txt (file, size=14848514)
    // 3   - c.dat (file, size=8504156)
    // 4   - d (dir)
    // 10    - j (file, size=4060174)
    // 11    - d.log (file, size=8033020)
    // 12    - d.ext (file, size=5626152)
    // 13    - k (file, size=7214296)

    let inputs: Vec<String> = vec![
      "$ cd /", // 0
      "$ ls",
      "dir a", // 1
      "14848514 b.txt", // 2
      "8504156 c.dat", // 3
      "dir d", // 4
      "$ cd a",
      "$ ls",
      "dir e", // 5
      "29116 f", // 6
      "2557 g", // 7
      "62596 h.lst", // 8
      "$ cd e",
      "$ ls",
      "584 i", // 9
      "$ cd ..",
      "$ cd ..",
      "$ cd d",
      "$ ls",
      "4060174 j", // 10
      "8033020 d.log", // 11
      "5626152 d.ext", // 12
      "7214296 k", // 13
    ]
      .iter()
      .map(|l| l.to_string())
      .collect();
    inputs
  }
}