use crate::Config;
use anyhow::Result;
use gitlab::{
    api::{self, common::AccessLevel, projects, AsyncQuery, Pagination},
    types::{Project, ProjectVariable},
    GitlabBuilder, ProjectId,
};
use tokio::task::JoinSet;
use tokio_stream::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BoundVariable {
    pub project_id: ProjectId,
    pub variable: ProjectVariable,
}

pub async fn fetch_variables(config: &Config) -> Result<(Vec<Project>,Vec<BoundVariable>)> {
    let client = GitlabBuilder::new(&config.host, &config.token)
        .build_async()
        .await?;
    let projects = projects::Projects::builder()
        .min_access_level(AccessLevel::Maintainer)
        .archived(false)
        .build()?;
    let projects = api::paged(projects, Pagination::All);
    let projects = projects.iter_async(&client);
    tokio::pin!(projects);
    let mut set: JoinSet<Result<_>> = JoinSet::new();
    while let Some(project) = projects.next().await {
        let project: Project = project?;
        let client = client.clone();
        set.spawn(async move {
            let project_vars: Vec<ProjectVariable> =
                projects::variables::ProjectVariable::builder()
                    .project(project.id.value())
                    .key("")
                    .build()?
                    .query_async(&client)
                    .await?;
            let project_vars: Vec<_> = project_vars
                .into_iter()
                .map(|variable| BoundVariable {
                    project_id: project.id,
                    variable,
                })
                .collect();
            Ok((project, project_vars))
        });
    }
    let mut variables = Vec::new();
    let mut projects = Vec::new();
    while let Some(result) = set.join_next().await {
        let (project, mut project_vars) = result??;
        variables.append(&mut project_vars);
        projects.push(project);
    }
    Ok((projects, variables))
}
