use git2::Repository;
use inquire::error::InquireError;
use inquire::Select;

fn main() {
    let repo = open_curr_repo();

    let branches = get_branches(&repo);

    let branch = pp_branches(&branches);

    checkout_branch(&repo, branch)
}

fn open_curr_repo() -> Repository {
    match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    }
}

fn get_branches(repo: &Repository) -> Vec<(git2::Branch, git2::BranchType)> {
    match repo.branches(None) {
        Ok(branches) => branches.filter_map(|b| b.ok()).collect(),
        Err(_) => panic!("failed to get branches"),
    }
}

fn pp_branches<'a>(branches: &'a [(git2::Branch, git2::BranchType)]) -> &'a str {
    let items = branches
        .iter()
        .map(|b| b.0.name().unwrap().unwrap())
        .collect::<Vec<_>>();
    let mut select: Select<&str> = Select::new("Git branches:", items);

    select.vim_mode = true;

    let ans: Result<&str, InquireError> = select.prompt();

    match ans {
        Ok(ans) => ans,
        Err(_) => panic!("The shopping list could not be processed"),
    }
}

fn checkout_branch(repo: &Repository, branch_name: &str) {
    let (object, reference) = repo.revparse_ext(branch_name).expect("Object not found");

    repo.checkout_tree(&object, None)
        .expect("Failed to checkout");

    match reference {
        // gref is an actual reference like branches or tags
        Some(gref) => repo.set_head(gref.name().unwrap()),
        // this is a commit, not a reference
        None => repo.set_head_detached(object.id()),
    }
    .expect("Failed to set HEAD");
}
