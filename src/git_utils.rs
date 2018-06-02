use std::fmt;

use git2::{DescribeOptions, Object, Repository};

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
                    write!(f, "[detached@{}]", tag)
                } else {
                    write!(f, "[detached@{}]", oid)
                }
            },
            Branch(name) =>  write!(f, "[{}]", name)
        }
    }
}

pub fn get_head_reference(repo: &Repository) -> Result<Head> {
    let head = repo.head()?;
    let is_detached_head = repo.head_detached()?;
    if is_detached_head {
        let object = head.peel_to_commit()?.into_object();
        Ok(Detached(get_commit_short_id(&object), get_tag(&object)))
    } else {
        Ok(Branch(head.shorthand().unwrap_or("").to_string()))
    }
}

fn get_tag(object: &Object) -> Option<String> {
    let mut describe_options = DescribeOptions::new();
    let describe_tags = describe_options.describe_tags().max_candidates_tags(0);
    let tag = object.describe(describe_tags).ok().and_then(|d| d.format(None).ok());
    tag
}

fn get_commit_short_id(object: &Object) -> String {
    let short_id = object.short_id().unwrap();
    let commit_id = short_id.as_str().unwrap();
    commit_id.to_string()
}
