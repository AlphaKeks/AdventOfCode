use {
	advent_of_code::AdventOfCode,
	anyhow::{anyhow, Context, Result},
	std::{cell::RefCell, collections::HashMap, rc::Rc},
};

#[derive(Default)]
pub struct AoC {
	root: Rc<Directory>,
}

#[derive(Default)]
struct Directory {
	size: RefCell<usize>,
	parent: Option<Rc<Directory>>,
	children: RefCell<HashMap<String, Rc<Directory>>>,
}

impl Directory {
	fn compute_size(&self) -> usize {
		*self.size.borrow()
			+ self
				.children
				.borrow()
				.values()
				.fold(0, |acc, directory| acc + directory.compute_size())
	}
}

impl AoC {
	pub fn new(input: &'static str) -> Result<Self> {
		let root = Rc::new(Directory::default());

		let mut cwd = Rc::clone(&root);

		for (left, right) in input
			.lines()
			.filter_map(|line| line.split_once(' '))
		{
			match (left, right) {
				("$", "ls") => {}
				("$", "cd /") => {
					cwd = Rc::clone(&root);
				}
				("$", "cd ..") => {
					let new_cwd = match cwd.parent.as_ref() {
						Some(parent) => Rc::clone(parent),
						None => Rc::clone(&root),
					};

					cwd = new_cwd;
				}
				("$", cd) => {
					let (cd, dirname) = cd
						.split_once(' ')
						.ok_or(anyhow!("Expected `cd dirname`."))?;

					// Sanity check
					assert_eq!(cd, "cd");

					let new_cwd = Rc::clone(
						cwd.children
							.borrow()
							.get(dirname)
							.ok_or(anyhow!(
								"We assume that the \"user\" will never enter a directory they have not \
								 previously discovered through `$ ls`."
							))?,
					);

					cwd = new_cwd;
				}
				("dir", dirname) => {
					cwd.children.borrow_mut().insert(
						String::from(dirname),
						Rc::new(Directory {
							parent: Some(Rc::clone(&cwd)),
							..Default::default()
						}),
					);
				}
				(size, _) => {
					let size = size
						.parse::<usize>()
						.context("Expected filesize.")?;

					*cwd.size.borrow_mut() += size;
				}
			}
		}

		Ok(Self { root })
	}
}

impl AdventOfCode for AoC {
	type SolutionA = usize;
	type SolutionB = usize;

	const PROBLEM: &'static str = include_str!("./problem.md");
	const DAY: usize = 7;

	fn part_a(&mut self) -> Result<Self::SolutionA> {
		let mut total_size = 0;

		let mut to_visit = vec![Rc::clone(&self.root)];
		while let Some(directory) = to_visit.pop() {
			for subdirectory in directory.children.borrow().values() {
				to_visit.push(Rc::clone(subdirectory));
			}

			let size = directory.compute_size();
			if size <= 100_000 {
				total_size += size;
			}
		}

		Ok(total_size)
	}

	fn part_b(&mut self) -> Result<Self::SolutionB> {
		let used_space = self.root.compute_size();
		let free_space = 70_000_000 - used_space;
		let required_space = 30_000_000 - free_space;

		let mut smallest_required = usize::MAX;

		let mut to_visit = vec![Rc::clone(&self.root)];
		while let Some(directory) = to_visit.pop() {
			for subdirectory in directory.children.borrow().values() {
				to_visit.push(Rc::clone(subdirectory));
			}

			let size = directory.compute_size();
			if size >= required_space {
				smallest_required = smallest_required.min(size);
			}
		}

		Ok(smallest_required)
	}
}
