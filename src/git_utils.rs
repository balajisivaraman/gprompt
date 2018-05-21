use std::fmt;

use git2::{DescribeOptions, Repository};

use Result;

pub enum Head {
    Detached(String, Option<String>),
    Branch(String)
}

use self::Head::*;

impl fmt::Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Detached(oid, maybe_tag) => {
                if let Some(tag) = maybe_tag {
                    write!(f, "detached@{}", tag)
                } else {
                    write!(f, "detached@{}", oid)
                }
            },
            Branch(name) =>  write!(f, "{}", name)
        }
    }
}

pub fn get_head_reference(repo: &Repository) -> Result<Head> {
    let head = repo.head()?;
    let is_detached_head = repo.head_detached()?;
    if is_detached_head {
        let commit = head.peel_to_commit()?;
        let object = commit.as_object();
        let short_id = object.short_id()?;
        let commit_id = short_id.as_str().unwrap();
        let mut describe_options = DescribeOptions::new();
        let describe_tags = describe_options.describe_tags().max_candidates_tags(0);
        let tag = object.describe(describe_tags).ok().and_then(|d| d.format(None).ok());
        Ok(Detached(commit_id.to_string(), tag))
    } else {
        Ok(Branch(head.shorthand().unwrap().to_string()))
    }
}

